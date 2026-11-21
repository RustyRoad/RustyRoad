//! Validate persistence and rollback inputs before rendering migration SQL.
use super::super::{matching, types::TableAudit};
use super::{catalog_type, create};

/// Enforce reviewed identity; key redesigns separately require catalog evidence.
pub(super) fn contract(audit: &mut TableAudit, keys: &[String]) -> bool {
    if audit.status == "missing_table" {
        create::validate(audit, keys);
        if audit.mapping == "source_row_query_contract" && keys.is_empty() {
            audit.blockers.push(
                "Whole-row queries establish ownership, not keys; review --primary-key or an explicit keyless --table contract".into(),
            );
        }
    }
    if !audit.blockers.is_empty() || audit.table.is_none() {
        return false;
    }
    if !matches!(
        audit.mapping.as_str(),
        "explicit_persistence_contract"
            | "catalog_contract_match"
            | "source_row_query_contract"
            | "proposed_new_table_contract"
    ) {
        audit
            .blockers
            .push("A confirmed table identity is required".into());
        return false;
    }
    true
}

/// Check catalog-derived identifiers and types used to restore the observed schema.
pub(super) fn changes(audit: &mut TableAudit) {
    for change in &audit.column_changes {
        if let Err(error) = matching::identifier(&change.before.name) {
            audit.blockers.push(error);
        }
        if let Err(error) = catalog_type::check(&change.before.sql_type) {
            audit
                .blockers
                .push(format!("Invalid observed type for rollback: {error}"));
        }
    }
}
