use super::error::MigrationValidationError;
use super::guard;
use super::identity::Identity;
use sqlx::{AssertSqlSafe, MySqlPool};

pub(super) async fn database(
    pool: &MySqlPool,
    id: &Identity,
) -> Result<(), MigrationValidationError> {
    guard::database(&id.database)?;
    sqlx::query(AssertSqlSafe(format!("DROP DATABASE `{}`", id.database)))
        .execute(pool)
        .await
        .map(|_| ())
        .map_err(|error| {
            MigrationValidationError::new(format!(
                "Could not remove isolated MySQL validation database '{}': {error}",
                id.database
            ))
        })
}

pub(super) async fn user(pool: &MySqlPool, id: &Identity) -> Result<(), MigrationValidationError> {
    guard::user(&id.user)?;
    sqlx::query(AssertSqlSafe(format!("DROP USER '{}'@'%'", id.user)))
        .execute(pool)
        .await
        .map(|_| ())
        .map_err(|error| {
            MigrationValidationError::new(format!(
                "Could not remove scoped MySQL validation user '{}': {error}",
                id.user
            ))
        })
}
