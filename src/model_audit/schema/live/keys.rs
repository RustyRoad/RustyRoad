//! Constraint identity, ordered columns and inbound dependencies from the same snapshot.
mod sql;
use super::super::{
    connection,
    types::{Catalog, PrimaryKey},
};
use sqlx::{Postgres, Row, Transaction};

/// Keep inbound foreign keys, including dependencies from other schemas.
pub(super) async fn read(
    tx: &mut Transaction<'_, Postgres>,
    catalog: &mut Catalog,
) -> Result<(), String> {
    let rows = sqlx::query(sql::PRIMARY_KEYS)
        .bind(&catalog.schema)
        .fetch_all(&mut **tx)
        .await
        .map_err(|error| {
            format!(
                "Primary-key catalog query: {}",
                connection::diagnostic(error)
            )
        })?;
    for row in rows {
        let name: String = row.try_get("table_name").map_err(connection::diagnostic)?;
        if let Some(table) = catalog.tables.iter_mut().find(|t| t.name == name) {
            table.primary_key = row.try_get("columns").map_err(connection::diagnostic)?;
            table.primary_key_details = Some(PrimaryKey {
                name: row.try_get("name").map_err(connection::diagnostic)?,
                deferrable: row.try_get("deferrable").map_err(connection::diagnostic)?,
                initially_deferred: row
                    .try_get("initially_deferred")
                    .map_err(connection::diagnostic)?,
                referenced_by: row
                    .try_get("referenced_by")
                    .map_err(connection::diagnostic)?,
                standard_index: row
                    .try_get("standard_index")
                    .map_err(connection::diagnostic)?,
            });
        }
    }
    Ok(())
}
