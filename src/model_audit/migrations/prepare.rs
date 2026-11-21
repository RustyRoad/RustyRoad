//! Read-only generation planning and filesystem/history preflight.
use super::{groups, history, preflight, storage, Prepared};
use crate::model_audit::{confirmation::Scope, types::Report};
use clap::ArgMatches;

/// Skip conflicting tables but finish every eligible preflight before writing anything.
pub(super) fn prepare(
    report: &Report,
    args: &ArgMatches,
    scope: Scope,
) -> Result<Prepared, String> {
    let mapped = preflight::mapped(report, args)?;
    let catalog = report.catalog.as_ref().ok_or(
        "--generate-migrations needs a configured database or --schema-snapshot; no database config was detected (use --env/--database or configure RustyRoad database settings)",
    )?;
    let mut prepared = groups::collect(mapped, scope);
    for proposal in &prepared.proposals {
        storage::preflight(&report.root, proposal)?;
    }
    let mut proposals = Vec::new();
    for proposal in prepared.proposals {
        match history::inspect(&report.root, &catalog.catalog.schema, &proposal)? {
            Some(reason) => {
                prepared.diagnostics.push(format!(
                    "Migration skipped for table {}: {reason}",
                    proposal.table,
                ));
                prepared.skipped.push((proposal.table.clone(), reason));
            }
            None => proposals.push(proposal),
        }
    }
    prepared.proposals = proposals;
    Ok(prepared)
}
