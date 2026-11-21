//! Require an inferred database or the exact, non-generic SQLx PostgreSQL type.
use crate::model_audit::schema::rust_types::Resolver;
use syn::{PathArguments, Type};

/// Accept `_` or SQLx's PostgreSQL re-exports, never generic type lookalikes.
pub(super) fn postgres(ty: &Type, resolver: &Resolver) -> bool {
    if matches!(ty, Type::Infer(_)) {
        return true;
    }
    let Type::Path(ty) = resolver.resolve(ty) else {
        return false;
    };
    if ty.qself.is_some()
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
    match names.as_slice() {
        [sqlx, postgres] => sqlx == "sqlx" && postgres == "Postgres",
        [sqlx, module, postgres] => {
            sqlx == "sqlx" && module == "postgres" && postgres == "Postgres"
        }
        _ => false,
    }
}
