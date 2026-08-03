//! Reads a migration's up/down SQL from disk.

use crate::database::migrations::CustomMigrationError;
use crate::database::statement::is_multi_statement;
use std::path::Path;
use std::{fs, io};

/// Directory holding migration folders.
pub const MIGRATIONS_DIR: &str = "./config/database/migrations";

/// Returns the statements in a migration's `up.sql`.
pub fn up(version: &str) -> Result<Vec<String>, CustomMigrationError> {
    read(version, "up.sql")
}

/// Returns the statements in a migration's `down.sql`.
pub fn down(version: &str) -> Result<Vec<String>, CustomMigrationError> {
    read(version, "down.sql")
}

/// Reads one migration file, split into individual statements.
fn read(version: &str, file: &str) -> Result<Vec<String>, CustomMigrationError> {
    let path = Path::new(MIGRATIONS_DIR).join(version).join(file);
    let sql = fs::read_to_string(&path).map_err(|error| {
        CustomMigrationError::IoError(io::Error::new(
            error.kind(),
            format!("Could not read '{}': {error}", path.display()),
        ))
    })?;

    Ok(split(&sql))
}

/// Splits a script into statements, preserving single statements verbatim.
///
/// Statements are applied individually so that a versioned schema is published
/// only after every statement has succeeded.
pub fn split(sql: &str) -> Vec<String> {
    if !is_multi_statement(sql) {
        let trimmed = sql.trim().trim_end_matches(';').trim();
        return if trimmed.is_empty() {
            Vec::new()
        } else {
            vec![trimmed.to_string()]
        };
    }

    sql.split(';')
        .map(str::trim)
        .filter(|statement| !statement.is_empty())
        .map(str::to_string)
        .collect()
}
