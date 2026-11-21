//! Preserve constructor aliases when inspecting function-local bindings.
use crate::model_audit::schema::rust_types::Resolver;

/// A binding named `db` may shadow `use sqlx as db`, not just `sqlx` itself.
pub(super) fn protected(name: &syn::Ident, model: &str, resolver: &Resolver) -> bool {
    name == "sqlx"
        || name == model
        || super::super::super::calls::query(&syn::parse_quote!(#name), resolver, true)
        || super::super::super::calls::query(&syn::parse_quote!(#name::query_as), resolver, true)
}
