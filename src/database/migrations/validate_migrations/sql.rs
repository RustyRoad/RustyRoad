use super::error::MigrationValidationError;
use super::model::MigrationFile;
use std::fs;

pub(super) fn read(migration: &MigrationFile) -> Result<String, MigrationValidationError> {
    fs::read_to_string(&migration.up_sql_path).map_err(|error| {
        MigrationValidationError::new(format!(
            "Could not read migration '{}' from '{}': {error}",
            migration.name,
            migration.up_sql_path.display()
        ))
    })
}

pub(super) fn execution_error(
    migration: &MigrationFile,
    database: &str,
    error: sqlx::Error,
) -> MigrationValidationError {
    MigrationValidationError::new(format!(
        "Migration '{}' failed against the isolated {database} validation database: {error}",
        migration.name
    ))
}
