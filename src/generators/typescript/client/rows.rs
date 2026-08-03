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
pub(in crate::generators::typescript) fn key_type(table: &Table, key: &str) -> &'static str {
    let Some(column) = table.column(key) else {
        return "string";
    };
    let lowered = column.sql_type.to_lowercase();

    let numeric = ["int", "smallint", "bigint", "serial", "real", "double"]
        .iter()
        .any(|prefix| lowered.starts_with(prefix));

    if numeric {
        "number"
    } else {
        "string"
    }
}
