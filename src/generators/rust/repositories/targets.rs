//! Conflict targets from the catalog's representable, immediate unique keys.

use super::super::naming::snake;
use crate::database::introspection::Table;
use std::collections::BTreeSet;

/// Deduplicates equivalent keys; e.g. two unique `page_id` indexes yield one target.
/// Index producers must honor the catalog contract: no predicates or expressions.
pub(super) fn keys(table: &Table) -> Vec<Vec<String>> {
    if table.view {
        return Vec::new();
    }
    table.uniques.iter().map(|key| &key.columns)
        .chain(table.indexes.iter().filter(|index| index.unique).map(|index| &index.columns))
        .filter(|key| !key.is_empty() && key.iter().all(|name| table.column(name).is_some()))
        .map(|key| {
            let mut key = key.clone();
            key.sort();
            key.dedup();
            key
        })
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

/// Names a target; e.g. `suffix(&["tenant".into(), "slug".into()])` is `tenant_and_slug`.
pub(super) fn suffix(key: &[String]) -> String {
    key.iter().map(|name| snake(name)).collect::<Vec<_>>().join("_and_")
}
