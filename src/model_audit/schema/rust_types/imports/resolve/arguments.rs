//! Resolve nested type arguments without changing the enclosing path identity.
use super::super::Resolver;
use syn::{GenericArgument, Path, PathArguments};

/// Expand argument aliases, e.g. `Vec<Key>`, at the original lexical scope.
pub(super) fn resolve(resolver: &Resolver, path: &mut Path, scope: usize, depth: usize) {
    for segment in &mut path.segments {
        if let PathArguments::AngleBracketed(args) = &mut segment.arguments {
            for argument in &mut args.args {
                if let GenericArgument::Type(inner) = argument {
                    *inner = resolver.resolve_at(inner, scope, depth + 1);
                }
            }
        }
    }
}
