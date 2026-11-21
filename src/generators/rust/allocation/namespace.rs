//! Deterministic allocation that does not steal another object's preferred spelling.

use std::collections::{BTreeMap, BTreeSet};

pub(super) fn allocate<K: Ord>(
    candidates: BTreeMap<K, String>,
    reserved: &[&str],
    separator: &str,
) -> BTreeMap<K, String> {
    let preferred: BTreeSet<_> = candidates.values().cloned().collect();
    let mut used: BTreeSet<String> = reserved.iter().map(|name| name.to_string()).collect();
    candidates
        .into_iter()
        .map(|(key, base)| {
            let mut name = base.clone();
            if !used.insert(name.clone()) {
                let mut sequence = 2;
                loop {
                    // Keyword escaping may already have left a trailing underscore.
                    name = format!("{}{separator}{sequence}", base.trim_end_matches('_'));
                    if !preferred.contains(&name) && used.insert(name.clone()) {
                        break;
                    }
                    sequence += 1;
                }
            }
            (key, name)
        })
        .collect()
}
