use super::error::MigrationValidationError;
use super::identity::Identity;
use crate::database::Database;
use sqlx::postgres::{PgConnectOptions, PgPool};

pub(super) async fn admin(database: &Database) -> Result<PgPool, MigrationValidationError> {
    let options = PgConnectOptions::new()
        .username(&database.username)
        .password(&database.password)
        .database("postgres")
        .host(&database.host)
        .port(database.port);
    PgPool::connect_with(options).await.map_err(|error| {
        MigrationValidationError::new(format!(
            "Could not connect to PostgreSQL test server at {}:{}: {error}",
            database.host, database.port
        ))
    })
}

pub(super) async fn scoped(
    database: &Database,
    id: &Identity,
) -> Result<PgPool, MigrationValidationError> {
    let options = PgConnectOptions::new()
        .username(&id.user)
        .password(&id.password)
        .database(&id.database)
        .host(&database.host)
        .port(database.port);
    PgPool::connect_with(options).await.map_err(|error| {
        MigrationValidationError::new(format!(
            "Could not connect to isolated PostgreSQL validation database '{}': {error}",
            id.database
        ))
    })
}
