//! Row type rendering and primary key typing.

use super::super::casing::{binding, to_pascal, Casing};
use crate::database::introspection::Table;

/// Renders the row types for one table.
///
/// Inferred from the Drizzle table rather than the Zod schema: the repository
/// returns what the driver produced, and the two disagree on some column types.
pub(super) fn row_types(table: &Table, casing: Casing) -> String {
    let name = binding(&table.name, casing);
    let type_name = to_pascal(&table.name);

    format!(
        "export type {type_name}Row = InferSelectModel<typeof {name}>;\n\
         export type New{type_name}Row = InferInsertModel<typeof {name}>;\n"
    )
}

/// Returns the TypeScript type of a table's primary key column.
///
/// Indexing the inferred select row preserves the exact Drizzle type, including
/// PostgreSQL enum unions. Reducing every nonnumeric key to `string` makes the
/// value too wide for Drizzle's `eq(column, value)` overload.
pub(in crate::generators::typescript) fn key_type(
    table: &Table,
    key: &str,
    casing: Casing,
) -> String {
    format!(
        "{}Row[{}]",
        to_pascal(&table.name),
        quoted_property(key, casing)
    )
}

/// Returns whether a key is represented as a JavaScript number.
///
/// The router needs this separately from the repository type so REST path values
/// can be coerced before reaching a numerically typed Drizzle column.
pub(in crate::generators::typescript) fn key_is_numeric(table: &Table, key: &str) -> bool {
    let Some(column) = table.column(key) else {
        return false;
    };
    let lowered = column.sql_type.to_lowercase();

    ["int", "smallint", "bigint", "serial", "real", "double"]
        .iter()
        .any(|prefix| lowered.starts_with(prefix))
}

/// Returns a quoted property key after applying the configured casing.
pub(in crate::generators::typescript) fn quoted_property(key: &str, casing: Casing) -> String {
    let property = super::super::casing::identifier(key, casing);
    if property.starts_with('"') {
        property
    } else {
        format!("\"{property}\"")
    }
}
