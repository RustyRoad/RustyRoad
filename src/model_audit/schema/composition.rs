//! Transparent domain wrappers retain the schema of their selected generated row.
mod attributes;
use super::{
    rust_types::Resolver,
    types::{Catalog, TableAudit},
};
use crate::model_audit::types::{Candidate, Model};
use syn::{PathArguments, Type};

/// Uses a generated target only for an unambiguous, concrete transparent wrapper.
/// The original candidate and its caller-review warnings remain unchanged.
/// # Examples
/// ```sh
/// rustyroad model audit --source src/models --report-only
/// ```
pub(super) fn storage(candidate: &Candidate) -> Option<Model> {
    let wrapper = &candidate.model;
    let generated = candidate.target.as_ref()?;
    if wrapper.kind != "tuple_or_unit"
        || !wrapper.generics.is_empty()
        || wrapper.fields.len() != 1
        || generated.kind != "struct"
        || !generated.generics.is_empty()
        || generated.fields.is_empty()
        || !attributes::transparent(&wrapper.attributes)
    {
        return None;
    }
    let field = wrapper.fields.get("0")?;
    if !field.attributes.is_empty() {
        return None;
    }
    let parsed = syn::parse_str::<Type>(&field.ty).ok()?;
    let resolved = Resolver::for_model(wrapper).resolve(&parsed);
    let Type::Path(path) = resolved else {
        return None;
    };
    if path.qself.is_some()
        || path
            .path
            .segments
            .iter()
            .any(|segment| !matches!(segment.arguments, PathArguments::None))
    {
        return None;
    }
    let parts: Vec<_> = path
        .path
        .segments
        .iter()
        .map(|segment| segment.ident.to_string())
        .collect();
    if !parts
        .iter()
        .map(String::as_str)
        .eq(["db", "models", generated.name.as_str()])
    {
        return None;
    }
    Some(generated.clone())
}

/// Requires generated wrapper storage to agree with the observed catalog before adoption.
/// A wrapper is not a source of table DDL, including when its generated row is stale.
/// # Examples
/// ```sh
/// rustyroad model audit --source src/models --database --report-only
/// ```
pub(super) fn check(audit: &mut TableAudit, catalog: &Catalog, keys: Option<&[String]>) {
    audit.mapping = "generated_row_composition".into();
    let table = audit
        .table
        .as_ref()
        .and_then(|name| catalog.tables.iter().find(|table| &table.name == name));
    if let Some(table) = table {
        if keys.is_some_and(|keys| keys != table.primary_key.as_slice()) {
            audit
                .blockers
                .push("Generated wrapper storage cannot authorize primary-key changes".into());
        }
    }
    if table.is_none()
        || !audit.missing_columns.is_empty()
        || !audit.column_changes.is_empty()
        || !audit.database_only.is_empty()
        || !audit.incompatible_columns.is_empty()
    {
        audit.blockers.push("Generated wrapper storage differs from the catalog; repair schema/pull before adoption, never derive DDL from a generated row".into());
    }
}
