//! Import header for `schema.ts`.

use super::columns;
use super::enums;
use super::types;
use crate::database::introspection::{Schema, Table};
use std::collections::BTreeSet;

/// Renders the import header, collecting every symbol the file uses.
///
/// Emitting only what is used keeps the header honest and avoids unused-import
/// warnings in the generated project.
pub(super) fn render(schema: &Schema, tables: &[&Table]) -> String {
    let mut symbols: BTreeSet<&'static str> = BTreeSet::new();
    symbols.insert("pgTable");

    if !schema.enums.is_empty() {
        symbols.insert("pgEnum");
    }

    for table in tables {
        symbols.extend(column_builders(schema, table));
        symbols.extend(super::constraints::imports(table));
    }

    let mut header = format!(
        "import {{ {} }} from \"drizzle-orm/pg-core\";\n",
        symbols.into_iter().collect::<Vec<_>>().join(", ")
    );

    if schema.tables.iter().any(columns::needs_sql_import) {
        header.push_str("import { sql } from \"drizzle-orm\";\n");
    }

    header
}

/// Returns the builders a table's columns require.
fn column_builders(schema: &Schema, table: &Table) -> Vec<&'static str> {
    table
        .columns
        .iter()
        // An enum column references its declaration, not a column builder.
        .filter(|column| enums::lookup(schema, &column.sql_type).is_none())
        .map(|column| types::map(&column.sql_type, column.auto_increment).import)
        .collect()
}
