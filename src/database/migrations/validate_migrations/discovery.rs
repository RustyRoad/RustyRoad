use super::error::MigrationValidationError;
use super::migration_directory::load;
use super::model::MigrationFile;
use std::fs;
use std::path::Path;

pub(super) fn discover(root: &Path) -> Result<Vec<MigrationFile>, MigrationValidationError> {
    if !root.is_dir() {
        return Err(MigrationValidationError::new(format!(
            "No migrations directory found at '{}'. Expected: ./config/database/migrations/<timestamp>-<name>/up.sql",
            root.display()
        )));
    }
    let mut entries = fs::read_dir(root)
        .map_err(|error| {
            MigrationValidationError::new(format!(
                "Could not read migrations directory '{}': {error}",
                root.display()
            ))
        })?
        .map(|entry| {
            entry.map_err(|error| {
                MigrationValidationError::new(format!(
                    "Could not inspect an entry in migrations directory '{}': {error}",
                    root.display()
                ))
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    entries.retain(|entry| entry.path().is_dir());
    entries.sort_by_key(|entry| entry.file_name());
    if entries.is_empty() {
        return Err(MigrationValidationError::new(format!(
            "No migrations found in '{}'. Validation requires at least one migration.",
            root.display()
        )));
    }
    entries.into_iter().map(load).collect()
}
