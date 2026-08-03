//! Rolling back a started-but-incomplete migration.

use super::{context, report, SCHEMA};
use crate::database::versions::{lifecycle, query, source};

/// Undoes the in-progress migration, keeping the previous version in place.
pub(super) async fn run() {
    let connection = context::connect().await;

    let active = match query::active(&connection).await {
        Ok(Some(active)) => active,
        Ok(None) => return report::fail("no migration is in progress"),
        Err(error) => return report::fail(&error.to_string()),
    };

    // Undoing a started migration applies its down.sql, when it has one.
    let sql = source::down(&active).unwrap_or_default();

    match lifecycle::rollback(&connection, SCHEMA, &sql).await {
        Ok(version) => report::rolled_back(&version),
        Err(error) => report::fail(&error.to_string()),
    }
}
