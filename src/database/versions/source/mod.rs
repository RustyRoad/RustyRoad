//! Loading a migration's definition, declarative or raw SQL.

mod raw;

use super::ops::model::{Migration, Operation, RawSql};
use crate::database::migrations::CustomMigrationError;
use std::path::{Path, PathBuf};
use std::{fs, io};

pub use raw::{down, split, up, MIGRATIONS_DIR};

/// Declarative filenames, preferred over raw SQL when present.
const DECLARATIVE: &[&str] = &["migration.json", "migration.yaml", "migration.yml"];

/// Returns the migration definition for `version`.
///
/// A declarative `migration.json` takes precedence; otherwise `up.sql` is wrapped as
/// raw operations so both authoring styles share one execution path.
pub fn load(version: &str) -> Result<Migration, CustomMigrationError> {
    let directory = Path::new(MIGRATIONS_DIR).join(version);

    if let Some(path) = declarative_path(&directory) {
        return parse(&path, version);
    }

    Ok(Migration {
        name: version.to_string(),
        operations: up(version)?
            .into_iter()
            .map(|sql| {
                Operation::Sql(RawSql {
                    up: sql,
                    down: None,
                })
            })
            .collect(),
    })
}

/// Returns the declarative definition path, when one exists.
fn declarative_path(directory: &Path) -> Option<PathBuf> {
    DECLARATIVE
        .iter()
        .map(|name| directory.join(name))
        .find(|path| path.is_file())
}

/// Parses a declarative migration file.
///
/// JSON is a YAML subset, so a JSON-shaped document in a `.yaml` file parses too;
/// anything else is reported rather than silently ignored.
fn parse(path: &Path, version: &str) -> Result<Migration, CustomMigrationError> {
    let text = fs::read_to_string(path).map_err(|error| {
        CustomMigrationError::IoError(io::Error::new(
            error.kind(),
            format!("Could not read '{}': {error}", path.display()),
        ))
    })?;

    let mut migration: Migration = serde_json::from_str(&text).map_err(|error| {
        CustomMigrationError::IoError(io::Error::other(format!(
            "Could not parse '{}'. Expected a JSON-compatible document. Details: {error}",
            path.display()
        )))
    })?;

    if migration.name.is_empty() {
        migration.name = version.to_string();
    }
    Ok(migration)
}
