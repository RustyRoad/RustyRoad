//! Apply scope bindings after normalizing relative paths and type arguments.
use super::super::Resolver;
use syn::{Type, TypePath};

/// Unknown bindings and glob namespaces must remain explicit unresolved types.
pub(super) fn resolve(
    resolver: &Resolver,
    mut resolved: TypePath,
    scope: usize,
    depth: usize,
    name: &syn::Ident,
) -> Type {
    let binding = resolver
        .scopes
        .get(scope)
        .and_then(|map| map.get(&name.to_string()));
    match binding {
        Some(Some(binding)) => {
            let Some((expanded, identity)) = binding.expand(&resolved.path) else {
                return syn::parse_str("__rustyroad_unresolved_alias_shape").unwrap();
            };
            resolved.path = expanded;
            if identity {
                return Type::Path(resolved);
            }
            resolver.resolve_at(&Type::Path(resolved), scope, depth + 1)
        }
        Some(None) => syn::parse_str("__rustyroad_unresolved_local_type").unwrap(),
        None if resolver
            .scopes
            .get(scope)
            .is_some_and(|map| map.contains_key("*")) =>
        {
            // A glob may shadow the prelude or an external crate's name.
            // Only absolute paths bypass unresolved wildcard namespaces.
            syn::parse_str("__rustyroad_unresolved_glob_import").unwrap()
        }
        None => Type::Path(resolved),
    }
}
