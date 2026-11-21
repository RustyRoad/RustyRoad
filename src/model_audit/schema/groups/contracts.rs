//! Shared-table consistency includes unchanged fields, not just requested additions.
use super::super::{
    rust_types,
    types::{Column, TableAudit},
};
use crate::model_audit::types::Candidate;
use std::collections::BTreeMap;

/// Reject contradictory storage contracts even when one model already matches the catalog.
/// # Examples
/// ```sh
/// rustyroad model audit --source src/models --database --report-only
/// ```
pub(super) fn conflicts(candidates: &[Candidate], indices: &[usize], table: &str) -> Vec<String> {
    let mut desired: BTreeMap<&str, &Column> = BTreeMap::new();
    let mut blockers = Vec::new();
    if indices.iter().any(|i| {
        matches!(
            candidates[*i]
                .table_audit
                .as_ref()
                .unwrap()
                .mapping
                .as_str(),
            "reviewed_generated_projection" | "generated_row_composition"
        )
    }) && indices.iter().any(|i| {
        let audit = candidates[*i].table_audit.as_ref().unwrap();
        !audit.missing_columns.is_empty() || !audit.column_changes.is_empty()
    }) {
        blockers.push(format!(
            "Shared table {table}: generated storage review cannot authorize peer schema changes"
        ));
    }
    for index in indices {
        let candidate = &candidates[*index];
        let audit = candidate.table_audit.as_ref().unwrap();
        if !audit.blockers.is_empty() {
            blockers.push(format!(
                "Shared table {table}: {} requires review",
                candidate.model.id
            ));
        }
        for column in &audit.desired_columns {
            if let Some(previous) = desired.insert(&column.name, column) {
                if rust_types::normalized(&previous.sql_type)
                    != rust_types::normalized(&column.sql_type)
                    || previous.nullable != column.nullable
                {
                    blockers.push(format!(
                        "Shared table {table}: conflicting definitions for {}",
                        column.name
                    ));
                }
            }
        }
    }
    blockers.sort();
    blockers.dedup();
    blockers
}

/// Union additions and alterations deterministically after every desired contract agrees.
/// # Examples
/// ```sh
/// rustyroad model audit --source src/models --database --dry-run
/// ```
pub(super) fn merge(candidates: &[Candidate], indices: &[usize], combined: &mut TableAudit) {
    let mut additions = BTreeMap::new();
    let mut changes = BTreeMap::new();
    let mut desired = BTreeMap::new();
    for index in indices {
        let audit = candidates[*index].table_audit.as_ref().unwrap();
        for column in &audit.missing_columns {
            additions.insert(column.name.clone(), column.clone());
        }
        for change in &audit.column_changes {
            changes.insert(change.after.name.clone(), change.clone());
        }
        for column in &audit.desired_columns {
            desired.insert(column.name.clone(), column.clone());
        }
    }
    combined.missing_columns = additions.into_values().collect();
    combined.column_changes = changes.into_values().collect();
    combined.desired_columns = desired.into_values().collect();
}
