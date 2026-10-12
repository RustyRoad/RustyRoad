//! Pool sizing, read from the environment so a deployment can tune it without a rebuild.

use std::env;
use std::time::Duration;

/// How the shared pool is sized and recycled.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PoolSettings {
    /// `DATABASE_POOL_MAX_CONNECTIONS`, default 50.
    pub max_connections: u32,
    /// `DATABASE_POOL_MIN_CONNECTIONS`, default 5, capped at the maximum.
    pub min_connections: u32,
    /// `DATABASE_POOL_ACQUIRE_TIMEOUT_SECS`, default 30.
    pub acquire_timeout: Duration,
    /// `DATABASE_POOL_IDLE_TIMEOUT_SECS`, default 600.
    pub idle_timeout: Duration,
    /// `DATABASE_POOL_MAX_LIFETIME_SECS`, default 1800.
    pub max_lifetime: Duration,
}

impl Default for PoolSettings {
    fn default() -> Self {
        Self {
            max_connections: 50,
            min_connections: 5,
            acquire_timeout: Duration::from_secs(30),
            idle_timeout: Duration::from_secs(600),
            max_lifetime: Duration::from_secs(1800),
        }
    }
}

impl PoolSettings {
    /// Reads the settings, keeping the default for any variable unset or unparsable.
    pub fn from_env() -> Self {
        let defaults = Self::default();

        Self {
            max_connections: count("DATABASE_POOL_MAX_CONNECTIONS", defaults.max_connections),
            min_connections: count("DATABASE_POOL_MIN_CONNECTIONS", defaults.min_connections),
            acquire_timeout: seconds(
                "DATABASE_POOL_ACQUIRE_TIMEOUT_SECS",
                defaults.acquire_timeout,
            ),
            idle_timeout: seconds("DATABASE_POOL_IDLE_TIMEOUT_SECS", defaults.idle_timeout),
            max_lifetime: seconds("DATABASE_POOL_MAX_LIFETIME_SECS", defaults.max_lifetime),
        }
    }
}

/// Reads a whole-number variable, keeping `default` when it is unset or unparsable.
fn count(key: &str, default: u32) -> u32 {
    env::var(key)
        .ok()
        .and_then(|value| value.trim().parse().ok())
        .unwrap_or(default)
}

/// Reads a duration in whole seconds, keeping `default` when unset or unparsable.
fn seconds(key: &str, default: Duration) -> Duration {
    env::var(key)
        .ok()
        .and_then(|value| value.trim().parse().ok())
        .map(Duration::from_secs)
        .unwrap_or(default)
}
