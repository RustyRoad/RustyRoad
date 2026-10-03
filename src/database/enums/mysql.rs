//! MySQL enum column reader.

use super::model::DatabaseEnum;
use super::parser::parse_mysql_enum_values;
use crate::database::migrations::CustomMigrationError;
use sqlx::Row;

/// MySQL enum columns, since MySQL keeps enum value lists on the columns.
pub(crate) async fn mysql_enums(
    conn: &sqlx::MySqlPool,
) -> Result<Vec<DatabaseEnum>, CustomMigrationError> {
    let rows = sqlx::query(
        "SELECT table_name, column_name, column_type \
         FROM information_schema.columns \
         WHERE table_schema = DATABASE() AND data_type = 'enum' \
         ORDER BY table_name, ordinal_position",
    )
    .fetch_all(conn)
    .await
    .map_err(|e| CustomMigrationError::SendError(Box::new(e)))?;

    Ok(rows
        .into_iter()
        .map(|row| {
            let table: String = row.get("table_name");
            let column: String = row.get("column_name");
            let column_type: String = row.get("column_type");
            DatabaseEnum {
                values: parse_mysql_enum_values(&column_type),
                name: column_type,
                table: Some(table),
                column: Some(column),
            }
        })
        .collect())
}
