//! Collect row-query targets only from unconditional functions and implementations.
use super::{impls, namespace, walk};
use crate::model_audit::{schema::rust_types::Resolver, types::Model};
use std::collections::BTreeSet;

/// For example, collect a whole-row query in `impl Row { fn load() { ... } }`.
pub(super) fn tables(items: &[syn::Item], model: &Model, resolver: &Resolver) -> BTreeSet<String> {
    let mut targets = BTreeSet::new();
    for item in items {
        match item {
            syn::Item::Fn(function) if namespace::unconditional(&function.attrs) => {
                walk::collect(
                    &function.block,
                    &function.sig,
                    model,
                    false,
                    resolver,
                    &mut targets,
                );
            }
            syn::Item::Impl(implementation) => {
                impls::collect(implementation, model, resolver, &mut targets);
            }
            _ => {}
        }
    }
    targets
}
