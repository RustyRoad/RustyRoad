//! `schema.ts` generation from an introspected schema.

use super::casing::{binding, Casing};
use super::{columns, constraints, ordering, types};
use crate::database::introspection::{Schema, Table};
use std::collections::BTreeSet;

/// Renders the complete `schema.ts` file.
pub fn render(schema: &Schema, casing: Casing) -> String {
    let tables = ordering::ordered(schema);
    let names: Vec<String> = tables.iter().map(|table| table.name.clone()).collect();

    let mut file = imports(schema, &tables);
    file.push('\n');

    for table in &tables {
        file.push_str(&declaration(table, &names, casing));
        file.push_str("\n\n");
    }

    file
}

/// Renders the import header, collecting every symbol the file uses.
fn imports(schema: &Schema, tables: &[&Table]) -> String {
    let mut symbols: BTreeSet<&'static str> = BTreeSet::new();
    symbols.insert("pgTable");

    for table in tables {
        for column in &table.columns {
            symbols.insert(types::map(&column.sql_type, column.auto_increment).import);
        }
        symbols.extend(constraints::imports(table));
    }

    // `customType` is not a real export; bytea has no direct builder.
    symbols.remove("customType");

    let mut header = format!(
        "import {{ {} }} from \"drizzle-orm/pg-core\";\n",
        symbols.into_iter().collect::<Vec<_>>().join(", ")
    );

    if schema.tables.iter().any(columns::needs_sql_import) {
        header.push_str("import { sql } from \"drizzle-orm\";\n");
    }

    header
}

/// Renders one `export const <name> = pgTable(...)` declaration.
fn declaration(table: &Table, names: &[String], casing: Casing) -> String {
    let body = table
        .columns
        .iter()
        .map(|column| columns::render(table, column, casing))
        .collect::<Vec<_>>()
        .join(",\n");

    format!(
        "export const {} = pgTable(\"{}\", {{\n{body},\n}}{});",
        binding(&table.name, casing),
        table.name,
        constraints::render(table, names, casing)
    )
}
