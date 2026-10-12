//! Resolving where the shared pool connects.
//!
//! Checked in priority order, so a deployment can override the checked-in config file
//! without editing it:
//!
//! 1. `DATABASE_URL`
//! 2. `DATABASE_HOST`, `DATABASE_PORT`, `DATABASE_USER`, `DATABASE_PASSWORD`, `DATABASE_NAME`
//! 3. the `[database]` section of the first RustyRoad config file found (see `locate`)

use super::{locate, toml_source};
use sqlx::postgres::PgConnectOptions;
use std::{env, fs};

/// The connection settings every source has to supply, in `build`'s order.
pub(super) const KEYS: [&str; 5] = ["host", "port", "user", "password", "name"];

/// Returns the connect options for the shared pool.
///
/// Built from parts rather than formatted into a URL, so a password containing `@` or `/`
/// needs no escaping.
pub fn connect_options() -> Result<PgConnectOptions, String> {
    if let Ok(url) = env::var("DATABASE_URL") {
        return url
            .parse()
            .map_err(|error| format!("DATABASE_URL is not a valid Postgres URL: {error}"));
    }

    if let Some(values) = from_env() {
        return build(&values, "DATABASE_* environment variables");
    }

    let path = locate::config_file().ok_or_else(|| {
        "no database configuration: set DATABASE_URL, the DATABASE_* variables, \
         or add a [database] section to rustyroad.toml"
            .to_string()
    })?;
    let source = path.display().to_string();
    let contents =
        fs::read_to_string(&path).map_err(|error| format!("failed to read {source}: {error}"))?;

    build(&toml_source::values(&contents, &source)?, &source)
}

/// Reads the `DATABASE_*` variables, or nothing unless all of them are set.
fn from_env() -> Option<Vec<String>> {
    KEYS.iter()
        .map(|key| env::var(format!("DATABASE_{}", key.to_uppercase())).ok())
        .collect()
}

/// Builds connect options from values ordered as `KEYS`.
pub(super) fn build(values: &[String], source: &str) -> Result<PgConnectOptions, String> {
    let [host, port, user, password, name] = values else {
        return Err(format!("{source} does not supply every database setting"));
    };
    let port = port
        .parse::<u16>()
        .map_err(|error| format!("{source} has an invalid database port '{port}': {error}"))?;

    Ok(PgConnectOptions::new()
        .host(host)
        .port(port)
        .username(user)
        .password(password)
        .database(name))
}
