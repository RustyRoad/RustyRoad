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
mod rust_models;
mod typescript;

use crate::database::introspection;
use crate::database::migrations::baseline;
use clap::ArgMatches;

pub(crate) use command::pull as pull_command;

/// Schema introspected when none is given.
const DEFAULT_SCHEMA: &str = "public";

/// Writes schema-derived artifacts, selecting Rust-specific conflict metadata.
/// Errors are reported by the owning introspection or output stage.
///
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub async fn run(matches: &ArgMatches) {
    let schema_name = matches
        .get_one::<String>("schema")
        .map(String::as_str)
        .unwrap_or(DEFAULT_SCHEMA);

    let connection = match baseline::connect().await {
        Ok(connection) => connection,
        Err(error) => return report::fail(&error.to_string()),
    };

    let result = if matches.get_one::<String>("language").map(String::as_str) == Some("rust") {
        introspection::read_for_rust(&connection, schema_name).await
    } else {
        introspection::read(&connection, schema_name).await
    };
    let schema = match result {
        Ok(schema) => filter::apply(matches, schema),
        Err(error) => return report::fail(&error.to_string()),
    };

    if schema.tables.is_empty() {
        if matches.get_one::<String>("language").map(String::as_str) == Some("rust") {
            if let Err(error) = rust_models::check_empty() {
                return report::fail(&error);
            }
        }
        return report::empty(schema_name);
    }

    if matches.get_one::<String>("language").map(String::as_str) == Some("rust") {
        let partial_keys =
            match introspection::read_partial_unique_indexes(&connection, schema_name).await {
                Ok(keys) => keys
                    .into_iter()
                    .filter(|key| schema.table(&key.table).is_some())
                    .collect::<Vec<_>>(),
                Err(error) => return report::fail(&error.to_string()),
            };
        models::write(matches, &schema, &partial_keys);
        return;
    }

    // Models are skipped when the TypeScript write failed, rather than layered onto a
    // half-written output folder.
    if typescript::write(matches, &schema) {
        models::write(matches, &schema, &[]);
    }
}
