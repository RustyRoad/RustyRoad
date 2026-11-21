//! Preserve conditional and conflicting import bindings as unresolved names.
use super::{super::binding::Binding, super::codecs::unconditional, use_tree};
use std::collections::BTreeMap;
use syn::Item;

/// `use uuid::Uuid` resolves only when no other import binds the same name.
pub(super) fn bindings(items: &[Item]) -> BTreeMap<String, Option<Binding>> {
    let mut result = BTreeMap::new();
    for item in items {
        let Item::Use(import) = item else {
            continue;
        };
        for (name, path) in use_tree::bindings(import) {
            let binding = if unconditional(&import.attrs) {
                path.map(Binding::Import)
            } else {
                None
            };
            result
                .entry(name)
                .and_modify(|existing| *existing = None)
                .or_insert(binding);
        }
    }
    result
}
