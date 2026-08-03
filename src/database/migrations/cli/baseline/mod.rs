//! `rustyroad migration baseline` — adopt the current schema as already migrated.

mod command;
mod prompt;

pub(crate) use command::baseline as baseline_command;

use crate::database::migrations::baseline;
use clap::ArgMatches;

pub(super) async fn run(matches: &ArgMatches) {
    super::print_config();

    let identities = match baseline::identities() {
        Ok(identities) => identities,
        Err(error) => return fail(&error.to_string()),
    };
    if identities.is_empty() {
        println!("No migrations found. Nothing to baseline.");
        return;
    }

    if !prompt::confirmed(matches, identities.len()) {
        println!("Baseline cancelled. No changes were made.");
        return;
    }

    let connection = match baseline::connect().await {
        Ok(connection) => connection,
        Err(error) => return fail(&error.to_string()),
    };

    match baseline::create(&connection).await {
        Ok(recorded) => prompt::report(&recorded, identities.len()),
        Err(error) => fail(&error.to_string()),
    }
}

/// Reports a failure and exits nonzero so callers and CI can detect it.
fn fail(message: &str) {
    eprintln!("Baseline failed: {message}");
    std::process::exit(1);
}
