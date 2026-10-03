//! Writing generated TypeScript into the output folder.

mod files;
mod groups;
mod outputs;
mod report;
mod wiring;

use super::casing::Casing;
use super::heyapi;
use crate::database::introspection::Schema;
use std::path::Path;
use std::{fs, io};

pub use crate::generators::ownership::Outcome;
pub use files::ROUTE_PREFIX;
pub use outputs::Outputs;
pub use report::Report;

/// Subfolder holding the OpenAPI document and Hey API config.
const CLIENT_DIR: &str = "openapi";

/// The composition file, which the developer owns after the first run.
const COMPOSITION: &str = "api.ts";

/// Writes the selected artifacts into `out`, creating it when absent.
///
/// Files derived from the database are always rewritten; the composition file is
/// written only when absent, so hand-written procedures survive. Pass `force` to
/// overwrite it too.
pub fn write(
    out: &Path,
    model: &Schema,
    casing: Casing,
    outputs: Outputs,
    force: bool,
) -> io::Result<Report> {
    fs::create_dir_all(out)?;

    let mut outcomes = groups::server(out, model, casing, outputs, force)?;
    if outputs.sdk {
        outcomes.extend(groups::client(out, model, casing)?);
    }

    Ok(Report {
        unwired: unwired(out, model, casing, &outcomes),
        outcomes,
    })
}

/// Returns the tables a preserved composition file does not serve.
///
/// A freshly written composition lists every table, so only a preserved one can be
/// out of date.
fn unwired(out: &Path, model: &Schema, casing: Casing, outcomes: &[Outcome]) -> Vec<String> {
    let preserved = outcomes
        .iter()
        .any(|outcome| outcome.is_preserved() && outcome.path().ends_with(COMPOSITION));

    if preserved {
        wiring::unwired(&out.join(COMPOSITION), model, casing)
    } else {
        Vec::new()
    }
}
