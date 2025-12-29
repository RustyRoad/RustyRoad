use crate::database::{get_mysql_pool, get_pg_pool, get_sqlite_pool};
use crate::writers::create_database_if_not_exists;
use sqlx::mysql::{MySqlConnectOptions, MySqlPool};
use sqlx::postgres::{PgConnectOptions, PgPool};
use sqlx::sqlite::SqlitePool;
use std::error::Error;
use std::fs;
use std::io;
use std::sync::Arc;
use toml::Value;

use super::databasetype::DatabaseType;

#[derive(Debug, Clone)]
pub struct Database {
    pub name: String,
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub database_type: DatabaseType,
}

#[derive(Debug, Clone)]
pub enum DatabaseConnection {
    Pg(Arc<PgPool>),
    MySql(Arc<MySqlPool>),
    Sqlite(Arc<SqlitePool>),
}

/// # Name: Database
/// ## Description
/// Struct representing a database connection configuration.
/// ## Fields
/// * `name` - The name of the database.
/// * `username` - The username for the database connection.
/// * `password` - The password for the database connection.
/// * `host` - The host where the database is located.
/// * `port` - The port on which the database is running.
/// * `database_type` - The type of the database (e.g., MySQL, PostgreSQL, SQLite).
/// ## Example
/// ```
/// use rustyroad::database::Database;
/// let db = Database::new(
///     "my_database".to_string(),
///    "my_user".to_string(),
///   "my_password".to_string(),
///   "localhost".to_string(),
///  5432,
/// "postgres",
/// );
/// ```
impl Database {
    /// # Name: new
    /// ## Description
    /// Creates a new instance of the `Database` struct.
    /// ## Arguments
    /// * `name` - The name of the database.
    /// * `username` - The username for the database connection.
    /// * `password` - The password for the database connection.
    /// * `host` - The host where the database is located.
    /// * `port` - The port on which the database is running.
    /// * `database_type` - The type of the database (e.g., "postgres", "mysql", "sqlite").
    /// ## Returns
    /// * `Database` - A new instance of the `Database` struct.
    /// ## Example
    /// ```rust
    /// use rustyroad::database::Database;
    /// let db = Database::new(
    ///    "my_database".to_string(),
    ///    "my_user".to_string(),
    ///    "my_password".to_string(),
    ///    "localhost".to_string(),
    ///    5432,
    ///    "postgres",
    /// );
    /// ```
    pub fn new(
        name: String,
        username: String,
        password: String,
        host: String,
        port: u16,
        database_type: &str,
    ) -> Database {
        Database {
            name,
            username,
            password,
            host,
            port,
            database_type: match database_type {
                "postgres" => DatabaseType::Postgres,
                "mysql" => DatabaseType::Mysql,
                "sqlite" => DatabaseType::Sqlite,
                _ => DatabaseType::Mysql,
            },
        }
    }

    /// # Name: create_database_connection
    /// ## Description
    /// Creates a database connection based on the database type.
    /// ## Returns
    /// * `Result<DatabaseConnection, Box<dyn Error + Send>>` - A result containing the database connection or an error.
    /// ## Example
    /// ```rust
    /// use rustyroad::database::Database;
    /// let db = Database::new(
    ///   "my_database".to_string(),
    ///   "my_user".to_string(),
    ///   "my_password".to_string(),
    ///   "localhost".to_string(),
    ///   5432,
    ///   "postgres",
    /// );
    /// let connection = db.create_database_connection().await.unwrap();
    /// ```
    pub async fn create_database_connection(
        &self,
    ) -> Result<DatabaseConnection, Box<dyn Error + Send>> {
        match &self.database_type {
            DatabaseType::Mysql => {
                let options = MySqlConnectOptions::new()
                    .username(&self.username)
                    .password(&self.password)
                    .database(&self.name)
                    .host(&self.host)
                    .port(self.port);
                let pool = MySqlPool::connect_with(options)
                    .await
                    .unwrap_or_else(|e| panic!(
                        "Failed to create MySQL connection pool.\n\n\
                        Config: rustyroad.toml [database] section\n\
                        Host: {}:{}\n\
                        Database: {}\n\
                        User: {}\n\n\
                        Check that MySQL is running and credentials are correct.\n\n\
                        Original error: {}",
                        self.host, self.port, self.name, self.username, e
                    ));
                Ok(DatabaseConnection::MySql(Arc::new(pool)))
            }
            DatabaseType::Sqlite => {
                let pool = SqlitePool::connect(&format!("{}.db", self.name))
                    .await
                    .unwrap_or_else(|e| panic!(
                        "Could not connect to SQLite database at '{}.db'.\n\n\
                        Ensure the file exists and is readable, or check permissions.\n\n\
                        Original error: {}",
                        self.name, e
                    ));
                Ok(DatabaseConnection::Sqlite(Arc::new(pool)))
            }
            DatabaseType::Postgres => {
                let database: Database = Database::get_database_from_rustyroad_toml().unwrap();
                let name = database.name.clone();
                let username = database.username.clone();
                let host = database.host.clone();
                let port = database.port;
                let admin_database_url = format!(
                    "postgres://{}:{}@{}:{}/postgres",
                    username, database.password, host, port,
                );
                create_database_if_not_exists(admin_database_url.as_str(), database)
                    .await
                    .unwrap_or_else(|e| panic!(
                        "Failed to create PostgreSQL database '{}'.\n\n\
                        Admin URL: postgres://{}:***@{}:{}/postgres\n\
                        Config: rustyroad.toml\n\n\
                        Ensure PostgreSQL is running and the admin credentials can create databases.\n\n\
                        Original error: {}",
                        name, username, host, port, e
                    ));

                let options = PgConnectOptions::new()
                    .username(&self.username)
                    .password(&self.password)
                    .database(&self.name)
                    .host(&self.host)
                    .port(self.port);
                let pool = PgPool::connect_with(options)
                    .await
                    .unwrap_or_else(|e| panic!(
                        "Failed to create PostgreSQL connection pool for '{}' at {}:{}.\n\n\
                        Config file: rustyroad.toml\n\n\
                        Original error: {}",
                        self.name, self.host, self.port, e
                    ));

                Ok(DatabaseConnection::Pg(Arc::new(pool)))
            }
            DatabaseType::Mongo => todo!(),
        }
    }

    /// # Name: get_database_from_rustyroad_toml
    /// ## Description
    /// Reads the database configuration from the `rustyroad.toml` file based on the current environment.
    /// ## Returns
    /// * `Result<Database, std::io::Error>` - A result containing the `Database` struct or an error if the file could not be read.
    /// ## Example
    /// ```rust
    /// use rustyroad::database::Database;
    /// let database = Database::get_database_from_rustyroad_toml().unwrap();
    /// ```
    pub fn get_database_from_rustyroad_toml() -> Result<Database, std::io::Error> {
        let environment = std::env::var("ENVIRONMENT").unwrap_or("dev".to_string());
        let file_name = if environment == "dev" {
            "rustyroad.toml".to_string()
        } else {
            format!("rustyroad.{}.toml", environment)
        };

        let file = fs::read_to_string(&file_name).map_err(|e| {
            io::Error::new(
                e.kind(),
                format!(
                    "RustyRoad could not read '{file_name}'.\n\nRun this command from your project root (the folder containing '{file_name}').\nIf you haven't created a project yet, run: rustyroad new <project_name>\n\nOriginal error: {e}",
                ),
            )
        })?;

        let toml: Value = toml::from_str(&file).map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "Failed to parse '{file_name}'.\n\nMake sure it contains a [database] section.\n\nOriginal error: {e}",
                ),
            )
        })?;

        let database_table = toml
            .get("database")
            .and_then(|v| v.as_table())
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "'{file_name}' is missing a [database] section.\n\nExpected something like:\n\n[database]\ndatabase_name = \"my_db\"\ndatabase_user = \"user\"\ndatabase_password = \"pass\"\ndatabase_host = \"localhost\"\ndatabase_port = \"5432\"\ndatabase_type = \"postgres\"\n",
                    ),
                )
            })?;

        let get_required = |key: &str| -> Result<&str, io::Error> {
            database_table
                .get(key)
                .and_then(|v| v.as_str())
                .ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!(
                            "'{file_name}' is missing [database].{key}.\n\nSee README for the expected rustyroad.toml format.",
                        ),
                    )
                })
        };

        let database_name = get_required("database_name")?.to_string();
        let database_user = get_required("database_user")?.to_string();
        let database_password = get_required("database_password")?.to_string();
        let database_host = get_required("database_host")?.to_string();
        let database_port_raw = get_required("database_port")?;
        let database_port = database_port_raw.parse::<u16>().map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "'{file_name}' has an invalid [database].database_port value '{database_port_raw}'. Expected a number like '5432'.\n\nOriginal error: {e}",
                ),
            )
        })?;
        let database_type = get_required("database_type")?;

        Ok(Database::new(
            database_name,
            database_user,
            database_password,
            database_host,
            database_port,
            database_type,
        ))
    }

    /// # Name: get_db_pool
    /// Description: Returns a database connection pool based on the database type.
    ///
    /// # Arguments
    /// * `database` - Database struct
    ///
    /// # Returns
    /// * `DatabaseConnection` - Database connection pool
    ///
    /// # Example
    /// ```
    /// use rustyroad::database::Database;
    /// let database = Database::get_database_from_rustyroad_toml().unwrap();
    /// let db_pool = database.get_db_pool(database).unwrap();
    /// ```
    pub async fn get_db_pool(database: Database) -> Result<PoolConnection, Box<dyn Error + Send>> {
        match database.database_type {
            DatabaseType::Mysql => {
                let pool = get_mysql_pool(&database)
                    .await
                    .expect("Error getting mysql pool");
                Ok(PoolConnection::MySql(pool))
            }
            DatabaseType::Sqlite => {
                let pool = get_sqlite_pool(&database)
                    .await
                    .expect("Error getting sqlite pool");
                Ok(PoolConnection::Sqlite(pool))
            }
            DatabaseType::Postgres => {
                let pool = get_pg_pool(&database).await.expect("Error getting pg pool");
                Ok(PoolConnection::Pg(pool))
            }
            DatabaseType::Mongo => todo!(),
        }
    }
}

/// # Name: PoolConnection
/// ## Description
/// Enum representing different types of database connection pools.
/// ## Variants
/// * `Pg` - PostgreSQL connection pool.
/// * `MySql` - MySQL connection pool.
/// * `Sqlite` - SQLite connection pool.
/// ## Example
/// ```rust
/// use rustyroad::database::PoolConnection;
/// let pg_pool: PoolConnection = PoolConnection::Pg(sqlx::PgPool::connect("postgres://user:password@localhost/dbname").await.unwrap());
/// let mysql_pool: PoolConnection = PoolConnection::MySql(sqlx::MySqlPool::connect("mysql://user:password@localhost/dbname").await.unwrap());
/// let sqlite_pool: PoolConnection = PoolConnection::Sqlite(sqlx::SqlitePool::connect("sqlite://my_database.db").await.unwrap());
/// ```
#[derive(Debug, Clone)]
pub enum PoolConnection {
    Pg(sqlx::PgPool),
    MySql(sqlx::MySqlPool),
    Sqlite(sqlx::SqlitePool),
}
