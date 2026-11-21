//! Retain generation-time blockers on the declarations that could not get drafts.
use crate::model_audit::types::Report;

/// Mark one skipped table, e.g. when migration history overlaps its proposal.
pub(super) fn table(report: &mut Report, table: &str, reason: &str) {
    for candidate in &mut report.candidates {
        let Some(audit) = &mut candidate.table_audit else {
            continue;
        };
        if audit.table.as_deref() == Some(table) && audit.proposal.is_some() {
            let blocker = format!("Migration generation unresolved: {reason}");
            if !audit.blockers.contains(&blocker) {
                audit.blockers.push(blocker);
            }
        }
    }
}

/// Mark unwritten proposals after a batch failure; retain earlier artifact evidence.
pub(super) fn pending(report: &mut Report, reason: &str) {
    for candidate in &mut report.candidates {
        let Some(audit) = &mut candidate.table_audit else {
            continue;
        };
        let Some(proposal) = &audit.proposal else {
            continue;
        };
        if report
            .migration_files
            .iter()
            .any(|file| file.model_id == proposal.model_id)
        {
            continue;
        }
        let blocker = format!("Migration generation unresolved: {reason}");
        if !audit.blockers.contains(&blocker) {
            audit.blockers.push(blocker);
        }
    }
}
