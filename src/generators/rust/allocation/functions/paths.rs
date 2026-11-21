use super::{key, Key};
use crate::database::introspection::{Schema, Table};
use crate::generators::rust::naming::snake;
use std::collections::BTreeMap;

/// Reserves full ordered two-edge path identities in the shared function namespace.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn append(
    schema: &Schema,
    table: &Table,
    stem: &str,
    candidates: &mut BTreeMap<Key, String>,
) {
    for first in &table.foreign_keys {
        let Some(related) = schema.tables.iter().find(|t| t.name == first.foreign_table) else {
            continue;
        };
        for second in &related.foreign_keys {
            candidates.insert(
                key(
                    table,
                    "related_path_ordered",
                    &[first.name.clone(), second.name.clone()],
                    None,
                ),
                format!(
                    "related_path_ordered_{stem}_by_{}_and_{}",
                    snake(&first.name),
                    snake(&second.name)
                ),
            );
        }
    }
}
