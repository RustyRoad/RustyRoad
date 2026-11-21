//! Human-readable table facts and SQL previews; output is not database execution.

use super::types::TableAudit;
use std::fmt::Write;

/// Show actual table differences separately from generated-Rust comparisons.
pub(in crate::model_audit) fn table(audit: &TableAudit) -> String {
    let mut out = format!(
        "  DATABASE: {} [{}; {}]\n",
        audit.table.as_deref().unwrap_or("unmapped"),
        audit.status,
        audit.mapping
    );
    if audit.shared_models.len() > 1 {
        let _ = writeln!(
            out,
            "    Shared table models (one migration): {}",
            audit.shared_models.join(", ")
        );
    }
    for column in &audit.missing_columns {
        let _ = writeln!(
            out,
            "    Missing column: {} {} nullable={}",
            column.name, column.sql_type, column.nullable
        );
    }
    for change in &audit.column_changes {
        let _ = writeln!(
            out,
            "    Change column: {} {} nullable={} -> {} nullable={}",
            change.after.name,
            change.before.sql_type,
            change.before.nullable,
            change.after.sql_type,
            change.after.nullable
        );
    }
    if !audit.database_only.is_empty() {
        let _ = writeln!(
            out,
            "    Database-only columns (retained): {}",
            audit.database_only.join(", ")
        );
    }
    for (name, reason) in &audit.incompatible_columns {
        let _ = writeln!(out, "    Incompatible {name}: {reason}");
    }
    for blocker in &audit.blockers {
        let _ = writeln!(out, "    DDL BLOCKER: {blocker}");
    }
    if let Some(proposal) = &audit.proposal {
        let _ = writeln!(
            out,
            "    Proposed UP (not applied):\n{}    Proposed DOWN:\n{}",
            proposal.up, proposal.down
        );
        for warning in &proposal.warnings {
            let _ = writeln!(out, "    DDL REVIEW: {warning}");
        }
    }
    out
}
