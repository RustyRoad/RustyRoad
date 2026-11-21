//! Protect source-resolved SQLx aliases against lexical value/type shadowing.
use super::calls;
use crate::model_audit::schema::rust_types::Resolver;
use syn::Item;

/// A protected binding may name the row, SQLx, or a resolved constructor alias.
pub(super) fn protected(name: &syn::Ident, model: &str, resolver: &Resolver) -> bool {
    name == model
        || name == "sqlx"
        || calls::query(&syn::parse_quote!(#name), resolver, true)
        || calls::query(&syn::parse_quote!(#name::query_as), resolver, false)
}

/// Imports are checked by the scope resolver; value declarations also shadow paths.
pub(super) fn safe_item(item: &Item, model: &str, resolver: &Resolver) -> bool {
    let name = match item {
        Item::Fn(item) => &item.sig.ident,
        Item::Const(item) => &item.ident,
        Item::Static(item) => &item.ident,
        Item::Mod(item) => &item.ident,
        Item::Struct(item) if item.ident != model => &item.ident,
        Item::Enum(item) => &item.ident,
        Item::Type(item) => &item.ident,
        Item::Trait(item) => &item.ident,
        Item::Union(item) => &item.ident,
        Item::ExternCrate(item) => item
            .rename
            .as_ref()
            .map(|(_, name)| name)
            .unwrap_or(&item.ident),
        _ => return true,
    };
    !protected(name, model, resolver)
}
