//! Keep actual migration file outcomes distinct from proposals and diagnostics.
use crate::model_audit::types::Report;
use std::fmt::Write;

/// Render durable file paths and unresolved diagnostics without claiming SQL execution.
pub(super) fn append(out: &mut String, report: &Report) {
    for migration in &report.migration_files {
        let _ = writeln!(
            out,
            "Migration {}: {} [{}]",
            migration.state,
            migration.directory.display(),
            migration.model_id
        );
    }
    for diagnostic in &report.diagnostics {
        let _ = writeln!(out, "Diagnostic: {diagnostic}");
    }
    for limitation in &report.limitations {
        let _ = writeln!(out, "Limitation: {limitation}");
    }
}
