//! The process-wide pool, connected once on first use.

use super::{config::connect_options, settings::PoolSettings};
use sqlx::postgres::{PgPool, PgPoolOptions};
use tokio::sync::OnceCell;

static SHARED_POOL: OnceCell<PgPool> = OnceCell::const_new();

/// Returns the shared pool, connecting it on the first call.
///
/// Concurrent first calls connect once; the rest wait for that same initialisation.
pub async fn get_or_init_pool() -> Result<&'static PgPool, String> {
    SHARED_POOL.get_or_try_init(connect).await
}

/// Connects the shared pool and returns a handle to it.
///
/// For binaries that should fail at startup rather than on their first query.
pub async fn init_shared_pool() -> Result<PgPool, String> {
    get_or_init_pool().await.cloned()
}

/// Installs a pool the application built itself, e.g. with custom connect options.
///
/// Fails once the shared pool exists, since queries may already be running on it.
pub fn set_pool(pool: PgPool) -> Result<(), String> {
    SHARED_POOL
        .set(pool)
        .map_err(|_| "the shared database pool is already initialised".to_string())
}

/// Returns the shared pool, panicking when it cannot be connected.
///
/// The query helpers use this: a missing database is a deployment error, not something
/// each generated method can meaningfully recover from.
pub async fn pool() -> &'static PgPool {
    get_or_init_pool()
        .await
        .unwrap_or_else(|error| panic!("failed to get the database pool: {error}"))
}

/// Begins a transaction on the shared pool.
pub async fn begin() -> Result<sqlx::Transaction<'static, sqlx::Postgres>, sqlx::Error> {
    pool().await.begin().await
}

/// Opens the pool from the environment and the RustyRoad config file.
async fn connect() -> Result<PgPool, String> {
    let options = connect_options()?;
    let settings = PoolSettings::from_env();

    PgPoolOptions::new()
        .max_connections(settings.max_connections)
        .min_connections(settings.min_connections.min(settings.max_connections))
        .acquire_timeout(settings.acquire_timeout)
        .idle_timeout(settings.idle_timeout)
        .max_lifetime(settings.max_lifetime)
        .test_before_acquire(true)
        .connect_with(options)
        .await
        .map_err(|error| format!("failed to create the database pool: {error}"))
}
