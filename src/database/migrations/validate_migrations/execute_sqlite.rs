use super::error::MigrationValidationError;
use super::model::MigrationFile;
use super::sql;
use sqlx::SqlitePool;

pub(super) async fn run(
    pool: &SqlitePool,
    migrations: &[MigrationFile],
) -> Result<(), MigrationValidationError> {
    for migration in migrations {
        let statement = sql::read(migration)?;
        sqlx::raw_sql(&statement)
            .execute(pool)
            .await
            .map_err(|error| sql::execution_error(migration, "SQLite", error))?;
    }
    Ok(())
}
