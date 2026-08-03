//! Clap definition for `rustyroad pull`.

use super::help;
use clap::{Arg, ArgAction, Command};

/// Returns the `pull` command.
pub(crate) fn pull() -> Command {
    Command::new("pull")
        .alias("introspect")
        .about("Introspect the database into a TypeScript folder")
        .long_about(help::long_about())
        .arg(
            Arg::new("out")
                .long("out")
                .short('o')
                .help("Output folder (default: ./db)"),
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
                .help("Identifier casing (default: camel)"),
        )
        .arg(
            Arg::new("schema-only")
                .long("schema-only")
                .action(ArgAction::SetTrue)
                .help("Emit only schema.ts and relations.ts"),
        )
        .arg(
            Arg::new("force")
                .long("force")
                .action(ArgAction::SetTrue)
                .help("Also overwrite files you own, such as api.ts"),
        )
}
