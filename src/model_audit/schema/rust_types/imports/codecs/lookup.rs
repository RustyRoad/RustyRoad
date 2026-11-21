//! Scope-aware storage lookup, retaining original names inside optional wrappers.
mod binding;
mod optional;
mod relative;
use super::super::Resolver;
use syn::{PathArguments, Type};

/// Resolve explicit local contracts and imports, never external names by basename.
///
/// Standard optional wrappers include `std::option::Option<Local>` and imported
/// aliases; resolving their inner type would erase the local declaration name.
pub(in crate::model_audit::schema::rust_types::imports) fn lookup(
    resolver: &Resolver,
    ty: &Type,
    scope: usize,
    depth: usize,
) -> Option<String> {
    if depth > 16 {
        return None;
    }
    let Type::Path(value) = ty else { return None };
    if value.qself.is_some() {
        return None;
    }
    let segments = &value.path.segments;
    let first = segments.first()?;
    if let Some(inner) = optional::inner(resolver, ty, scope) {
        return lookup(resolver, inner, scope, depth + 1);
    }
    if value.path.leading_colon.is_some()
        || segments
            .iter()
            .any(|part| !matches!(part.arguments, PathArguments::None))
    {
        return None;
    }
    if first.ident == "super" || first.ident == "self" {
        return relative::lookup(resolver, &value.path, scope, depth);
    }
    if segments.len() != 1 {
        return resolver.source.as_ref()?.storage(&value.path, scope);
    }
    binding::storage(resolver, &first.ident.to_string(), scope, depth)
}
