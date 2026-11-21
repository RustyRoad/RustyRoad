//! Resolve a single-name local codec or follow its explicit import binding.
use super::Resolver;
use syn::{Type, TypePath};

/// Prefer the local declaration, then follow an explicit alias such as `Status`.
pub(super) fn storage(
    resolver: &Resolver,
    name: &str,
    scope: usize,
    depth: usize,
) -> Option<String> {
    if let Some(value) = resolver.codecs.get(scope)?.get(name) {
        return value.clone();
    }
    let path = resolver
        .scopes
        .get(scope)?
        .get(name)?
        .as_ref()?
        .path()
        .clone();
    let ty = Type::Path(TypePath {
        attrs: Vec::new(),
        qself: None,
        path,
    });
    super::lookup(resolver, &ty, scope, depth + 1)
}
