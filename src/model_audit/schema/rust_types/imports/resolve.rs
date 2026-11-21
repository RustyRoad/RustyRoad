//! Bounded alias expansion with module-local shadowing and generic recursion.
mod arguments;
mod binding;
use super::Resolver;
use syn::Type;

impl Resolver {
    /// Expand only source-proven aliases; cycles and unknown imports stay unknown.
    pub(super) fn resolve_at(&self, ty: &Type, scope: usize, depth: usize) -> Type {
        if depth >= 16 {
            return syn::parse_str("__rustyroad_unresolved_alias_cycle").unwrap();
        }
        let Type::Path(original) = ty else {
            return ty.clone();
        };
        // Do not erase generic namespace arguments while expanding aliases or
        // removing `self`/`super`; they cannot prove a concrete codec identity.
        if super::super::wrappers::path(ty).is_none() {
            return ty.clone();
        }
        let mut resolved = original.clone();
        arguments::resolve(self, &mut resolved.path, scope, depth);
        // Absolute paths name the external crate, regardless of local aliases.
        if resolved.path.leading_colon.is_some() {
            return Type::Path(resolved);
        }
        let Some(first) = resolved.path.segments.first().cloned() else {
            return ty.clone();
        };
        if first.ident == "super" && scope > 0 {
            resolved.path.segments = resolved.path.segments.into_iter().skip(1).collect();
            return self.resolve_at(&Type::Path(resolved), scope - 1, depth + 1);
        }
        if first.ident == "self" {
            resolved.path.segments = resolved.path.segments.into_iter().skip(1).collect();
            return self.resolve_at(&Type::Path(resolved), scope, depth + 1);
        }
        binding::resolve(self, resolved, scope, depth, &first.ident)
    }
}
