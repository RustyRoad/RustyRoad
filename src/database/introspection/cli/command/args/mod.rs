//! The command's arguments, grouped by what they control.

mod models;
mod output;

use clap::Command;

pub(super) use models::models;
pub(super) use output::typescript;

/// Adds a boolean flag.
fn flag(name: &'static str, help: &'static str) -> clap::Arg {
    clap::Arg::new(name)
        .long(name)
        .action(clap::ArgAction::SetTrue)
        .help(help)
}

/// Adds an argument taking a value.
fn value(name: &'static str, help: &'static str) -> clap::Arg {
    clap::Arg::new(name).long(name).help(help)
}

/// Applies a list of arguments to a command.
fn with(command: Command, args: Vec<clap::Arg>) -> Command {
    args.into_iter().fold(command, Command::arg)
}
