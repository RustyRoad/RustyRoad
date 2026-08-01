//! Placeholder-style selection for ledger statements.
//!
//! Postgres uses `$1`-style placeholders while MySQL and SQLite use `?`.

use crate::database::DatabaseConnection;

/// Selects the placeholder variant appropriate for `connection`.
pub(super) fn pick<'a>(
    connection: &DatabaseConnection,
    numbered: &'a str,
    positional: &'a str,
) -> &'a str {
    match connection {
        DatabaseConnection::Pg(_) => numbered,
        DatabaseConnection::MySql(_) | DatabaseConnection::Sqlite(_) => positional,
    }
}
