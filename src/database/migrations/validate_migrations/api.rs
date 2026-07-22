use super::backend;
use super::discovery::discover;
use super::error::MigrationValidationError;
use super::identity::Identity;
use super::model::MigrationValidationReport;
use crate::database::{get_config_file_name, Database};
use std::path::Path;

const MIGRATIONS_DIR: &str = "./config/database/migrations";

pub async fn validate_migrations() -> Result<MigrationValidationReport, MigrationValidationError> {
    let config_file = get_config_file_name();
    let database = Database::get_database_from_rustyroad_toml().map_err(|error| {
        MigrationValidationError::new(format!(
            "Could not load database configuration from '{config_file}': {error}"
        ))
    })?;
    validate_with_database(&database, Path::new(MIGRATIONS_DIR)).await
}

pub(super) async fn validate_with_database(
    database: &Database,
    directory: &Path,
) -> Result<MigrationValidationReport, MigrationValidationError> {
    let migrations = discover(directory)?;
    backend::validate(database, &Identity::generate(), &migrations).await?;
    Ok(MigrationValidationReport {
        database_type: database.database_type.clone(),
        migrations_validated: migrations.len(),
    })
}
