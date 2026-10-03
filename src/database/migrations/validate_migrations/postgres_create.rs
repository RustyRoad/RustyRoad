use super::cleanup;
use super::error::MigrationValidationError;
use super::guard;
use super::identity::Identity;
use super::postgres_drop;
use sqlx::{AssertSqlSafe, PgPool};

pub(super) async fn resources(
    pool: &PgPool,
    id: &Identity,
) -> Result<(), MigrationValidationError> {
    guard::database(&id.database)?;
    guard::user(&id.user)?;
    guard::password(&id.password)?;
    let role = format!(
        "CREATE ROLE {} LOGIN PASSWORD '{}' NOSUPERUSER NOCREATEDB NOCREATEROLE NOINHERIT",
        id.user, id.password
    );
    sqlx::query(AssertSqlSafe(role))
        .execute(pool)
        .await
        .map_err(|error| {
            MigrationValidationError::new(format!(
                "Could not create scoped PostgreSQL role: {error}"
            ))
        })?;
    let database = format!("CREATE DATABASE {} OWNER {}", id.database, id.user);
    if let Err(error) = sqlx::query(AssertSqlSafe(database)).execute(pool).await {
        let failure = MigrationValidationError::new(format!(
            "Could not create isolated PostgreSQL database '{}': {error}",
            id.database
        ));
        let drop_role = postgres_drop::role(pool, id).await;
        return Err(cleanup::after_validation(Err(failure), drop_role).unwrap_err());
    }
    Ok(())
}
