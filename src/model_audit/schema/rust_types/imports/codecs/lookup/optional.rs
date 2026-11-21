//! Recognize optional wrappers without erasing their original local type names.
use super::Resolver;
use syn::{GenericArgument, PathArguments, Type};

/// Expand only the wrapper identity; `Option<Local>` must retain `Local` itself.
pub(super) fn inner<'a>(resolver: &Resolver, ty: &'a Type, scope: usize) -> Option<&'a Type> {
    let resolved = resolver.resolve_at(ty, scope, 0);
    crate::model_audit::schema::rust_types::wrappers::generic(
        &resolved,
        &["Option", "std::option::Option", "core::option::Option"],
    )?;
    let Type::Path(value) = ty else { return None };
    let PathArguments::AngleBracketed(args) = &value.path.segments.last()?.arguments else {
        return None;
    };
    if args.args.len() != 1 {
        return None;
    }
    match args.args.first()? {
        GenericArgument::Type(inner) => Some(inner),
        _ => None,
    }
}
