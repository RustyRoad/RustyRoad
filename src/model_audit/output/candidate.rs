//! Explain declaration comparison separately from PostgreSQL storage evidence.
use crate::model_audit::types::Report;
use std::fmt::Write;

/// Preserve per-model blockers in text output, e.g. an unknown storage codec.
pub(super) fn append(out: &mut String, report: &Report) {
    for candidate in &report.candidates {
        let _ = writeln!(
            out,
            "\n{}: {} ({})",
            candidate.model.id, candidate.classification, candidate.match_basis
        );
        let _ = writeln!(out, "  Usage analysis: {:?}", candidate.usage);
        if let Some(target) = &candidate.target {
            let _ = writeln!(out, "  Generated target: {}", target.id);
        }
        let _ = writeln!(
            out,
            "  Declaration differences: {:?}",
            candidate.differences
        );
        for blocker in &candidate.blockers {
            let _ = writeln!(out, "  Blocker: {blocker}");
        }
        for action in &candidate.actions {
            let _ = writeln!(out, "  Review: {action}");
        }
        if let Some(audit) = &candidate.table_audit {
            let _ = writeln!(
                out,
                "  Table: {:?}; mapping: {}; status: {}",
                audit.table, audit.mapping, audit.status
            );
            for blocker in &audit.blockers {
                let _ = writeln!(out, "  Schema blocker: {blocker}");
            }
            if let Some(proposal) = &audit.proposal {
                let _ = writeln!(
                    out,
                    "  Proposal: {} ({})",
                    proposal.operation, proposal.table
                );
                let _ = writeln!(out, "  Up SQL (not applied):\n{}", proposal.up);
                let _ = writeln!(out, "  Down SQL (not applied):\n{}", proposal.down);
            }
        }
    }
}
