//! Grouping the emitted files by ownership.

use super::emit::write_owned;
use super::files;
use super::heyapi;
use super::outputs::Outputs;
use super::ownership::{Outcome, Ownership};
use super::{CLIENT_DIR, ROUTE_PREFIX};
use crate::database::introspection::Schema;
use crate::generators::typescript::casing::Casing;
use std::path::Path;
use std::{fs, io};

/// Writes the server-side files.
pub(super) fn server(
    out: &Path,
    model: &Schema,
    casing: Casing,
    outputs: Outputs,
    force: bool,
) -> io::Result<Vec<Outcome>> {
    files::server(model, casing, outputs)
        .into_iter()
        .filter(|emit| emit.enabled)
        .map(|emit| write_owned(out, emit.name, &emit.contents, emit.ownership, force))
        .collect()
}

/// Writes the OpenAPI document and Hey API config.
pub(super) fn client(out: &Path, model: &Schema, casing: Casing) -> io::Result<Vec<Outcome>> {
    let directory = out.join(CLIENT_DIR);
    fs::create_dir_all(&directory)?;

    heyapi::render(model, casing, ROUTE_PREFIX)
        .into_iter()
        .map(|file| {
            write_owned(
                &directory,
                file.name,
                &file.contents,
                client_ownership(file.name),
                false,
            )
        })
        .collect()
}

/// Returns who owns one of the client files.
///
/// The config is a scaffold: a project will edit its `output` path, and losing that
/// on every run would be worse than leaving it slightly stale.
fn client_ownership(name: &str) -> Ownership {
    if name == "openapi-ts.config.ts" {
        Ownership::Scaffold
    } else {
        Ownership::Generated
    }
}
