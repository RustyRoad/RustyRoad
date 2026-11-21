//! Rebase explicit self/super references while preserving source-module lookup.
use super::Resolver;
use syn::{Path, Type, TypePath};

/// Resolve a relative path, delegating root-level `super` to declared modules.
pub(super) fn lookup(
    resolver: &Resolver,
    path: &Path,
    scope: usize,
    depth: usize,
) -> Option<String> {
    let parent = path.segments.first()?.ident == "super";
    if parent && scope == 0 {
        return resolver.source.as_ref()?.storage(path, scope);
    }
    let next = if parent { scope.checked_sub(1)? } else { scope };
    let mut path = path.clone();
    path.segments = path.segments.into_iter().skip(1).collect();
    let ty = Type::Path(TypePath {
        attrs: Vec::new(),
        qself: None,
        path,
    });
    super::lookup(resolver, &ty, next, depth + 1)
}
