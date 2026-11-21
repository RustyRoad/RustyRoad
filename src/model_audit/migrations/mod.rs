//! Reviewable canonical migration generation, never execution or model replacement.

mod groups;
mod history;
mod outcomes;
mod preflight;
mod prepare;
mod provenance;
mod storage;
#[cfg(test)]
mod tests;
mod writer;

use super::types::Report;
use clap::ArgMatches;

/// Table blockers are local; file safety failures remain batch-wide.
struct Prepared {
    proposals: Vec<super::schema::types::Proposal>,
    diagnostics: Vec<String>,
    skipped: Vec<(String, String)>,
}

/// Generate selected contracts, e.g. `--table Row=rows --generate-migrations`.
pub(super) fn generate(
    report: &mut Report,
    args: &ArgMatches,
    scope: Option<super::confirmation::Scope>,
) {
    let Some(scope) = scope else {
        return;
    };
    provenance::warn(report);
    let prepared = match prepare::prepare(report, args, scope) {
        Ok(prepared) => prepared,
        Err(error) => {
            outcomes::pending(report, &error);
            report
                .diagnostics
                .push(format!("Migration generation blocked: {error}"));
            return;
        }
    };
    for (table, reason) in &prepared.skipped {
        outcomes::table(report, table, reason);
    }
    report.diagnostics.extend(prepared.diagnostics);
    if args.get_flag("dry-run") {
        return;
    }
    writer::write(report, prepared.proposals);
}
