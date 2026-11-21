use super::ProvisionError;
use crate::database::Database;
use sqlx::{postgres::PgConnectOptions, ConnectOptions, Connection, PgConnection};
use std::time::Duration;

/// Opens an explicit PostgreSQL connection without printing URLs or SQL statements.
/// Does not load process-global configuration or auto-create a database.
/// # Examples
/// ```no_run
/// # async fn example(config: &rustyroad::database::Database)
/// # -> Result<(), rustyroad::database::tenant::ProvisionError> {
/// let _connection = rustyroad::database::tenant::connection::open(config, "postgres").await?;
/// # Ok(()) }
/// ```
pub async fn open(config: &Database, database: &str) -> Result<PgConnection, ProvisionError> {
    let options = PgConnectOptions::new().host(&config.host).port(config.port)
        .username(&config.username).password(&config.password).database(database)
        .application_name("rustyroad-tenant-provisioner").disable_statement_logging();
    tokio::time::timeout(Duration::from_secs(15), PgConnection::connect_with(&options))
        .await.map_err(|_| ProvisionError::invalid("connection_timeout"))?
        .map_err(|error| ProvisionError::database("connection", &error))
}

/// Takes a session-scoped, nonblocking lock for a deterministic tenant identity.
/// Closing the dedicated connection releases it, including cancellation/error paths.
/// # Examples
/// ```no_run
/// # async fn example(conn: &mut sqlx::PgConnection)
/// # -> Result<(), rustyroad::database::tenant::ProvisionError> {
/// rustyroad::database::tenant::connection::lock(conn, "rf_00000000000000000000000000000000").await?;
/// # Ok(()) }
/// ```
pub async fn lock(conn: &mut PgConnection, name: &str) -> Result<(), ProvisionError> {
    let acquired: bool = sqlx::query_scalar(
        "SELECT pg_try_advisory_lock(hashtextextended($1, 0))",
    ).bind(name).fetch_one(conn).await
        .map_err(|error| ProvisionError::database("tenant_lock", &error))?;
    if !acquired { return Err(ProvisionError::invalid("tenant_provisioning_busy")); }
    Ok(())
}
