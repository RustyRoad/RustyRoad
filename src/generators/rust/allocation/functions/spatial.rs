//! Spatial operation identities participate in the shared collision allocator.
use super::{key, Key};
use crate::database::introspection::Table;
use crate::generators::rust::{naming::snake, repositories::spatial};
use std::collections::BTreeMap;

/// Reserves containment and declared-point projection names for this relation.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn append(table: &Table, stem: &str, candidates: &mut BTreeMap<Key, String>) {
    for column in spatial::columns(table) {
        candidates.insert(
            key(
                table,
                "contains_point",
                std::slice::from_ref(&column.name),
                None,
            ),
            format!("contains_{stem}_{}_point", snake(&column.name)),
        );
    }
    for column in spatial::point_columns(table) {
        candidates.insert(
            key(
                table,
                "point_coordinates",
                std::slice::from_ref(&column.name),
                None,
            ),
            format!("coordinates_{stem}_{}", snake(&column.name)),
        );
    }
}
