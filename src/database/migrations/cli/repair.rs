//! `rustyroad migration repair-ledger` — normalize a legacy ledger.

use crate::database::migrations::{baseline, ledger};
use clap::{Arg, ArgAction, ArgMatches, Command};
use std::io::{self, IsTerminal};

/// Returns the explicit legacy-ledger repair subcommand.
pub(crate) fn command() -> Command {
    Command::new("repair-ledger")
        .about("Deduplicate and constrain the migration ledger")
        .long_about(
            "Keeps the newest row for each migration identity, removes superseded\n\
             duplicate history, and enforces UNIQUE (name, direction).\n\n\
             The operation runs in one database transaction and is idempotent.\n\
             It never executes migration up.sql or down.sql files.\n\n\
             CONFIG:\n\
              Database connection from ./rustyroad.toml (or ./rustyroad.<ENVIRONMENT>.toml).\n\n\
             EXAMPLE:\n\
              rustyroad migration repair-ledger\n\
              ENVIRONMENT=prod rustyroad migration repair-ledger --yes\n",
        )
        .arg(
            Arg::new("yes")
                .long("yes")
                .short('y')
                .action(ArgAction::SetTrue)
                .help("Confirm deletion of superseded duplicate ledger rows"),
        )
        .after_help("Example:\n  rustyroad migration repair-ledger --yes\n")
}

/// Runs the confirmed transactional ledger repair.
pub(super) async fn run(matches: &ArgMatches) {
    super::print_config();
    if !confirmed(matches) {
        println!("Ledger repair cancelled. No changes were made.");
        return;
    }

    let connection = match baseline::connect().await {
        Ok(connection) => connection,
        Err(error) => return fail(&error.to_string()),
    };
    if let Err(error) = ledger::ensure_table(&connection).await {
        return fail(&error.to_string());
    }

    match ledger::repair(&connection).await {
        Ok(report) => println!(
            "Ledger repaired transactionally: {} rows before, {} retained, {} removed.",
            report.rows_before, report.rows_after, report.rows_removed
        ),
        Err(error) => fail(&error.to_string()),
    }
}

/// Requires an interactive confirmation unless `--yes` was supplied.
fn confirmed(matches: &ArgMatches) -> bool {
    if matches.get_flag("yes") {
        return true;
    }
    if !io::stdin().is_terminal() {
        eprintln!("Refusing ledger repair without confirmation. Re-run with --yes.");
        return false;
    }
    dialoguer::Confirm::new()
        .with_prompt("Delete superseded duplicate migration-ledger rows?")
        .default(false)
        .interact()
        .unwrap_or(false)
}

/// Reports a repair failure with a nonzero exit status.
fn fail(message: &str) {
    eprintln!("Ledger repair failed: {message}");
    std::process::exit(1);
}
