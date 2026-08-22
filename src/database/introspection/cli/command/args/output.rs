//! Arguments controlling the TypeScript output.

use super::{flag, value, with};
use clap::{Arg, Command};

/// Adds the arguments controlling the TypeScript output.
pub(in crate::database::introspection::cli::command) fn typescript(command: Command) -> Command {
    with(
        command,
        vec![
            Arg::new("out")
                .long("out")
                .short('o')
                .help("Output folder (default: ./db)"),
            Arg::new("language")
                .long("language")
                .value_parser(["typescript", "rust"])
                .help("Output language (default: typescript)"),
            value("schema", "Database schema to introspect (default: public)"),
            clap::Arg::new("tables")
                .long("tables")
                .action(clap::ArgAction::Append)
                .help("Only these tables; repeatable, trailing * matches a prefix"),
            clap::Arg::new("exclude")
                .long("exclude")
                .action(clap::ArgAction::Append)
                .help("Drop these tables; repeatable, trailing * matches a prefix"),
            Arg::new("casing")
                .long("casing")
                .value_parser(["camel", "preserve"])
                .help("Identifier casing (default: camel)"),
            flag("schema-only", "Emit only schema.ts and relations.ts"),
            flag("force", "Also overwrite files you own, such as api.ts"),
        ],
    )
}
