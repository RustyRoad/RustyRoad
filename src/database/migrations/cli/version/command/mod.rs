//! Clap definitions for the versioned migration lifecycle commands.

mod text;

use clap::{Arg, Command};

/// Returns the `start` subcommand.
pub(crate) fn start() -> Command {
    Command::new("start")
        .about("Start a migration, serving old and new schema versions together")
        .long_about(text::START)
        .arg(
            Arg::new("name")
                .required(true)
                .help("Migration directory name (<timestamp>-<name>)"),
        )
}

/// Returns the `complete` subcommand.
pub(crate) fn complete() -> Command {
    Command::new("complete")
        .about("Complete the in-progress migration and retire the old version")
        .long_about(text::COMPLETE)
}

/// Returns the `rollback-version` subcommand.
pub(crate) fn rollback() -> Command {
    Command::new("rollback-version")
        .about("Undo the in-progress migration, keeping the previous version")
        .long_about(text::ROLLBACK)
}

/// Returns the `version-status` subcommand.
pub(crate) fn status() -> Command {
    Command::new("version-status")
        .about("Show the current schema version and any migration in progress")
        .long_about(text::STATUS)
}
