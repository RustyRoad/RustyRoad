//! Partial conflict identities never participate in ordinary equality targets.
use super::{key, Key};
use crate::database::introspection::{PartialUniqueIndex, Schema};
use crate::generators::rust::naming::snake;
use std::collections::BTreeMap;

/// Adds separately named partial insert-ignore functions to the shared namespace.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn candidates(schema: &Schema, indexes: &[PartialUniqueIndex]) -> BTreeMap<Key, String> {
    let mut candidates = super::candidates::build(schema);
    for index in indexes {
        if let Some(table) = schema.table(&index.table) {
            candidates.insert(
                key(
                    table,
                    "partial_insert",
                    std::slice::from_ref(&index.name),
                    None,
                ),
                format!(
                    "insert_{}_ignore_on_{}",
                    snake(&table.name),
                    snake(&index.name)
                ),
            );
        }
    }
    candidates
}
