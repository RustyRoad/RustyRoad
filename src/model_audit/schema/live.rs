//! A single repeatable-read, read-only catalog snapshot. No user-table data is read.
mod columns;
mod keys;

use super::{
    connection,
    types::{Catalog, Table},
};
use sqlx::{postgres::PgPoolOptions, Row};
use std::{path::Path, time::Duration};

/// Introspect all relation names, even empty tables, so absence is not guessed from columns.
pub(super) async fn read(
    root: &Path,
    schema: &str,
    config: Option<&Path>,
) -> Result<Catalog, String> {
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(Duration::from_secs(15))
        .connect_with(connection::options(root, config)?)
        .await
        .map_err(connection::diagnostic)?;
    let mut tx = pool.begin().await.map_err(connection::diagnostic)?;
    sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ, READ ONLY")
        .execute(&mut *tx)
        .await
        .map_err(connection::diagnostic)?;
    let visible: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM pg_namespace WHERE nspname=$1 AND has_schema_privilege(oid, 'USAGE'))")
        .bind(schema).fetch_one(&mut *tx).await.map_err(connection::diagnostic)?;
    if !visible {
        return Err("Requested PostgreSQL schema is absent or lacks USAGE permission".into());
    }
    let rows = sqlx::query("SELECT c.relname AS name, c.relkind::text AS kind, c.relispartition FROM pg_class c JOIN pg_namespace n ON n.oid=c.relnamespace WHERE n.nspname=$1 ORDER BY c.relname")
        .bind(schema).fetch_all(&mut *tx).await.map_err(connection::diagnostic)?;
    let mut catalog = Catalog {
        version: 1,
        schema: schema.into(),
        complete: true,
        tables: vec![],
        reserved_names: vec![],
    };
    for row in rows {
        let name: String = row.try_get("name").map_err(connection::diagnostic)?;
        let kind: String = row.try_get("kind").map_err(connection::diagnostic)?;
        let partition: bool = row
            .try_get("relispartition")
            .map_err(connection::diagnostic)?;
        catalog.reserved_names.push(name.clone());
        let kind = match kind.as_str() {
            "r" if !partition => "table",
            "r" => "partitioned_table",
            "p" => "partitioned_table",
            "v" => "view",
            "m" => "materialized_view",
            "f" => "foreign_table",
            _ => continue,
        };
        catalog.tables.push(Table {
            name,
            kind: kind.into(),
            columns: vec![],
            primary_key: vec![],
            primary_key_details: None,
        });
    }
    let names: Vec<String> = sqlx::query_scalar("SELECT t.typname FROM pg_type t JOIN pg_namespace n ON n.oid=t.typnamespace WHERE n.nspname=$1")
        .bind(schema).fetch_all(&mut *tx).await.map_err(connection::diagnostic)?;
    catalog.reserved_names.extend(names);
    catalog.reserved_names.sort();
    catalog.reserved_names.dedup();
    columns::read(&mut tx, &mut catalog).await?;
    keys::read(&mut tx, &mut catalog).await?;
    tx.rollback().await.map_err(connection::diagnostic)?;
    pool.close().await;
    Ok(catalog)
}
