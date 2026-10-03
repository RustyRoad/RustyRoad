use super::cleanup;
use super::error::MigrationValidationError;
use super::guard;
use super::identity::Identity;
use super::mysql_drop;
use sqlx::{AssertSqlSafe, MySqlPool};

pub(super) async fn resources(
    pool: &MySqlPool,
    id: &Identity,
) -> Result<(), MigrationValidationError> {
    guard::database(&id.database)?;
    guard::user(&id.user)?;
    guard::password(&id.password)?;
    let user = format!(
        "CREATE USER '{}'@'%' IDENTIFIED BY '{}'",
        id.user, id.password
    );
    sqlx::query(AssertSqlSafe(user))
        .execute(pool)
        .await
        .map_err(|error| {
            MigrationValidationError::new(format!(
                "Could not create a scoped MySQL validation user: {error}"
            ))
        })?;
    let database = format!("CREATE DATABASE `{}`", id.database);
    if let Err(error) = sqlx::query(AssertSqlSafe(database)).execute(pool).await {
        let failure = MigrationValidationError::new(format!(
            "Could not create isolated MySQL validation database '{}': {error}",
            id.database
        ));
        return Err(
            cleanup::after_validation(Err(failure), mysql_drop::user(pool, id).await).unwrap_err(),
        );
    }
    let grant = format!(
        "GRANT ALL PRIVILEGES ON `{}`.* TO '{}'@'%'",
        id.database, id.user
    );
    if let Err(error) = sqlx::query(AssertSqlSafe(grant)).execute(pool).await {
        let failure = MigrationValidationError::new(format!(
            "Could not grant access to isolated MySQL validation database: {error}"
        ));
        let database = mysql_drop::database(pool, id).await;
        let user = mysql_drop::user(pool, id).await;
        return Err(
            cleanup::after_validation(Err(failure), cleanup::combine(database, user)).unwrap_err(),
        );
    }
    Ok(())
}
