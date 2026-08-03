//! Import symbols required by table-level constraints.

use crate::database::introspection::Table;

/// Returns the `drizzle-orm/pg-core` imports a table's constraints require.
///
/// Emitting only what is used keeps the header honest and avoids unused-import
/// warnings in the generated project.
pub(in crate::generators::typescript) fn imports(table: &Table) -> Vec<&'static str> {
    let mut imports = Vec::new();

    if table.indexes.iter().any(|index| index.unique) {
        imports.push("uniqueIndex");
    }
    if table.indexes.iter().any(|index| !index.unique) {
        imports.push("index");
    }
    if !table.foreign_keys.is_empty() {
        imports.push("foreignKey");
    }
    if table.primary_key.len() >= 2 {
        imports.push("primaryKey");
    }
    if !table.uniques.is_empty() {
        imports.push("unique");
    }

    imports
}
