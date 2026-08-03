//! Confirmation and reporting for `migration baseline`.

use clap::ArgMatches;
use std::io::{self, IsTerminal};

/// Returns `true` when the operation may proceed.
///
/// Baselining asserts that the database already contains every migration's
/// changes, so it requires explicit acknowledgement.
pub(super) fn confirmed(matches: &ArgMatches, count: usize) -> bool {
    if matches.get_flag("yes") {
        return true;
    }

    println!(
        "This records {count} migration(s) as applied WITHOUT running them.\n\
         Only do this when the database schema already reflects them.\n"
    );

    if !io::stdin().is_terminal() {
        eprintln!("Refusing to baseline without confirmation. Re-run with --yes.");
        return false;
    }

    dialoguer::Confirm::new()
        .with_prompt("Record these migrations as applied?")
        .default(false)
        .interact()
        .unwrap_or(false)
}

/// Prints what the baseline changed.
pub(super) fn report(recorded: &[String], total: usize) {
    if recorded.is_empty() {
        println!("Ledger already up to date; {total} migration(s) were already recorded.");
        return;
    }

    println!("Recorded {} migration(s) as applied:", recorded.len());
    for id in recorded {
        println!("  {id}");
    }

    let skipped = total - recorded.len();
    if skipped > 0 {
        println!("{skipped} migration(s) were already recorded.");
    }
    println!("\nSubsequent 'rustyroad migration all' runs will skip these.");
}
