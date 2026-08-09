//! Printing the current lifecycle state.

use super::super::status::Status;
use crate::database::versions::naming;

/// Prints the current lifecycle state.
pub(in crate::database::migrations::cli::version) fn status(status: &Status) {
    line("Current version", status.current.as_deref(), status);
    line("In progress    ", status.active.as_deref(), status);

    if status.active.is_some() {
        println!("\nRun 'rustyroad migration complete' or 'rustyroad migration rollback-version'.");
    }
    if let Some(baseline) = &status.baseline {
        println!("Last baseline:   {baseline}");
    }
    println!(
        "\nVersion history is separate from _rustyroad_migrations bookkeeping. \
         Run 'rustyroad migration list' for ledger provenance; ledger rows do not \
         verify live effects."
    );
}

/// Prints one labelled version line, with its schema when versioned.
fn line(label: &str, version: Option<&str>, status: &Status) {
    match version {
        Some(version) => {
            println!("{label}: {version}");
            if status.versioned {
                println!(
                    "  schema: {}",
                    naming::versioned_schema(&status.schema, version)
                );
            }
        }
        None => println!("{label}: none"),
    }
}
