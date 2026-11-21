//! Read-only audit connections never create databases or change the process cwd.
mod settings;
use sqlx::postgres::PgConnectOptions;
use std::{env, path::Path};

/// Resolve credentials without serializing them; DATABASE_URL has first priority.
pub(super) fn options(root: &Path, config: Option<&Path>) -> Result<PgConnectOptions, String> {
    let options = if let Ok(url) = env::var("DATABASE_URL") {
        url.parse()
            .map_err(|_| "Invalid PostgreSQL DATABASE_URL (value redacted)".to_string())?
    } else {
        let values = settings::values(root, config)?;
        PgConnectOptions::new()
            .host(&values[0])
            .port(
                values[1]
                    .parse()
                    .map_err(|_| "Invalid database port (value redacted)")?,
            )
            .username(&values[2])
            .password(&values[3])
            .database(&values[4])
    };
    Ok(options.options([("default_transaction_read_only", "on")]))
}

/// Classify connection errors without echoing passwords or connection URLs.
pub(super) fn diagnostic(error: sqlx::Error) -> String {
    match error {
        sqlx::Error::Database(error) => format!(
            "PostgreSQL catalog request rejected (SQLSTATE {})",
            error.code().as_deref().unwrap_or("unknown")
        ),
        sqlx::Error::PoolTimedOut => "PostgreSQL connection timed out".into(),
        sqlx::Error::Io(error) => format!("PostgreSQL connection IO failure ({:?})", error.kind()),
        sqlx::Error::Tls(_) => "PostgreSQL TLS connection failure".into(),
        _ => "PostgreSQL catalog query/decoding failed".into(),
    }
}
