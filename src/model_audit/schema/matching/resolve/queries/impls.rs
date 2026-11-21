//! Named rows in repository helpers retain identity without treating `Self` as a row.
use super::{calls, namespace, walk};
use crate::model_audit::{schema::rust_types::Resolver, types::Model};
use std::collections::BTreeSet;

/// Example: `impl Repository { fn load() { query_as::<_, Row>(...); } }`.
/// Only an implementation of the audited row may use `Self` as row evidence.
pub(super) fn collect(
    implementation: &syn::ItemImpl,
    model: &Model,
    resolver: &Resolver,
    targets: &mut BTreeSet<String>,
) {
    if !namespace::unconditional(&implementation.attrs)
        || implementation.trait_.is_some()
        || !implementation.generics.params.is_empty()
    {
        return;
    }
    let own = calls::row(&implementation.self_ty, model, false);
    for item in &implementation.items {
        if let syn::ImplItem::Fn(function) = item {
            if namespace::unconditional(&function.attrs) {
                walk::collect(
                    &function.block,
                    &function.sig,
                    model,
                    own,
                    resolver,
                    targets,
                );
            }
        }
    }
}
