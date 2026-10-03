//! Clap definition for the `db` command group.

use clap::Command;

/// Returns the `db` command group.
pub(crate) fn db_command() -> Command {
    Command::new("db")
        .about("Database operations")
        .subcommand(
            Command::new("schema")
                .about("Inspect database schema")
                .long_about(
                    "Lists all tables and their columns from the connected database.\n\n\
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
        .subcommand_required(true)
        .arg_required_else_help(true)
}
