//! Canonical SQLx path checks use explicit source bindings, never basenames.
use super::super::Resolver;
use syn::{Path, Type};

/// Resolve `use sqlx::Type as Codec` before recognizing the SQLx trait.
pub(super) fn is(path: &Path, expected: &str, resolver: &Resolver) -> bool {
    let ty = Type::Path(syn::TypePath {
        attrs: Vec::new(),
        qself: None,
        path: path.clone(),
    });
    let Type::Path(resolved) = resolver.resolve(&ty) else {
        return false;
    };
    resolved.qself.is_none()
        && resolved
            .path
            .segments
            .iter()
            .all(|part| matches!(part.arguments, syn::PathArguments::None))
        && resolved
            .path
            .segments
            .iter()
            .map(|part| part.ident.to_string())
            .collect::<Vec<_>>()
            .join("::")
            == expected
}
