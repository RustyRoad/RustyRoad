//! Resolve SQLx constructor identity without trusting function basenames.
use crate::model_audit::schema::rust_types::Resolver;
use syn::{PathArguments, Type};

/// Resolve aliases, e.g. `use sqlx as db`, without trusting function basenames.
pub(in crate::model_audit::schema::matching::resolve::queries) fn query(
    path: &syn::Path,
    resolver: &Resolver,
    macros: bool,
) -> bool {
    let mut path = path.clone();
    let Some(last) = path.segments.last_mut() else {
        return false;
    };
    if macros && !matches!(last.arguments, PathArguments::None) {
        return false;
    }
    last.arguments = PathArguments::None;
    if path
        .segments
        .iter()
        .any(|s| !matches!(s.arguments, PathArguments::None))
    {
        return false;
    }
    let resolved = resolver.resolve(&syn::parse_quote!(#path));
    let Type::Path(ty) = resolved else {
        return false;
    };
    if ty.qself.is_some()
        || ty
            .path
            .segments
            .iter()
            .any(|s| !matches!(s.arguments, PathArguments::None))
    {
        return false;
    }
    let names: Vec<_> = ty
        .path
        .segments
        .iter()
        .map(|s| s.ident.to_string())
        .collect();
    names == ["sqlx", "query_as"] || (macros && names == ["sqlx", "query_as_unchecked"])
}
