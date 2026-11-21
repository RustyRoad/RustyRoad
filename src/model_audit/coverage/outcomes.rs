//! Keep absent catalog evidence separate from a verified unchanged contract.
use crate::model_audit::types::Candidate;

/// A proposal is not a generated artifact, e.g. when generation was not requested.
pub(super) fn classify(candidate: &Candidate) -> &'static str {
    let Some(audit) = &candidate.table_audit else {
        return if ["enum", "alias"].contains(&candidate.model.kind.as_str()) {
            "non_table_declaration"
        } else {
            "unresolved_contract"
        };
    };
    if audit.status == "non_table" {
        return "non_table_declaration";
    }
    if !audit.blockers.is_empty() {
        return "unresolved_contract";
    }
    if audit.proposal.is_some() {
        return "migration_proposed";
    }
    if audit.status == "table_matched" {
        return "no_schema_change";
    }
    "unresolved_contract"
}
