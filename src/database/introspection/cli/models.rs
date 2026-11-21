//! Selecting and writing the generated model folders.
//!
//! Kept apart from the command's own flow because each generator needs its own default
//! path, output folder, and report; putting all three inline made `run` a list of
//! near-identical blocks.

use super::report;
use crate::database::introspection::{PartialUniqueIndex, Schema};
use crate::generators::tetherscript;
use crate::generators::typescript::{self, Casing, Outputs};
use clap::ArgMatches;
use std::path::PathBuf;

/// Default TetherScript model folder.
///
/// Outside `src/` because a `.tether` module is loaded at runtime by path rather than
/// compiled into the Rust binary.
const TETHER_OUT: &str = "./models";

/// Default folder for standalone TypeScript validation schemas.
const ZOD_OUT: &str = "./src/schemas";

/// Writes the requested model languages, preserving their output settings.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn write(matches: &ArgMatches, schema: &Schema, partial_keys: &[PartialUniqueIndex]) {
    let force = matches.get_flag("force");

    if matches.get_one::<String>("language").map(String::as_str) == Some("rust") {
        super::rust_models::write(matches, schema, force, partial_keys);
        return;
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
/// # Examples
/// ```sh
/// rustyroad pull --zod
/// ```
fn folder(matches: &ArgMatches, flag: &str, default: &str) -> PathBuf {
    PathBuf::from(
        matches
            .get_one::<String>(flag)
            .map(String::as_str)
            .unwrap_or(default),
    )
}
