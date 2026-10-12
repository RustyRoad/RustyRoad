//! Conflict targets from the catalog's representable, immediate unique keys.

use super::super::naming::snake;
use crate::database::introspection::Table;
use std::collections::BTreeSet;

/// Deduplicates equivalent keys; e.g. two unique `page_id` indexes yield one target.
/// Index producers must honor the catalog contract: no predicates or expressions.
///
/// Equivalence ignores column order, but the emitted key keeps the catalog's order, so
/// `UNIQUE (tenant_id, slug)` yields `tenant_id_and_slug` with arguments in that order
/// rather than a re-sorted `slug_and_tenant_id` that is easy to call with swapped values.
pub(super) fn keys(table: &Table) -> Vec<Vec<String>> {
    if table.view {
        return Vec::new();
    }
    let mut seen = BTreeSet::new();
    let mut targets = Vec::new();
    for key in table
        .uniques
        .iter()
        .map(|key| &key.columns)
        .chain(
            table
                .indexes
                .iter()
                .filter(|index| index.unique)
                .map(|index| &index.columns),
        )
        .filter(|key| !key.is_empty() && key.iter().all(|name| table.column(name).is_some()))
    {
        let mut ordered = Vec::new();
        for name in key {
            if !ordered.contains(name) {
                ordered.push(name.clone());
            }
        }
        let mut identity = ordered.clone();
        identity.sort();
        if seen.insert(identity) {
            targets.push(ordered);
        }
    }
    targets.sort_by(|left, right| {
        let mut left_sorted = left.clone();
        let mut right_sorted = right.clone();
        left_sorted.sort();
        right_sorted.sort();
        left_sorted.cmp(&right_sorted)
    });
    targets
}

/// Names a target; e.g. `suffix(&["tenant".into(), "slug".into()])` is `tenant_and_slug`.
pub(super) fn suffix(key: &[String]) -> String {
    key.iter()
        .map(|name| snake(name))
        .collect::<Vec<_>>()
        .join("_and_")
}
