//! Write prepared migrations, retaining evidence when a later write fails.
use super::{outcomes, storage};
use crate::model_audit::{schema::types::Proposal, types::Report};

/// Record every generated or reused draft; never execute its SQL.
pub(super) fn write(report: &mut Report, proposals: Vec<Proposal>) {
    for proposal in proposals {
        let schema = &report
            .catalog
            .as_ref()
            .expect("catalog preflight")
            .catalog
            .schema;
        match storage::write(&report.root, schema, &proposal) {
            Ok(migration) => {
                report.read_only &= migration.state == "already_generated";
                report.migration_files.push(migration);
            }
            Err(error) => {
                outcomes::pending(report, &format!("Migration file write failed: {error}"));
                report.read_only = false;
                report.diagnostics.push(format!(
                    "Migration file write failed: {error}. Earlier files are retained and listed; SQL was not applied."
                ));
                break;
            }
        }
    }
}
