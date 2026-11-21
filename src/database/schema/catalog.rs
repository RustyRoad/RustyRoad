//! Read-only connections and table discovery shared by humans and agents.
use crate::database::migrations::CustomMigrationError;
use crate::database::{Database, DatabaseConnection, DatabaseType};
use sqlx::{mysql::MySqlConnectOptions, postgres::PgConnectOptions};
use std::sync::Arc;

/// Connect without invoking the application helper that creates missing databases.
pub(super) async fn connect(
    database: &Database,
) -> Result<DatabaseConnection, CustomMigrationError> {
    Ok(match database.database_type {
        DatabaseType::Postgres => DatabaseConnection::Pg(Arc::new(
            sqlx::PgPool::connect_with(
                PgConnectOptions::new()
                    .host(&database.host)
                    .port(database.port)
                    .username(&database.username)
                    .password(&database.password)
                    .database(&database.name),
            )
            .await?,
        )),
        DatabaseType::Mysql => DatabaseConnection::MySql(Arc::new(
            sqlx::MySqlPool::connect_with(
                MySqlConnectOptions::new()
                    .host(&database.host)
                    .port(database.port)
                    .username(&database.username)
                    .password(&database.password)
                    .database(&database.name),
            )
            .await?,
        )),
        DatabaseType::Sqlite => DatabaseConnection::Sqlite(Arc::new(
            sqlx::SqlitePool::connect(&format!("{}.db", database.name)).await?,
        )),
        _ => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Unsupported,
                "Schema inspection supports PostgreSQL, MySQL, and SQLite",
            )
            .into())
        }
    })
}

pub(super) async fn table_names(
    connection: &DatabaseConnection,
) -> Result<Vec<String>, CustomMigrationError> {
    let mut names: Vec<String> =
        match connection {
            DatabaseConnection::Pg(pool) => sqlx::query_scalar(
                "SELECT table_name FROM information_schema.tables WHERE table_schema = 'public'",
            )
            .fetch_all(&**pool)
            .await?,
            DatabaseConnection::MySql(pool) => sqlx::query_scalar(
                "SELECT table_name FROM information_schema.tables WHERE table_schema = DATABASE()",
            )
            .fetch_all(&**pool)
            .await?,
            DatabaseConnection::Sqlite(pool) => {
                sqlx::query_scalar("SELECT name FROM main.sqlite_master WHERE type = 'table'")
                    .fetch_all(&**pool)
                    .await?
            }
        };
    names.sort();
    Ok(names)
}
