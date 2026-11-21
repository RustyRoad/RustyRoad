//! Explicit projection contracts retain source differences but never authorize DDL.
use super::types::{Catalog, TableAudit};
use crate::model_audit::types::{Candidate, Model};
use std::collections::BTreeMap;

/// Binds reviewed source projections to their already selected generated targets.
/// No conversion is inferred or verified; missing/ambiguous mappings are rejected.
/// # Examples
/// ```sh
/// rustyroad model audit --database --map Asset=CreativeAssets --storage-projection Asset=CreativeAssets --report-only
/// ```
pub(super) fn read(
    values: &[String],
    candidates: &[Candidate],
) -> Result<BTreeMap<String, Model>, String> {
    let mut mappings = BTreeMap::new();
    for value in values {
        let (source, target) = value
            .split_once('=')
            .ok_or("--storage-projection expects SOURCE=GENERATED")?;
        let found: Vec<_> = candidates
            .iter()
            .filter(|c| c.model.id == source || c.model.name == source)
            .collect();
        let [candidate] = found.as_slice() else {
            return Err(format!(
                "Projection source {source:?} is missing or ambiguous; use its full ID"
            ));
        };
        let generated = candidate
            .target
            .as_ref()
            .ok_or("Projection requires a selected generated target; supply --map")?;
        if (generated.id != target && generated.name != target)
            || generated.kind != "struct"
            || generated.fields.is_empty()
            || !generated.generics.is_empty()
            || candidate.model.kind != "struct"
            || !candidate.model.generics.is_empty()
        {
            return Err(format!(
                "Projection {source:?} must name its concrete generated row target"
            ));
        }
        if mappings
            .insert(candidate.model.id.clone(), generated.clone())
            .is_some()
        {
            return Err(format!("Duplicate projection contract for {source:?}"));
        }
    }
    Ok(mappings)
}

/// Requires complete generated/live agreement and retains persisted source key fields.
/// This mode never supplies migrations, including when another schema check finds drift.
/// # Examples
/// ```sh
/// rustyroad model audit --database --map Asset=CreativeAssets --storage-projection Asset=CreativeAssets --report-only
/// ```
pub(super) fn check(
    model: &Model,
    audit: &mut TableAudit,
    catalog: &Catalog,
    keys: Option<&[String]>,
) {
    audit.mapping = "reviewed_generated_projection".into();
    let table = audit
        .table
        .as_ref()
        .and_then(|name| catalog.tables.iter().find(|t| &t.name == name));
    if let Some(table) = table {
        if keys.is_some_and(|keys| keys != table.primary_key.as_slice())
            || table
                .primary_key
                .iter()
                .any(|key| !model.fields.contains_key(key))
        {
            audit.blockers.push(
                "Projection must preserve the existing primary-key fields and contract".into(),
            );
        }
    }
    if table.is_none()
        || !audit.missing_columns.is_empty()
        || !audit.column_changes.is_empty()
        || !audit.database_only.is_empty()
        || !audit.incompatible_columns.is_empty()
    {
        audit.blockers.push("Generated projection storage differs from the complete catalog; repair schema/pull before review, never derive DDL from the projection".into());
    }
}
