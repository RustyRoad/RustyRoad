//! `relations.ts` generation from foreign keys.
//!
//! Drizzle uses these declarations for relational queries. They are derived from
//! the introspected foreign keys, so they cannot drift from the database.

mod collect;

use super::casing::Casing;
use crate::database::introspection::Schema;

/// Renders the complete `relations.ts` file.
pub fn render(schema: &Schema, casing: Casing) -> String {
    let sides = collect::collect(schema, casing);
    if sides.is_empty() {
        return String::from("// No foreign keys were found, so there are no relations.\n");
    }

    let names = sides.keys().cloned().collect::<Vec<_>>().join(", ");
    let mut file = format!(
        "import {{ relations }} from \"drizzle-orm\";\nimport {{ {names} }} from \"./schema\";\n\n"
    );

    for (table, entries) in &sides {
        file.push_str(&declaration(table, entries));
    }

    file
}

/// Renders one table's relations declaration.
fn declaration(table: &str, entries: &[String]) -> String {
    format!(
        "export const {table}Relations = relations({table}, ({{ {} }}) => ({{\n{}\n}}));\n\n",
        collect::helpers(entries),
        entries
            .iter()
            .map(|entry| format!("\t{entry}"))
            .collect::<Vec<_>>()
            .join(",\n")
    )
}
