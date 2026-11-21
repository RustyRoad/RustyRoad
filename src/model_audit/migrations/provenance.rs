//! Retain incomplete-discovery warnings in generated migration manifests.
use crate::model_audit::types::Report;

/// Mark eligible drafts when discovery has diagnostics; never suppress those diagnostics.
pub(super) fn warn(report: &mut Report) {
    if report.diagnostics.is_empty() {
        return;
    }
    let warning = "Source discovery is incomplete; review all audit diagnostics and source contracts before applying SQL";
    for candidate in &mut report.candidates {
        if let Some(proposal) = candidate
            .table_audit
            .as_mut()
            .and_then(|audit| audit.proposal.as_mut())
        {
            if !proposal.warnings.iter().any(|existing| existing == warning) {
                proposal.warnings.push(warning.into());
            }
        }
    }
}
