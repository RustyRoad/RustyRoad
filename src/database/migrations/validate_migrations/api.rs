use super::backend;
use super::discovery::discover;
use super::environment::require_test;
use super::error::MigrationValidationError;
use super::identity::Identity;
use super::model::MigrationValidationReport;
use crate::database::{get_environment, Database};
use std::path::Path;

const MIGRATIONS_DIR: &str = "./config/database/migrations";

pub async fn validate_migrations() -> Result<MigrationValidationReport, MigrationValidationError> {
    require_test(&get_environment())?;
    let database = Database::get_database_from_rustyroad_toml().map_err(|error| {
        MigrationValidationError::new(format!(
            "Could not load test database configuration from 'rustyroad.test.toml': {error}"
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
