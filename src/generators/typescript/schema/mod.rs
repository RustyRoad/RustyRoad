//! `schema.ts` generation from an introspected schema.

mod imports;

use super::casing::{binding, Casing};
use super::{columns, constraints, enums, ordering, types};
use crate::database::introspection::{Schema, Table};

/// Renders the complete `schema.ts` file.
pub fn render(schema: &Schema, casing: Casing) -> String {
    let tables = ordering::ordered(schema);
    let names: Vec<String> = tables.iter().map(|table| table.name.clone()).collect();

    let mut file = imports::render(schema, &tables);
    file.push('\n');

    // Enums are declared before the tables that reference them, since a `const` is
    // not hoisted.
    file.push_str(&enums::render(schema, casing));

    for table in &tables {
        file.push_str(&declaration(schema, table, &names, casing));
        file.push_str("\n\n");
    }

    file
}

/// Renders one `export const <name> = pgTable(...)` declaration.
fn declaration(
    schema: &Schema,
    table: &Table,
    names: &[String],
    casing: Casing,
) -> String {
    let body = table
        .columns
        .iter()
        .map(|column| columns::render(schema, table, column, casing))
        .collect::<Vec<_>>()
        .join(",\n");

    format!(
        "export const {} = pgTable(\"{}\", {{\n{body},\n}}{});",
        binding(&table.name, casing),
        table.name,
        constraints::render(table, names, casing)
    )
}
