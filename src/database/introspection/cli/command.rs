//! Clap definition for `rustyroad pull`.

use super::help;
use clap::{Arg, ArgAction, Command};

/// Returns the `pull` command.
pub(crate) fn pull() -> Command {
    Command::new("pull")
        .alias("introspect")
        .about("Introspect the database into a TypeScript or Rust API")
        .long_about(help::long_about())
        .arg(
            Arg::new("language")
                .long("language")
                .short('l')
                .value_parser(["typescript", "rust"])
                .default_value("typescript")
                .help("Generated server language (default: typescript)"),
        )
        .arg(
            Arg::new("out")
                .long("out")
                .short('o')
                .help("Output folder (defaults: ./db for TypeScript, ./src/db for Rust)"),
        )
        .arg(
            Arg::new("schema")
                .long("schema")
                .help("Database schema to introspect (default: public)"),
        )
        .arg(
            Arg::new("casing")
                .long("casing")
                .value_parser(["camel", "preserve"])
                .help("TypeScript identifier casing (default: camel)"),
        )
        .arg(
            Arg::new("schema-only")
                .long("schema-only")
                .action(ArgAction::SetTrue)
                .help("Emit only database schema/model files"),
        )
        .arg(
            Arg::new("force")
                .long("force")
                .action(ArgAction::SetTrue)
                .help("Also overwrite files you own, such as api.ts or api.rs"),
        )
}
