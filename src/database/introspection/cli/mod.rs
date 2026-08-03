//! `rustyroad pull` — introspect the database into a TypeScript folder.
//!
//! Mirrors `drizzle-kit pull`: read the live schema, then write a folder of
//! generated TypeScript. RustyRoad additionally emits a typed client and REST
//! routes, which Drizzle leaves to the application.

mod command;
mod report;

use crate::database::introspection;
use crate::database::migrations::baseline;
use crate::generators::typescript::{self, Casing, Outputs};
use clap::ArgMatches;
use std::path::PathBuf;

pub(crate) use command::pull as pull_command;

/// Default output folder, matching Drizzle's convention.
const DEFAULT_OUT: &str = "./db";

/// Schema introspected when none is given.
const DEFAULT_SCHEMA: &str = "public";

pub async fn run(matches: &ArgMatches) {
    let out = PathBuf::from(
        matches
            .get_one::<String>("out")
            .map(String::as_str)
            .unwrap_or(DEFAULT_OUT),
    );
    let schema_name = matches
        .get_one::<String>("schema")
        .map(String::as_str)
        .unwrap_or(DEFAULT_SCHEMA);
    let casing = Casing::parse(matches.get_one::<String>("casing").map(String::as_str));
    let outputs = outputs(matches);

    let connection = match baseline::connect().await {
        Ok(connection) => connection,
        Err(error) => return report::fail(&error.to_string()),
    };

    let schema = match introspection::read(&connection, schema_name).await {
        Ok(schema) => schema,
        Err(error) => return report::fail(&error.to_string()),
    };

    if schema.tables.is_empty() {
        return report::empty(schema_name);
    }

    match typescript::write(&out, &schema, casing, outputs) {
        Ok(written) => report::written(&schema, &written),
        Err(error) => report::fail(&error.to_string()),
    }
}

/// Returns which artifacts to emit, honouring `--schema-only`.
fn outputs(matches: &ArgMatches) -> Outputs {
    if matches.get_flag("schema-only") {
        Outputs::schema_only()
    } else {
        Outputs::all()
    }
}
