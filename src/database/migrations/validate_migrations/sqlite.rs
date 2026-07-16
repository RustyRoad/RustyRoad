use super::error::MigrationValidationError;
use super::execute_sqlite;
use super::guard;
use super::identity::Identity;
use super::model::MigrationFile;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};

pub(super) async fn validate(
    id: &Identity,
    migrations: &[MigrationFile],
) -> Result<(), MigrationValidationError> {
    guard::database(&id.database)?;
    let directory = tempfile::tempdir().map_err(|error| {
        MigrationValidationError::new(format!(
            "Could not create temporary SQLite validation directory: {error}"
        ))
    })?;
    let path = directory.path().join(format!("{}.db", id.database));
    let options = SqliteConnectOptions::new()
        .filename(path)
        .create_if_missing(true);
    let pool = SqlitePool::connect_with(options).await.map_err(|error| {
        MigrationValidationError::new(format!(
            "Could not create isolated SQLite validation database: {error}"
        ))
    })?;
    let result = execute_sqlite::run(&pool, migrations).await;
    pool.close().await;
    drop(directory);
    result
}
