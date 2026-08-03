//! Writing generated TypeScript into the output folder.

mod files;
mod outputs;

use super::casing::Casing;
use super::heyapi;
use crate::database::introspection::Schema;
use std::path::{Path, PathBuf};
use std::{fs, io};

pub use files::ROUTE_PREFIX;
pub use outputs::Outputs;

/// Subfolder holding the OpenAPI document and Hey API config.
const CLIENT_DIR: &str = "openapi";

/// Writes the selected artifacts into `out`, creating it when absent.
///
/// Returns the paths written, in emission order. Existing files are overwritten,
/// so hand-written code must live elsewhere.
pub fn write(
    out: &Path,
    model: &Schema,
    casing: Casing,
    outputs: Outputs,
) -> io::Result<Vec<PathBuf>> {
    fs::create_dir_all(out)?;
    let mut written = Vec::new();

    for (enabled, name, contents) in files::server(model, casing, outputs) {
        if enabled {
            written.push(write_file(out, name, &contents)?);
        }
    }

    if outputs.sdk {
        written.extend(client_files(out, model, casing)?);
    }

    Ok(written)
}

/// Writes the OpenAPI document and Hey API config.
///
/// The document is also generated at runtime by `openapi.ts` from the router; this
/// static copy exists so a client can be generated without running TypeScript.
fn client_files(
    out: &Path,
    model: &Schema,
    casing: Casing,
) -> io::Result<Vec<PathBuf>> {
    let directory = out.join(CLIENT_DIR);
    fs::create_dir_all(&directory)?;

    let mut written = Vec::new();
    for file in heyapi::render(model, casing, ROUTE_PREFIX) {
        written.push(write_file(&directory, file.name, &file.contents)?);
    }
    Ok(written)
}

/// Writes one file and returns its path.
fn write_file(directory: &Path, name: &str, contents: &str) -> io::Result<PathBuf> {
    let path = directory.join(name);
    fs::write(&path, contents)?;
    Ok(path)
}
