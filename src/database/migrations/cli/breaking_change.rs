mod approve;
mod pending;
mod report;
mod scan;

#[cfg(test)]
mod approve_test;
#[cfg(test)]
mod pending_test;

use crate::database::migrations::BreakingChangeFinding;
use clap::{Arg, ArgAction, ArgMatches};
use std::io;

pub(super) fn allow_breaking_arg() -> Arg {
    Arg::new("allow-breaking")
        .long("allow-breaking")
        .global(true)
        .action(ArgAction::SetTrue)
        .help("Apply migrations after acknowledging detected breaking schema changes")
}

/// Warns about a freshly generated migration.
///
/// A newly generated migration cannot already be applied, so this deliberately
/// scans the file directly rather than consulting the ledger.
pub(super) fn generated(name: &str) {
    match scan::named_on_disk(name) {
        Ok(findings) => report::warnings(&findings),
        Err(error) => eprintln!("Could not analyze generated migration: {error}"),
    }
}

pub(super) async fn approve_named(name: &str, matches: &ArgMatches) -> io::Result<()> {
    approve(scan::named(name).await?, matches)
}

pub(super) async fn approve_all(matches: &ArgMatches) -> io::Result<()> {
    approve(scan::all().await?, matches)
}

fn approve(findings: Vec<BreakingChangeFinding>, matches: &ArgMatches) -> io::Result<()> {
    report::warnings(&findings);
    approve::changes(&findings, matches.get_flag("allow-breaking"))
}
