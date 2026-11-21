//! Reviewable schema reconciliation: generation is not SQL execution.
mod alter;
mod catalog_type;
mod create;
mod keys;
mod planning;
mod review;
mod validation;
use super::types::{Catalog, Proposal, TableAudit};
use crate::model_audit::types::Model;

/// Plan additions and existing-column changes without inventing defaults or backfill values.
pub(super) fn build(model: &Model, catalog: &Catalog, audit: &mut TableAudit, keys: &[String]) {
    let Some((table, up, down)) = planning::sql(catalog, audit, keys) else {
        return;
    };
    audit.proposal = Some(Proposal {
        model_id: model.id.clone(),
        table,
        operation: if audit.status == "missing_table" {
            "create_table"
        } else {
            "alter_table"
        }
        .into(),
        columns: audit
            .missing_columns
            .iter()
            .map(|c| c.name.clone())
            .chain(audit.column_changes.iter().map(|c| c.after.name.clone()))
            .collect(),
        up,
        down,
        warnings: review::warnings(audit),
    });
}

/// Quote an already checked SQL identifier, e.g. `rows` -> `"rows"`.
fn quote(name: &str) -> String {
    format!("\"{}\"", name.replace('"', "\"\""))
}
