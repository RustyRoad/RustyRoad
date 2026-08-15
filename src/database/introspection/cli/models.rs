//! Selecting and writing the generated model folders.
//!
//! Kept apart from the command's own flow because each generator needs its own default
//! path, output folder, and report; putting all three inline made `run` a list of
//! near-identical blocks.

use super::report;
use crate::database::introspection::Schema;
use crate::generators::typescript::{self, Casing, Outputs};
use crate::generators::{rust, tetherscript};
use clap::ArgMatches;
use std::path::PathBuf;

/// Default Rust model folder, matching where a project's models already live.
const RUST_OUT: &str = "./src/models";

/// Default TetherScript model folder.
///
/// Outside `src/` because a `.tether` module is loaded at runtime by path rather than
/// compiled into the Rust binary.
const TETHER_OUT: &str = "./models";

/// Default folder for standalone TypeScript validation schemas.
const ZOD_OUT: &str = "./src/schemas";

/// Writes whichever model languages were asked for.
pub(super) fn write(matches: &ArgMatches, schema: &Schema) {
    let force = matches.get_flag("force");

    if matches.get_flag("models") {
        let out = folder(matches, "models-out", RUST_OUT);
        let options = rust::Options {
            actix: matches.get_flag("actix"),
        };

        match rust::write_with(&out, schema, options, force) {
            Ok(layout) => report::models("Rust models", &layout),
            Err(error) => report::fail(&error.to_string()),
        }
    }

    if matches.get_flag("tether-models") {
        let out = folder(matches, "tether-models-out", TETHER_OUT);
        match tetherscript::write(&out, schema, force) {
            Ok(layout) => report::models("TetherScript models", &layout),
            Err(error) => report::fail(&error.to_string()),
        }
    }

    if matches.get_flag("zod") {
        let out = folder(matches, "zod-out", ZOD_OUT);
        let casing = Casing::parse(matches.get_one::<String>("casing").map(String::as_str));

        match typescript::write(&out, schema, casing, Outputs::zod_only(), force) {
            Ok(generated) => report::artifacts("Zod schemas", &generated.outcomes),
            Err(error) => report::fail(&error.to_string()),
        }
    }
}

/// Returns the output folder for one generator, falling back to its default.
fn folder(matches: &ArgMatches, flag: &str, default: &str) -> PathBuf {
    PathBuf::from(
        matches
            .get_one::<String>(flag)
            .map(String::as_str)
            .unwrap_or(default),
    )
}
