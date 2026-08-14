//! Arguments selecting the model languages.

use super::{flag, value, with};
use clap::Command;

/// Adds the arguments selecting the model languages.
pub(in crate::database::introspection::cli::command) fn models(command: Command) -> Command {
    with(
        command,
        vec![
            flag("models", "Also emit Rust sqlx models"),
            value("models-out", "Rust model folder (default: ./src/models)"),
            flag(
                "actix",
                "With --models, also emit actix-web handlers and utoipa annotations",
            ),
            flag("tether-models", "Also emit TetherScript models"),
            value(
                "tether-models-out",
                "TetherScript model folder (default: ./models)",
            ),
        ],
    )
}
