//! Index explicit codecs without choosing between conflicting declarations.
use super::{derived, manual, unconditional};
use crate::model_audit::schema::rust_types::imports::{namespace, Resolver};
use std::collections::BTreeMap;
use syn::Item;

/// Index concrete declarations; duplicate or conditional contracts stay unresolved.
pub(in crate::model_audit::schema::rust_types::imports) fn collect(
    items: &[Item],
    resolver: &Resolver,
) -> BTreeMap<String, Option<String>> {
    let mut result = BTreeMap::new();
    for item in items {
        let (name, attrs, generics) = match item {
            Item::Struct(value) => (&value.ident, &value.attrs, &value.generics),
            Item::Enum(value) => (&value.ident, &value.attrs, &value.generics),
            _ => continue,
        };
        let value = if generics.params.is_empty()
            && unconditional(attrs)
            && namespace::unique(items, &name.to_string())
        {
            let mut values = manual::contracts(items, name, resolver);
            if let Some(value) = derived::contract(item, resolver) {
                values.push(Some(value));
            }
            if values.len() == 1 {
                values.pop().flatten()
            } else {
                None
            }
        } else {
            None
        };
        let key = name.to_string();
        result
            .entry(key)
            .and_modify(|existing| *existing = None)
            .or_insert(value);
    }
    result
}
