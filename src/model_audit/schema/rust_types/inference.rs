use super::{attributes, builtins, wrappers, Resolver};
use crate::model_audit::{
    schema::{type_mappings, types::Column},
    types::Field,
};
use std::collections::BTreeMap;
use syn::Type;

/// Keep wire metadata separate from SQL codecs; overrides retain priority.
/// Literal SQLx renames select storage columns without changing wire names or nullability.
/// # Examples
/// ```sh
/// rustyroad model audit --source src/models --report-only
/// ```
pub(super) fn infer(
    name: &str,
    field: &Field,
    mappings: &BTreeMap<String, String>,
    resolver: Option<&Resolver>,
) -> Result<Column, String> {
    let column_name = attributes::column_name(name, &field.attributes)?;
    let ty = syn::parse_str::<Type>(&field.ty)
        .map_err(|_| format!("{name}: unsupported type syntax"))?;
    let resolved = resolver
        .map(|resolver| resolver.resolve(&ty))
        .unwrap_or_else(|| ty.clone());
    let (inner, nullable) = if let Some(inner) = wrappers::generic(
        &resolved,
        &["Option", "std::option::Option", "core::option::Option"],
    ) {
        (inner, true)
    } else {
        (&resolved, false)
    };
    let original = if nullable {
        wrappers::generic(
            &ty,
            &["Option", "std::option::Option", "core::option::Option"],
        )
        .unwrap_or(&ty)
    } else {
        &ty
    };
    let sql_type = mappings.get(&type_mappings::spelling(original)).cloned()
        .or_else(|| mappings.get(&type_mappings::spelling(inner)).cloned())
        .or_else(|| resolver.and_then(|resolver| resolver.storage(original)))
        .or_else(|| builtins::sql(inner))
        .ok_or_else(|| format!("{name}: no PostgreSQL storage mapping for {}; set [models.type_mappings] or --type-map 'RUST_TYPE=POSTGRES_TYPE' (map the inner type for Option<T>)", field.ty))?;
    Ok(Column {
        name: column_name,
        sql_type,
        nullable,
    })
}
