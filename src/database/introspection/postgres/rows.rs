//! Row readers for introspection queries.

use super::super::assemble;
use super::super::model::Column;
use sqlx::{postgres::PgRow, Row};

/// Reads a text column, defaulting to empty.
pub(super) fn text(row: &PgRow, name: &str) -> String {
    row.try_get::<String, _>(name).unwrap_or_default()
}

/// Converts a column row into `(table, column)`.
pub(super) fn column(row: &PgRow) -> (String, Column) {
    (
        text(row, "table_name"),
        Column {
            name: text(row, "column_name"),
            sql_type: text(row, "sql_type"),
            nullable: row.try_get("nullable").unwrap_or(true),
            default: row
                .try_get::<Option<String>, _>("default_value")
                .ok()
                .flatten(),
            auto_increment: row.try_get("auto_increment").unwrap_or(false),
        },
    )
}

/// Reads a referential action code, which Postgres returns as a single char.
pub(super) fn action(row: &PgRow, name: &str) -> Option<String> {
    let code = row
        .try_get::<i8, _>(name)
        .map(|byte| (byte as u8 as char).to_string())
        .unwrap_or_default();
    assemble::action(&code)
}
