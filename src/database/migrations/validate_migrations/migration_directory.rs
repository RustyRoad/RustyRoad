use super::error::MigrationValidationError;
use super::model::MigrationFile;
use std::fs::DirEntry;

pub(super) fn load(directory: DirEntry) -> Result<MigrationFile, MigrationValidationError> {
    let name = directory
        .file_name()
        .into_string()
        .map_err(|_| MigrationValidationError::new("Invalid migration directory name"))?;
    validate_name(&name)?;
    let up_sql_path = directory.path().join("up.sql");
    if !up_sql_path.is_file() {
        return Err(MigrationValidationError::new(format!(
            "Migration '{name}' is missing '{}'. Every migration must contain up.sql.",
            up_sql_path.display()
        )));
    }
    Ok(MigrationFile { name, up_sql_path })
}

fn validate_name(name: &str) -> Result<(), MigrationValidationError> {
    let (timestamp, migration_name) = name.split_once('-').ok_or_else(|| invalid(name))?;
    if timestamp.is_empty()
        || !timestamp
            .chars()
            .all(|character| character.is_ascii_digit())
        || migration_name.is_empty()
    {
        return Err(invalid(name));
    }
    Ok(())
}

fn invalid(name: &str) -> MigrationValidationError {
    MigrationValidationError::new(format!(
        "Invalid migration directory '{name}'. Expected: <timestamp>-<name>"
    ))
}
