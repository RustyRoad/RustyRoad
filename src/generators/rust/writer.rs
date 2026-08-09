//! Writes generated Rust API artifacts while preserving composition files.

use super::{models, procedures, repositories};
use crate::database::introspection::Schema;
use crate::generators::report::{Outcome, Report};
use crate::generators::typescript::{heyapi, Casing};
use std::path::Path;
use std::{fs, io};

/// Which Rust artifacts `pull` emits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Outputs {
    models: bool,
    api: bool,
    sdk: bool,
}

impl Outputs {
    /// Models, repositories, procedures, composition, and OpenAPI client input.
    pub fn all() -> Self {
        Self {
            models: true,
            api: true,
            sdk: true,
        }
    }

    /// Only database row and input models.
    pub fn schema_only() -> Self {
        Self {
            models: true,
            api: false,
            sdk: false,
        }
    }
}

/// Writes a Rust database module into `out`.
pub fn write(out: &Path, schema: &Schema, outputs: Outputs, force: bool) -> io::Result<Report> {
    fs::create_dir_all(out)?;
    let mut outcomes = Vec::new();

    if outputs.models {
        outcomes.push(write_owned(
            out,
            "models.rs",
            &models::render(schema),
            Ownership::Generated,
            force,
        )?);
    }
    if outputs.api {
        for (name, contents, ownership) in [
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
        ] {
            outcomes.push(write_owned(out, name, &contents, ownership, force)?);
        }
    }
    if outputs.sdk {
        outcomes.extend(write_openapi(out, schema, force)?);
    }

    Ok(Report {
        outcomes,
        // api.rs calls one stable configure_generated function, so new tables are
        // automatically reachable without rewriting the developer-owned file.
        unwired: Vec::new(),
    })
}

#[derive(Clone, Copy)]
enum Ownership {
    Generated,
    Scaffold,
}

fn write_owned(
    directory: &Path,
    name: &str,
    contents: &str,
    ownership: Ownership,
    force: bool,
) -> io::Result<Outcome> {
    let path = directory.join(name);
    if matches!(ownership, Ownership::Scaffold) && path.exists() && !force {
        return Ok(Outcome::Preserved(path));
    }
    fs::write(&path, contents)?;
    Ok(Outcome::Written(path))
}

fn write_openapi(out: &Path, schema: &Schema, force: bool) -> io::Result<Vec<Outcome>> {
    let directory = out.join("openapi");
    fs::create_dir_all(&directory)?;
    heyapi::render(schema, Casing::Preserve, procedures::ROUTE_PREFIX)
        .into_iter()
        .map(|file| {
            let ownership = if file.name == "openapi-ts.config.ts" {
                Ownership::Scaffold
            } else {
                Ownership::Generated
            };
            write_owned(&directory, file.name, &file.contents, ownership, force)
        })
        .collect()
}
