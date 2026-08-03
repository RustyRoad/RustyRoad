//! Table-level constraint rendering for `schema.ts`.

mod foreign;
mod imports;
mod keys;

use super::casing::{identifier, Casing};
use crate::database::introspection::Table;

pub(super) use imports::imports;

/// Renders the table-level constraint array, or an empty string when none apply.
///
/// Drizzle passes these as a second argument to `pgTable`, since they span more
/// than one column and cannot be expressed on a single builder.
pub(super) fn render(table: &Table, names: &[String], casing: Casing) -> String {
    let mut entries: Vec<String> = Vec::new();

    entries.extend(keys::indexes(table, casing));
    entries.extend(foreign::render(table, names, casing));
    entries.extend(keys::composite_key(table, casing));
    entries.extend(keys::uniques(table, casing));

    if entries.is_empty() {
        return String::new();
    }

    format!(
        ", (table) => [\n{},\n]",
        entries
            .iter()
            .map(|entry| format!("\t{entry}"))
            .collect::<Vec<_>>()
            .join(",\n")
    )
}

/// Renders a `table.column` list.
fn columns(names: &[String], casing: Casing) -> String {
    names
        .iter()
        .map(|name| format!("table.{}", identifier(name, casing)))
        .collect::<Vec<_>>()
        .join(", ")
}
