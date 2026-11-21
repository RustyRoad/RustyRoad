//! Attach individual storage contracts and reconcile shared-table declarations.
use super::{
    composition, contracts, groups, matching, projections, proposals, type_mappings, types::Catalog,
};
use crate::model_audit::{
    cli::values,
    types::{Candidate, Model},
};
use clap::ArgMatches;
use std::{collections::BTreeSet, path::Path};

/// Audit every declaration before selection; reject unused primary-key contracts.
/// # Examples
/// ```sh
/// rustyroad model audit --source src/models --database --report-only
/// ```
pub(super) fn attach(
    args: &ArgMatches,
    root: &Path,
    models: &[Model],
    candidates: &mut [Candidate],
    catalog: &Catalog,
) -> Result<(), String> {
    contracts::validate(catalog)?;
    let mappings = matching::mappings(&values(args, "table"), models)?;
    let keys = contracts::keys(values(args, "primary-key"))?;
    let types = type_mappings::read(root, &values(args, "type-map"))?;
    let reviewed_projections = projections::read(&values(args, "storage-projection"), candidates)?;
    let mut consumed_keys = BTreeSet::new();
    for candidate in candidates.iter_mut() {
        let model = &candidate.model;
        let reviewed_projection = reviewed_projections.get(&model.id);
        let storage = reviewed_projection
            .cloned()
            .or_else(|| composition::storage(candidate));
        let mut identity = model.clone();
        if storage.is_some() {
            identity.kind = "struct".into();
        }
        let mut audit = matching::audit_with_storage(
            &identity,
            storage.as_ref().unwrap_or(model),
            catalog,
            mappings.get(&model.id),
            &types,
        );
        if storage.is_some() {
            audit.shared_models = vec![model.id.clone()];
        }
        let table_keys = audit.table.as_ref().and_then(|table| {
            keys.get(table).inspect(|_| {
                consumed_keys.insert(table.clone());
            })
        });
        if reviewed_projection.is_some() {
            projections::check(model, &mut audit, catalog, table_keys.map(Vec::as_slice));
        } else if storage.is_some() {
            composition::check(&mut audit, catalog, table_keys.map(Vec::as_slice));
        }
        // Existing key checks need the union of all model projections.
        // New tables retain independent contracts for conflict checks.
        if audit.status == "missing_table" {
            proposals::build(
                model,
                catalog,
                &mut audit,
                table_keys.map(Vec::as_slice).unwrap_or(&[]),
            );
        }
        candidate.table_audit = Some(audit);
    }
    for table in keys.keys() {
        if !consumed_keys.contains(table) {
            return Err(format!(
                "--primary-key {table} did not resolve to an audited table"
            ));
        }
    }
    groups::reconcile(candidates, catalog, &keys);
    Ok(())
}
