//! Select CREATE or ALTER SQL after checking the reviewed storage contract.
use super::super::types::{Catalog, TableAudit};
use super::{alter, create, keys, quote, validation};

/// Return table identity and both SQL directions, or retain blockers without a draft.
/// Generated row projections and wrappers never authorize schema changes.
/// # Examples
/// ```sh
/// rustyroad model audit --source src/models --database --dry-run
/// ```
pub(super) fn sql(
    catalog: &Catalog,
    audit: &mut TableAudit,
    requested_keys: &[String],
) -> Option<(String, String, String)> {
    if matches!(
        audit.mapping.as_str(),
        "reviewed_generated_projection" | "generated_row_composition"
    ) {
        return None;
    }
    if !validation::contract(audit, requested_keys) {
        return None;
    }
    let table = audit.table.clone()?;
    let qualified = format!("{}.{}", quote(&catalog.schema), quote(&table));
    let (up, down) = if audit.status == "missing_table" {
        create::sql(&qualified, audit, requested_keys)
    } else {
        let key_change = keys::plan(catalog, audit, requested_keys);
        validation::changes(audit);
        if !audit.blockers.is_empty() {
            return None;
        }
        if audit.missing_columns.is_empty()
            && audit.column_changes.is_empty()
            && key_change.is_none()
        {
            return None;
        }
        alter::sql(&qualified, audit, key_change.as_ref())
    };
    Some((table, up, down))
}
