//! Data-specific repairs cannot be invented from a Rust declaration.
use super::super::types::TableAudit;

/// Persist actionable warnings in the review manifest alongside both SQL directions.
pub(super) fn warnings(audit: &TableAudit) -> Vec<String> {
    let mut warnings = vec![
        "Review inferred storage types and business constraints; defaults, identity, unique/FK/index/check constraints are not inferred from structs".into(),
        "DOWN can destroy added data or fail after subsequent writes; reverse casts are not guaranteed lossless".into(),
        "Catalog observations can become stale; re-audit before applying. This command never applies SQL".into(),
        "Explicit storage mappings do not create database types/extensions or verify Rust codecs; review their availability and compatibility before applying".into(),
    ];
    if audit.status == "missing_table" {
        warnings.push("No existing relation proves persistence ownership; confirm this declaration should own a table before applying SQL".into());
    }
    if !audit.column_changes.is_empty() {
        warnings.push("Review explicit USING casts in both directions: PostgreSQL may reject unsupported casts, defaults or dependent constraints, or truncate/round values; ALTER TABLE may lock/rewrite the table".into());
    }
    if audit.missing_columns.iter().any(|column| !column.nullable) {
        warnings.push("Adding required columns fails on populated tables without a reviewed default/backfill. Before applying, review a staged nullable ADD, domain-specific backfill and SET NOT NULL in one transaction; no values are fabricated".into());
    }
    if audit
        .column_changes
        .iter()
        .any(|change| change.before.nullable && !change.after.nullable)
    {
        warnings.push("SET NOT NULL fails while NULLs exist. Review/backfill those rows before applying; no rows are deleted and no replacement values are guessed".into());
    }
    if audit
        .column_changes
        .iter()
        .any(|change| !change.before.nullable && change.after.nullable)
    {
        warnings.push("DOWN restores NOT NULL and will fail if later writes introduced NULLs; review/backfill before rollback".into());
    }
    warnings
}
