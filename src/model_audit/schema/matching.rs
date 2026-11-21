//! Explicit mappings and catalog-backed reconciliation of partial models.
mod fields;
mod resolve;
use super::types::{Catalog, TableAudit};
use crate::model_audit::types::Model;
use std::collections::BTreeMap;

/// Resolve `Model=table`; ambiguous declarations require their full model ID.
pub(super) fn mappings(
    values: &[String],
    models: &[Model],
) -> Result<BTreeMap<String, String>, String> {
    let mut result = BTreeMap::new();
    for mapping in values {
        let (model, table) = mapping
            .split_once('=')
            .ok_or("--table expects MODEL_ID_OR_UNIQUE_NAME=TABLE")?;
        identifier(table)?;
        let found: Vec<_> = models
            .iter()
            .filter(|m| m.id == model || m.name == model)
            .collect();
        let [model] = found.as_slice() else {
            return Err(format!(
                "Table mapping {model:?} is missing or ambiguous; use its full model ID"
            ));
        };
        if result.insert(model.id.clone(), table.into()).is_some() {
            return Err(format!("Duplicate table mapping for {}", model.id));
        }
    }
    Ok(result)
}

/// Restrict generated identifiers, e.g. `order_items`, to PostgreSQL's portable subset.
pub(super) fn identifier(value: &str) -> Result<(), String> {
    if value.is_empty()
        || value.len() > 63
        || !value
            .bytes()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'_')
        || value.as_bytes()[0].is_ascii_digit()
    {
        return Err(format!("Unsupported PostgreSQL identifier {value:?}; use lowercase letters, digits and underscores (max 63 bytes)"));
    }
    Ok(())
}

/// Compare a fixture catalog without custom codecs.
#[cfg(test)]
pub(super) fn audit(model: &Model, catalog: &Catalog, explicit: Option<&String>) -> TableAudit {
    audit_with_types(model, catalog, explicit, &BTreeMap::new())
}

/// Apply reusable storage contracts, retaining observed and desired column facts.
pub(super) fn audit_with_types(
    model: &Model,
    catalog: &Catalog,
    explicit: Option<&String>,
    type_mappings: &BTreeMap<String, String>,
) -> TableAudit {
    audit_with_storage(model, model, catalog, explicit, type_mappings)
}

/// Keep source-owned relation evidence while reading a proven inner row's fields.
/// # Examples
/// ```sh
/// rustyroad model audit --source src/models --report-only
/// ```
pub(super) fn audit_with_storage(
    model: &Model,
    storage: &Model,
    catalog: &Catalog,
    explicit: Option<&String>,
    type_mappings: &BTreeMap<String, String>,
) -> TableAudit {
    let (mut audit, table) = resolve::table(model, catalog, explicit);
    if !["ambiguous_table", "non_table"].contains(&audit.status.as_str()) {
        fields::compare(storage, table, type_mappings, &mut audit);
    }
    audit
}
