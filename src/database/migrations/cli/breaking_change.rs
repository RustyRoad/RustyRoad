mod approve;
mod report;
mod scan;

#[cfg(test)]
mod approve_test;

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

pub(super) fn generated(name: &str) {
    match scan::named(name) {
        Ok(findings) => report::warnings(&findings),
        Err(error) => eprintln!("Could not analyze generated migration: {error}"),
    }
}

pub(super) fn approve_named(name: &str, matches: &ArgMatches) -> io::Result<()> {
    approve(scan::named(name)?, matches)
}

pub(super) fn approve_all(matches: &ArgMatches) -> io::Result<()> {
    approve(scan::all()?, matches)
}

fn approve(findings: Vec<BreakingChangeFinding>, matches: &ArgMatches) -> io::Result<()> {
    report::warnings(&findings);
    approve::changes(&findings, matches.get_flag("allow-breaking"))
}
