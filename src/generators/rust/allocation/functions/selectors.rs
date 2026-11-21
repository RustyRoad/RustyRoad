//! Conflict/read/update candidates retain original ordered selector identities.

use super::{key, Key};
use crate::database::introspection::Table;
use crate::generators::rust::{
    naming::snake,
    repositories::{
        atomic::{increment, toggle},
        equality, relations, targets,
    },
};
use std::collections::BTreeMap;

/// Reserves names for keyed operations, including opt-in timestamp-touch patches.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out ./generated-db
/// ```
pub(super) fn append(table: &Table, stem: &str, candidates: &mut BTreeMap<Key, String>) {
    super::spatial::append(table, stem, candidates);
    for column in increment::columns(table) {
        candidates.insert(
            key(table, "increment", std::slice::from_ref(&column.name), None),
            format!("increment_{stem}_{}", snake(&column.name)),
        );
    }
    for column in toggle::columns(table) {
        candidates.insert(
            key(table, "toggle", std::slice::from_ref(&column.name), None),
            format!("toggle_{stem}_{}", snake(&column.name)),
        );
    }
    for columns in targets::keys(table) {
        let suffix = targets::suffix(&columns);
        for verb in [
            "find",
            "delete",
            "update",
            "update_touch",
            "upsert_patch",
            "upsert_patch_touch",
            "upsert_patch_clock",
            "upsert_many_patch_clock",
            "upsert_snapshot",
        ] {
            let base = if verb == "update_touch" {
                format!("update_{stem}_touch_by_{suffix}")
            } else {
                match verb.strip_prefix("upsert_") {
                    Some(mode) => format!("upsert_{stem}_{mode}_by_{suffix}"),
                    None => format!("{verb}_{stem}_by_{suffix}"),
                }
            };
            candidates.insert(key(table, verb, &columns, None), base);
        }
        for column in &table.columns {
            candidates.insert(
                key(table, "upsert_merge", &columns, Some(&column.name)),
                format!("upsert_{stem}_merge_{}_by_{suffix}", snake(&column.name)),
            );
        }
    }
    for columns in equality::keys(table) {
        let suffix = targets::suffix(&columns);
        for verb in ["first", "matching", "ordered"] {
            candidates.insert(
                key(table, verb, &columns, None),
                format!("{verb}_{stem}_by_{suffix}"),
            );
        }
    }
    for columns in relations::keys(table) {
        let suffix = targets::suffix(&columns);
        for verb in ["list", "delete"] {
            candidates.insert(
                key(table, verb, &columns, None),
                format!("{verb}_{stem}_by_{suffix}"),
            );
        }
    }
}
