//! Clap definition for the `db` command group.

use clap::{Arg, ArgAction, Command};

/// Returns the `db` command group.
pub(crate) fn db_command() -> Command {
    Command::new("db")
        .about("Database operations")
        .subcommand(
            Command::new("schema")
                .about("Inspect database schema")
                .arg(Arg::new("table").value_name("TABLE").help("Inspect only this table"))
                .arg(
                    Arg::new("interactive")
                        .long("interactive")
                        .short('i')
                        .action(ArgAction::SetTrue)
                        .conflicts_with("table")
                        .help("Choose a table with live prefix suggestions and Tab completion"),
                )
                .after_help("Examples:\n  rustyroad db schema users\n  rustyroad db schema --interactive\n  rustyroad --format json db schema users\n\nInteractive mode loads table names once. Type a prefix, Tab to complete, Enter\nto inspect. Ctrl-C/Ctrl-D cancels. Requires a terminal and text output.")
                .long_about(
                    "Lists tables and their columns from the connected database.\n\
                     Supply TABLE to inspect one table, or --interactive to choose one.\n\n\
                     CONFIG:\n\
                     Reads from ./rustyroad.toml by default.\n\
                     Set ENVIRONMENT=<env> to use ./rustyroad.<env>.toml instead.\n\n\
                     PREREQUISITES:\n\
                     - Must be run from your RustyRoad project root\n\
                     - Database must be reachable\n\n\
                     Supports: PostgreSQL, MySQL, SQLite\n\n\
                     EXAMPLE:\n\
                     rustyroad db schema\n\
                     ENVIRONMENT=prod rustyroad db schema\n",
                ),
        )
        .subcommand(
            Command::new("enums")
                .about("Inspect database enum types")
                .long_about(
                    "Lists the user-defined enum types and their allowed values.\n\n\
                     CONFIG:\n\
                     Reads from ./rustyroad.toml by default.\n\
                     Set ENVIRONMENT=<env> to use ./rustyroad.<env>.toml instead.\n\n\
                     PREREQUISITES:\n\
                     - Must be run from your RustyRoad project root\n\
                     - Database must be reachable\n\n\
                     BACKENDS:\n\
                     PostgreSQL: lists every enum type in the public schema with\n\
                     its values in declaration order.\n\
                     MySQL: lists enum columns, since MySQL keeps enum values on\n\
                     the columns instead of the type catalog.\n\
                     SQLite: has no enum type and reports an empty list.\n\n\
                     EXAMPLE:\n\
                     rustyroad db enums\n\
                     ENVIRONMENT=prod rustyroad db enums\n",
                ),
        )
        .subcommand(
            Command::new("tables")
                .about("List table names for discovery or completion")
                .arg(Arg::new("prefix").long("prefix").value_name("PREFIX")
                    .help("Filter by a literal, case-sensitive table-name prefix"))
                .after_help("Agent workflow:\n  rustyroad --format json db tables --prefix us\n  rustyroad --format json db schema users\n\nUses PostgreSQL public, the configured MySQL database, or SQLite main."),
        )
        .subcommand_required(true)
        .arg_required_else_help(true)
}
