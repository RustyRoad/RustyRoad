//! Inspect scalar source identities without guessing custom codec storage.
use syn::{GenericArgument, PathArguments, Type};

/// Unwrap one standard Option; all other constructors retain their own identity.
/// # Examples
/// ```sh
/// rustyroad model audit --source src/models --report-only
/// ```
pub(super) fn inner(ty: &Type) -> &Type {
    let Type::Path(path) = ty else { return ty };
    if path.qself.is_some() {
        return ty;
    }
    let name = quote::quote!(#path).to_string().replace(' ', "");
    if !["Option<", "std::option::Option<", "core::option::Option<"]
        .iter()
        .any(|prefix| name.starts_with(prefix))
    {
        return ty;
    }
    let Some(segment) = path.path.segments.last() else {
        return ty;
    };
    let PathArguments::AngleBracketed(arguments) = &segment.arguments else {
        return ty;
    };
    if arguments.args.len() != 1 {
        return ty;
    }
    match arguments.args.first() {
        Some(GenericArgument::Type(inner)) => inner,
        _ => ty,
    }
}

/// Recognize only the builtin String paths after the source resolver has run.
/// # Examples
/// ```sh
/// rustyroad model audit --source src/models --report-only
/// ```
pub(super) fn is_string(ty: &Type) -> bool {
    matches!(
        quote::quote!(#ty).to_string().replace(' ', "").as_str(),
        "String" | "std::string::String" | "alloc::string::String"
    )
}
