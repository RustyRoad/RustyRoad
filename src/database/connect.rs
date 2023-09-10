use super::Database;


/// ## Name: get_pg_pool
/// ## Description: This function returns a database connection pool for postgres
/// ## Parameters:
/// * `database` - Database struct
/// ## Returns:
/// * `Result<sqlx::PgPool, sqlx::Error>` - Returns a sqlx::PgPool
/// ## Example:
/// ```
/// let database = Database::get_database_from_rustyroad_toml().unwrap();
/// let pool: sqlx::PgPool = get_pg_pool(&database).await?;
/// ```
pub fn get_pg_pool(database: &Database) -> Result<sqlx::PgPool, sqlx::Error> {
    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        database.username, database.password, database.host, database.port, database.name
    );

    let db_pool = sqlx::PgPool::connect(&database_url).await?;
    Ok(db_pool)
}


/// ## Name: get_mysql_pool
/// ## Description: This function returns a database connection pool for mysql
/// ## Parameters:
/// * `database` - Database struct
/// ## Returns:
/// * `Result<sqlx::MySqlPool, sqlx::Error>` - Returns a sqlx::MySqlPool
/// ## Example:
/// ```
/// let database = Database::get_database_from_rustyroad_toml().unwrap();
/// let pool: sqlx::MySqlPool = get_mysql_pool(&database).await?;
/// ```
pub fn get_mysql_pool(database: &Database) -> Result<sqlx::MySqlPool, sqlx::Error> {
    let database_url = format!(
        "mysql://{}:{}@{}:{}/{}",
        database.username, database.password, database.host, database.port, database.name
    );

    let db_pool = sqlx::MySqlPool::connect(&database_url).await?;
    Ok(db_pool)
}


/// ## Name: get_sqlite_pool
/// ## Description: This function returns a database connection pool for sqlite
/// ## Parameters:
/// * `database` - Database struct
/// ## Returns:
/// * `Result<sqlx::SqlitePool, sqlx::Error>` - Returns a sqlx::SqlitePool
/// ## Example:
/// ```
/// let database = Database::get_database_from_rustyroad_toml().unwrap();
/// let pool: sqlx::SqlitePool = get_sqlite_pool(&database).await?;
/// ```
pub fn get_sqlite_pool(database: &Database) -> Result<sqlx::SqlitePool, sqlx::Error> {
    let database_url = format!("{}.db", database.name);

    let db_pool = sqlx::SqlitePool::connect(&database_url).await?;
    Ok(db_pool)
}