//! `rustyroad pull` — introspect the database into generated code.
//!
//! Mirrors `drizzle-kit pull`: read the live schema, then write a folder of generated
//! TypeScript. RustyRoad additionally emits a typed client and REST routes, which Drizzle
//! leaves to the application, and can emit Rust or TetherScript models from the same
//! introspection.

mod command;
mod filter;
#[cfg(test)]
mod filter_fixture;
#[cfg(test)]
mod filter_test;
mod help;
mod models;
mod notices;
mod report;
mod typescript;

use crate::database::introspection;
use crate::database::migrations::baseline;
use clap::ArgMatches;

pub(crate) use command::pull as pull_command;

/// Schema introspected when none is given.
const DEFAULT_SCHEMA: &str = "public";

pub async fn run(matches: &ArgMatches) {
    let schema_name = matches
        .get_one::<String>("schema")
        .map(String::as_str)
        .unwrap_or(DEFAULT_SCHEMA);

    let connection = match baseline::connect().await {
        Ok(connection) => connection,
        Err(error) => return report::fail(&error.to_string()),
    };

    let schema = match introspection::read(&connection, schema_name).await {
        Ok(schema) => filter::apply(matches, schema),
        Err(error) => return report::fail(&error.to_string()),
    };

    if schema.tables.is_empty() {
        return report::empty(schema_name);
    }

    if matches.get_one::<String>("language").map(String::as_str) == Some("rust") {
        models::write(matches, &schema);
        return;
    }

    // Models are skipped when the TypeScript write failed, rather than layered onto a
    // half-written output folder.
    if typescript::write(matches, &schema) {
        models::write(matches, &schema);
    }
}
