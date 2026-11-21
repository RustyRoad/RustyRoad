//! Read storage contracts in the owning catalog snapshot, never user-table data.
use super::super::{
    connection,
    types::{Catalog, Column},
};
use sqlx::{Postgres, Row, Transaction};

/// Attach observed columns without treating an empty table as absent.
pub(super) async fn read(
    tx: &mut Transaction<'_, Postgres>,
    catalog: &mut Catalog,
) -> Result<(), String> {
    let rows = sqlx::query(
        "SELECT c.relname AS table_name, a.attname AS name,
         format_type(a.atttypid,a.atttypmod) AS sql_type, NOT a.attnotnull AS nullable
         FROM pg_attribute a JOIN pg_class c ON c.oid=a.attrelid
         JOIN pg_namespace n ON n.oid=c.relnamespace
         WHERE n.nspname=$1 AND c.relkind IN ('r','p','v','m','f')
         AND a.attnum>0 AND NOT a.attisdropped ORDER BY c.relname,a.attnum",
    )
    .bind(&catalog.schema)
    .fetch_all(&mut **tx)
    .await
    .map_err(connection::diagnostic)?;
    for row in rows {
        let name: String = row.try_get("table_name").map_err(connection::diagnostic)?;
        if let Some(table) = catalog.tables.iter_mut().find(|t| t.name == name) {
            table.columns.push(Column {
                name: row.try_get("name").map_err(connection::diagnostic)?,
                sql_type: row.try_get("sql_type").map_err(connection::diagnostic)?,
                nullable: row.try_get("nullable").map_err(connection::diagnostic)?,
            });
        }
    }
    Ok(())
}
