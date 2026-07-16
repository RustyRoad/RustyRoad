use super::error::MigrationValidationError;
use super::guard;
use super::identity::Identity;
use sqlx::PgPool;

pub(super) async fn database(pool: &PgPool, id: &Identity) -> Result<(), MigrationValidationError> {
    guard::database(&id.database)?;
    sqlx::query(&format!("DROP DATABASE {}", id.database))
        .execute(pool)
        .await
        .map(|_| ())
        .map_err(|error| {
            MigrationValidationError::new(format!(
                "Could not remove isolated PostgreSQL database '{}': {error}",
                id.database
            ))
        })
}

pub(super) async fn role(pool: &PgPool, id: &Identity) -> Result<(), MigrationValidationError> {
    guard::user(&id.user)?;
    sqlx::query(&format!("DROP ROLE {}", id.user))
        .execute(pool)
        .await
        .map(|_| ())
        .map_err(|error| {
            MigrationValidationError::new(format!(
                "Could not remove scoped PostgreSQL role '{}': {error}",
                id.user
            ))
        })
}
