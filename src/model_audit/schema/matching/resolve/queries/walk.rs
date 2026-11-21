//! Walk literal calls only after checking all lexical bindings in the function.
mod conditional;
mod guard;
mod rows;
use crate::model_audit::{schema::rust_types::Resolver, types::Model};
use rows::Rows;
use std::collections::BTreeSet;
use syn::visit::Visit;

/// Gather safe calls, e.g. `sqlx::query_as::<_, Self>("SELECT * FROM rows")`.
pub(super) fn collect(
    block: &syn::Block,
    sig: &syn::Signature,
    model: &Model,
    own: bool,
    resolver: &Resolver,
    targets: &mut BTreeSet<String>,
) {
    if !sig.generics.params.is_empty() {
        return;
    }
    if guard::safe(block, sig, &model.name, resolver) {
        Rows {
            model,
            own,
            resolver,
            targets,
        }
        .visit_block(block);
    }
}
