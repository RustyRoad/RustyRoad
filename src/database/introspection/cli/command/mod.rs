//! Clap definition for `rustyroad pull`.

mod args;

use super::help;
use clap::Command;

/// Returns the `pull` command.
pub(crate) fn pull() -> Command {
    let command = Command::new("pull")
        .alias("introspect")
        .about("Introspect the database into a TypeScript folder")
        .long_about(help::long_about());

    args::models(args::typescript(command))
}
