use super::error::MigrationValidationError;
use super::identity::Identity;
use crate::database::Database;
use sqlx::mysql::{MySqlConnectOptions, MySqlPool};

pub(super) async fn admin(database: &Database) -> Result<MySqlPool, MigrationValidationError> {
    let options = MySqlConnectOptions::new()
        .username(&database.username)
        .password(&database.password)
        .host(&database.host)
        .port(database.port);
    MySqlPool::connect_with(options).await.map_err(|error| {
        MigrationValidationError::new(format!(
            "Could not connect to MySQL test server at {}:{}: {error}",
            database.host, database.port
        ))
    })
}

pub(super) async fn scoped(
    database: &Database,
    id: &Identity,
) -> Result<MySqlPool, MigrationValidationError> {
    let options = MySqlConnectOptions::new()
        .username(&id.user)
        .password(&id.password)
        .database(&id.database)
        .host(&database.host)
        .port(database.port);
    MySqlPool::connect_with(options).await.map_err(|error| {
        MigrationValidationError::new(format!(
            "Could not connect to isolated MySQL validation database '{}': {error}",
            id.database
        ))
    })
}
