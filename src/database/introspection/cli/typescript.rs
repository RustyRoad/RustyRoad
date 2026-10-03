//! Writing the generated TypeScript folder.

use super::report;
use crate::database::introspection::Schema;
use crate::generators::typescript::{self, Casing, Outputs};
use clap::ArgMatches;
use std::path::PathBuf;

/// Default output folder, matching Drizzle's convention.
const DEFAULT_OUT: &str = "./db";

/// Writes the TypeScript folder, reporting the outcome.
///
/// Returns `false` when the write failed, so a caller can skip work that would otherwise
/// build on a half-written folder.
pub(super) fn write(matches: &ArgMatches, schema: &Schema) -> bool {
    let out = PathBuf::from(
        matches
            .get_one::<String>("out")
            .map(String::as_str)
            .unwrap_or(DEFAULT_OUT),
    );
    let casing = Casing::parse(matches.get_one::<String>("casing").map(String::as_str));
    let force = matches.get_flag("force");

    match typescript::write(&out, schema, casing, outputs(matches), force) {
        Ok(report_data) => {
            report::written(schema, &report_data);
            true
        }
        Err(error) => {
            report::fail(&error.to_string());
            false
        }
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
