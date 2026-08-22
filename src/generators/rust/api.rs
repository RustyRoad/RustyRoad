//! Writing the canonical flat Rust API folder.

use super::{models, procedures, repositories};
use crate::database::introspection::Schema;
use crate::generators::ownership::{write_owned, Ownership};
use crate::generators::report::Outcome;
use std::path::Path;
use std::{fs, io};

/// Writes the canonical `src/db` Rust API output.
pub fn write(out: &Path, schema: &Schema, force: bool) -> io::Result<Vec<Outcome>> {
    fs::create_dir_all(out)?;

    let files = [
        ("models.rs", models::render(schema), Ownership::Generated),
        (
            "repositories.rs",
            repositories::render(schema),
            Ownership::Generated,
        ),
        (
            "procedures.rs",
            procedures::render(schema),
            Ownership::Generated,
        ),
        ("api.rs", procedures::composition(), Ownership::Scaffold),
        ("mod.rs", procedures::module(), Ownership::Scaffold),
    ];

    files
        .into_iter()
        .map(|(name, contents, ownership)| write_owned(out, name, &contents, ownership, force))
        .collect()
}
