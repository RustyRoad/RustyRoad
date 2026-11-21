//! Recognize the exact row declaration, never a qualified lookalike.
use crate::model_audit::types::Model;
use syn::{PathArguments, Type};

/// Accept `Row`, `self::Row`, or inherent `Self`, never a qualified lookalike.
pub(in crate::model_audit::schema::matching::resolve::queries) fn row(
    ty: &Type,
    model: &Model,
    own: bool,
) -> bool {
    let Type::Path(ty) = ty else { return false };
    if ty.qself.is_some()
        || ty.path.leading_colon.is_some()
        || ty
            .path
            .segments
            .iter()
            .any(|s| !matches!(s.arguments, PathArguments::None))
    {
        return false;
    }
    let names: Vec<_> = ty
        .path
        .segments
        .iter()
        .map(|s| s.ident.to_string())
        .collect();
    names == [model.name.as_str()]
        || names == ["self", model.name.as_str()]
        || (own && names == ["Self"])
}
