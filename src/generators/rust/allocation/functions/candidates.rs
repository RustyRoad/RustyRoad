//! Collect CRUD, selector and latest-function identities before allocating names.

use super::{key, selectors, Key};
use crate::database::introspection::Schema;
use crate::generators::rust::naming::snake;
use std::collections::BTreeMap;

/// Collect repository names before rendering schema-owned operations.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out ./generated-db
/// ```
pub(super) fn build(schema: &Schema) -> BTreeMap<Key, String> {
    let mut candidates = BTreeMap::new();
    for table in &schema.tables {
        let stem = snake(&table.name);
        for verb in [
            "list",
            "find",
            "get",
            "create",
            "create_many",
            "filter_first",
            "filter_list",
            "filter_count",
            "filter_ordered",
            "filter_ordered_columns",
            "filter_distinct_first",
            "filter_clock",
            "filter_clock_or_null",
            "filter_delete",
            "filter_delete_older_than_days",
            "filter_compare",
            "filter_compare_count",
            "filter_any_first",
            "filter_any_list",
            "filter_any_count",
            "filter_any_update",
            "filter_first_for_update",
            "update",
            "delete",
            "delete_all",
            "configure",
        ] {
            candidates.insert(key(table, verb, &[], None), format!("{verb}_{stem}"));
        }
        candidates.insert(
            key(table, "update_touch", &[], None),
            format!("update_{stem}_touch"),
        );
        candidates.insert(
            key(table, "update_touch_affected", &[], None),
            format!("update_{stem}_touch_affected"),
        );
        candidates.insert(
            key(table, "update_preserving", &[], None),
            format!("update_{stem}_preserving"),
        );
        candidates.insert(
            key(table, "create_clock", &[], None),
            format!("create_{stem}_with_database_time"),
        );
        selectors::append(table, &stem, &mut candidates);
        super::paths::append(schema, table, &stem, &mut candidates);
        for foreign in &table.foreign_keys {
            for verb in [
                "related_first",
                "related_list",
                "related_count",
                "related_left_ordered",
            ] {
                candidates.insert(
                    key(table, verb, std::slice::from_ref(&foreign.name), None),
                    format!("{verb}_{stem}_by_{}", snake(&foreign.name)),
                );
            }
        }
        for column in &table.columns {
            candidates.insert(
                key(table, "latest", std::slice::from_ref(&column.name), None),
                format!("latest_{stem}_by_{}", snake(&column.name)),
            );
        }
    }
    candidates
}
