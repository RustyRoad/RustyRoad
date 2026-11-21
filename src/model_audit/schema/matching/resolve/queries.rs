//! Conservative ownership evidence from literal, typed SQLx whole-row queries.
mod bindings;
mod calls;
mod collect;
mod impls;
mod namespace;
mod scope;
mod sql;
mod walk;
use crate::model_audit::{schema::rust_types::Resolver, types::Model};
use std::collections::BTreeSet;

/// Collect targets, e.g. `query_as::<_, Row>("SELECT * FROM rows")`.
pub(super) fn tables(model: &Model) -> BTreeSet<String> {
    let Some(file) = std::fs::read_to_string(&model.path)
        .ok()
        .and_then(|source| syn::parse_file(&source).ok())
    else {
        return BTreeSet::new();
    };
    if !namespace::unconditional(&file.attrs) {
        return BTreeSet::new();
    }
    let Some(items) = scope::items(&file.items, model) else {
        return BTreeSet::new();
    };
    if !namespace::unshadowed(items, &model.name) {
        return BTreeSet::new();
    }
    let resolver = Resolver::for_model(model);
    if !items
        .iter()
        .all(|item| bindings::safe_item(item, &model.name, &resolver))
    {
        return BTreeSet::new();
    }
    collect::tables(items, model, &resolver)
}
