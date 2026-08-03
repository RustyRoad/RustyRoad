//! Clap definition for `rustyroad pull`.

use clap::{Arg, ArgAction, Command};

/// Returns the `pull` command.
pub(crate) fn pull() -> Command {
    Command::new("pull")
        .alias("introspect")
        .about("Introspect the database into a TypeScript folder")
        .long_about(
            "Reads the live database schema and writes generated TypeScript.\n\n\
             OUTPUT (default ./db):\n\
              schema.ts     Drizzle table definitions, constraints, and indexes\n\
              relations.ts  one/many relations derived from foreign keys\n\
              client.ts     inferred row types and a typed repository per table\n\
              api.ts        REST routers over those repositories\n\n\
             Existing files are overwritten, so keep hand-written code elsewhere.\n\n\
             POSTGRES ONLY:\n\
              Introspection reads the Postgres catalog. Other backends are not\n\
              supported by this command yet.\n\n\
             CONFIG:\n\
              Database connection from ./rustyroad.toml (or ./rustyroad.<ENVIRONMENT>.toml).\n\n\
             EXAMPLES:\n\
              rustyroad pull\n\
              rustyroad pull --out ./src/db --casing preserve\n\
              rustyroad pull --schema-only\n",
        )
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
}
