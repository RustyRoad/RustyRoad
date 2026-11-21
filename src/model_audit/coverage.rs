//! Declaration accounting is independent of candidate selection and file writes.
mod outcomes;
mod text;
use super::types::{Candidate, Report};
use serde::Serialize;
use std::path::PathBuf;
pub(super) use text::render as text;

/// An outcome is evidence about a declaration, never permission to apply its SQL.
#[derive(Debug, Serialize)]
pub(super) struct Declaration {
    pub model_id: String,
    pub path: PathBuf,
    pub kind: String,
    pub selected: bool,
    pub table: Option<String>,
    pub outcome: String,
    pub blockers: Vec<String>,
    pub migration_directory: Option<PathBuf>,
}

/// Inventory every parsed declaration before filters, e.g. an unselected alias.
pub(super) fn inventory(candidates: &[Candidate]) -> Vec<Declaration> {
    candidates.iter().map(|candidate| {
        let audit = candidate.table_audit.as_ref();
        Declaration {
            model_id: candidate.model.id.clone(), path: candidate.model.path.clone(),
            kind: candidate.model.kind.clone(), selected: false,
            table: audit.and_then(|audit| audit.table.clone()),
            outcome: outcomes::classify(candidate).into(),
            blockers: audit.map(|audit| audit.blockers.clone()).unwrap_or_else(|| {
                if ["enum", "alias"].contains(&candidate.model.kind.as_str()) { vec![] }
                else { vec!["No catalog evidence; persistence and schema changes are unresolved".into()] }
            }),
            migration_directory: None,
        }
    }).collect()
}

/// Link generated/reused artifacts to all selected members of their shared group.
pub(super) fn reconcile(report: &mut Report) {
    for declaration in &mut report.declaration_coverage {
        let Some(candidate) = report
            .candidates
            .iter()
            .find(|candidate| candidate.model.id == declaration.model_id)
        else {
            continue;
        };
        declaration.selected = true;
        declaration.outcome = outcomes::classify(candidate).into();
        if let Some(audit) = &candidate.table_audit {
            declaration.blockers = audit.blockers.clone();
        }
        let proposal = candidate
            .table_audit
            .as_ref()
            .and_then(|audit| audit.proposal.as_ref());
        let migration = proposal.and_then(|proposal| {
            report
                .migration_files
                .iter()
                .find(|migration| migration.model_id == proposal.model_id)
        });
        if let Some(migration) = migration {
            declaration.outcome = migration.state.clone();
            declaration.migration_directory = Some(migration.directory.clone());
        }
    }
}
