//! Arguments selecting the model languages.

use super::{flag, value, with};
use clap::{Arg, Command};

/// Adds the arguments selecting the model languages.
pub(in crate::database::introspection::cli::command) fn models(command: Command) -> Command {
    with(
        command,
        vec![
            flag("tether-models", "Also emit TetherScript models"),
            value(
                "tether-models-out",
                "TetherScript model folder (default: ./models)",
            ),
            Arg::new("zod")
                .long("zod")
                .alias("zod-models")
                .action(clap::ArgAction::SetTrue)
                .help("Also emit standalone Drizzle-backed Zod schemas"),
            Arg::new("zod-out")
                .long("zod-out")
                .alias("zod-models-out")
                .help("Standalone Zod schema folder (default: ./src/schemas)"),
        ],
    )
}
