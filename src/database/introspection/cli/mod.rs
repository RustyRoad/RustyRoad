//! `rustyroad pull` — introspect the database into a TypeScript folder.
//!
//! Mirrors `drizzle-kit pull`: read the live schema, then write a folder of
//! generated TypeScript. RustyRoad additionally emits a typed client and REST
//! routes, which Drizzle leaves to the application.

mod command;
mod help;
mod notices;
mod report;

use crate::database::introspection;
use crate::database::migrations::baseline;
use crate::generators::rust as rust_generator;
use crate::generators::typescript::{self, Casing, Outputs};
use clap::ArgMatches;
use std::path::PathBuf;

pub(crate) use command::pull as pull_command;

const TYPESCRIPT_OUT: &str = "./db";
const RUST_OUT: &str = "./src/db";

/// Schema introspected when none is given.
const DEFAULT_SCHEMA: &str = "public";

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Language {
    TypeScript,
    Rust,
}

pub async fn run(matches: &ArgMatches) {
    let language = match matches.get_one::<String>("language").map(String::as_str) {
        Some("rust") => Language::Rust,
        _ => Language::TypeScript,
    };
    let default_out = match language {
        Language::TypeScript => TYPESCRIPT_OUT,
        Language::Rust => RUST_OUT,
    };
    let out = PathBuf::from(
        matches
            .get_one::<String>("out")
            .map(String::as_str)
            .unwrap_or(default_out),
    );
    let schema_name = matches
        .get_one::<String>("schema")
        .map(String::as_str)
        .unwrap_or(DEFAULT_SCHEMA);
    let casing = Casing::parse(matches.get_one::<String>("casing").map(String::as_str));
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

    let force = matches.get_flag("force");
    let result = match language {
        Language::TypeScript => {
            typescript::write(&out, &schema, casing, typescript_outputs(matches), force)
        }
        Language::Rust => rust_generator::write(&out, &schema, rust_outputs(matches), force),
    };
    match result {
        Ok(report_data) => report::written(&schema, &report_data, language),
        Err(error) => report::fail(&error.to_string()),
    }
}

/// Returns which artifacts to emit, honouring `--schema-only`.
fn typescript_outputs(matches: &ArgMatches) -> Outputs {
    if matches.get_flag("schema-only") {
        Outputs::schema_only()
    } else {
        Outputs::all()
    }
}

fn rust_outputs(matches: &ArgMatches) -> rust_generator::Outputs {
    if matches.get_flag("schema-only") {
        rust_generator::Outputs::schema_only()
    } else {
        rust_generator::Outputs::all()
    }
}
