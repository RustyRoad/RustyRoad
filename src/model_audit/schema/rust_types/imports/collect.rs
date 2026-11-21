//! Collect explicit imports while retaining ambiguous and locally shadowed names.
mod declarations;
mod globs;
mod imports;
mod use_tree;
use super::binding::Binding;
pub(super) use globs::paths as glob_paths;
use std::collections::BTreeMap;
use syn::Item;

/// Local type declarations shadow external codec names, e.g. `struct Uuid`.
pub(super) fn bindings(items: &[Item]) -> BTreeMap<String, Option<Binding>> {
    let mut bindings = imports::bindings(items);
    for item in items {
        let Some((name, alias)) = declarations::binding(item) else {
            continue;
        };
        bindings
            .entry(name)
            .and_modify(|existing| *existing = None)
            .or_insert(alias);
    }
    bindings
}
