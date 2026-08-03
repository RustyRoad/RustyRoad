//! Writing generated TypeScript into the output folder.

mod outputs;

use super::casing::Casing;
use super::{client, fastify, relations, schema, zod};
use crate::database::introspection::Schema;
use std::path::{Path, PathBuf};
use std::{fs, io};

pub use outputs::Outputs;

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

    let files = [
        (outputs.schema, "schema.ts", schema::render(model, casing)),
        (
            outputs.relations,
            "relations.ts",
            relations::render(model, casing),
        ),
        (outputs.client, "client.ts", client::render(model, casing)),
        (outputs.api, "zod.ts", zod::render(model, casing)),
        (outputs.api, "routes.ts", fastify::render(model, casing)),
    ];

    let mut written = Vec::new();
    for (enabled, name, contents) in files {
        if !enabled {
            continue;
        }
        let path = out.join(name);
        fs::write(&path, contents)?;
        written.push(path);
    }

    Ok(written)
}
