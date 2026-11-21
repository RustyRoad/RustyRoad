//! Resolve relation identity without requiring every stored column in a projection.
mod declaration;
mod eligibility;
mod identity;
mod queries;
use super::super::types::{Catalog, Table, TableAudit};
use crate::model_audit::{compare, types::Model};

/// A unique name match allows partial projections, e.g. `Row` can target `rows`.
pub(super) fn table<'a>(
    model: &Model,
    catalog: &'a Catalog,
    explicit: Option<&String>,
) -> (TableAudit, Option<&'a Table>) {
    if explicit.is_none()
        && (["enum", "alias"].contains(&model.kind.as_str()) || declaration::service_marker(model))
    {
        return (identity::non_table(model), None);
    }
    let targets = explicit
        .map(|_| Default::default())
        .unwrap_or_else(|| queries::tables(model));
    if targets.len() > 1 {
        let mut audit = identity::audit(model, String::new(), false, &[]);
        audit.status = "ambiguous_table".into();
        audit.mapping = "conflicting_source_row_queries".into();
        audit.table = None;
        audit.candidates = targets.into_iter().collect();
        audit
            .blockers
            .push("Conflicting whole-row query targets require --table MODEL=TABLE".into());
        eligibility::check(model, catalog, None, &mut audit);
        return (audit, None);
    }
    let target = explicit.or_else(|| targets.first());
    let candidates: Vec<_> = catalog
        .tables
        .iter()
        .filter(|table| {
            target
                .map(|name| table.name == *name)
                .unwrap_or_else(|| compare::name_key(&model.name) == compare::name_key(&table.name))
        })
        .collect();
    let table = (candidates.len() == 1).then(|| candidates[0]);
    let name = target
        .cloned()
        .or_else(|| table.map(|table| table.name.clone()))
        .unwrap_or_else(|| crate::generators::rust::casing::to_snake(&model.name));
    let mut audit = identity::audit(model, name, explicit.is_some(), &candidates);
    if explicit.is_none() && !targets.is_empty() {
        audit.mapping = "source_row_query_contract".into();
    }
    eligibility::check(model, catalog, table, &mut audit);
    (audit, table)
}
