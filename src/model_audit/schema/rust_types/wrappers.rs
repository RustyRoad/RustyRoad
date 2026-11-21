//! Inspect type constructors without confusing generic arguments with paths.
use syn::{GenericArgument, PathArguments, Type};

/// Return path identity without generic arguments; `Vec<i32>` has path `Vec`.
pub(super) fn path(ty: &Type) -> Option<String> {
    let Type::Path(ty) = ty else { return None };
    if ty.qself.is_some() {
        return None;
    }
    // A single absolute name denotes an external crate, not the Rust prelude.
    // For example, `::String` cannot establish the identity of `String`.
    if ty.path.leading_colon.is_some() && ty.path.segments.len() == 1 {
        return None;
    }
    // Only the final segment can be a supported type constructor.
    if ty
        .path
        .segments
        .iter()
        .rev()
        .skip(1)
        .any(|segment| !matches!(segment.arguments, PathArguments::None))
    {
        return None;
    }
    Some(
        ty.path
            .segments
            .iter()
            .map(|s| s.ident.to_string())
            .collect::<Vec<_>>()
            .join("::"),
    )
}

/// Read a concrete scalar identity; `Uuid<T>` does not prove a UUID codec.
pub(super) fn scalar(ty: &Type) -> Option<String> {
    let Type::Path(value) = ty else { return None };
    if !matches!(value.path.segments.last()?.arguments, PathArguments::None) {
        return None;
    }
    path(ty)
}

/// Read one type argument, e.g. `generic(Vec<i32>, ["Vec"])` returns `i32`.
pub(super) fn generic<'a>(ty: &'a Type, names: &[&str]) -> Option<&'a Type> {
    if !names.contains(&path(ty)?.as_str()) {
        return None;
    }
    let Type::Path(ty) = ty else { return None };
    let PathArguments::AngleBracketed(arguments) = &ty.path.segments.last()?.arguments else {
        return None;
    };
    if arguments.args.len() != 1 {
        return None;
    }
    match arguments.args.first()? {
        GenericArgument::Type(inner) => Some(inner),
        _ => None,
    }
}
