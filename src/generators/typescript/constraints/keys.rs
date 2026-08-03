//! Index, primary key, and unique constraint rendering.

use super::super::casing::Casing;
use crate::database::introspection::Table;

/// Renders index declarations.
pub(super) fn indexes(table: &Table, casing: Casing) -> Vec<String> {
    table
        .indexes
        .iter()
        .map(|index| {
            let builder = if index.unique { "uniqueIndex" } else { "index" };
            format!(
                "{builder}(\"{}\").using(\"btree\", {})",
                index.name,
                super::columns(&index.columns, casing)
            )
        })
        .collect()
}

/// Renders a composite primary key, which cannot be expressed inline.
pub(super) fn composite_key(table: &Table, casing: Casing) -> Vec<String> {
    if table.primary_key.len() < 2 {
        return Vec::new();
    }

    vec![format!(
        "primaryKey({{ columns: [{}], name: \"{}_pkey\" }})",
        super::columns(&table.primary_key, casing),
        table.name
    )]
}

/// Renders unique constraints.
pub(super) fn uniques(table: &Table, casing: Casing) -> Vec<String> {
    table
        .uniques
        .iter()
        .map(|unique| {
            format!(
                "unique(\"{}\").on({})",
                unique.name,
                super::columns(&unique.columns, casing)
            )
        })
        .collect()
}
