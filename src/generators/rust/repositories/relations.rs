//! Collection selector identities come from declared non-unique foreign keys.
use super::targets;
use crate::database::introspection::Table;
use std::collections::BTreeSet;

/// Preserve foreign-key column order, omit unique identities, and deduplicate aliases.
/// No uniqueness or conflict-write permission is inferred from a foreign key.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out ./generated-db
/// ```
pub(in crate::generators::rust) fn keys(table: &Table) -> Vec<Vec<String>> {
    if table.view {
        return Vec::new();
    }
    let mut unique: BTreeSet<_> = targets::keys(table)
        .iter()
        .map(|key| identity(key))
        .collect();
    if !table.primary_key.is_empty() {
        unique.insert(identity(&table.primary_key));
    }
    let mut seen = BTreeSet::new();
    let mut keys = Vec::new();
    for foreign in &table.foreign_keys {
        let columns = &foreign.columns;
        let canonical = identity(columns);
        if columns.is_empty()
            || canonical.len() != columns.len()
            || columns.iter().any(|name| table.column(name).is_none())
            || unique.contains(&canonical)
            || !seen.insert(canonical)
        {
            continue;
        }
        keys.push(columns.clone());
    }
    keys.sort_by_key(|key| identity(key));
    keys
}

/// Compare selector membership without changing the caller's argument ordering.
fn identity(columns: &[String]) -> Vec<String> {
    let mut sorted = columns.to_vec();
    sorted.sort();
    sorted.dedup();
    sorted
}
