//! Insert and delete statements for ledger rows.

use super::dialect;
use crate::database::DatabaseConnection;

/// Upsert that keeps one row per `(name, direction)`.
///
/// The conflict clause is backend-specific, so this cannot use [`dialect::pick`].
pub(super) fn upsert(connection: &DatabaseConnection) -> &'static str {
    match connection {
        DatabaseConnection::Pg(_) => {
            "INSERT INTO _rustyroad_migrations (name, direction) VALUES ($1, $2) \
             ON CONFLICT (name, direction) DO UPDATE SET applied_at = CURRENT_TIMESTAMP"
        }
        DatabaseConnection::MySql(_) => {
            "INSERT INTO _rustyroad_migrations (name, direction) VALUES (?, ?) \
             ON DUPLICATE KEY UPDATE applied_at = CURRENT_TIMESTAMP"
        }
        DatabaseConnection::Sqlite(_) => {
            "INSERT INTO _rustyroad_migrations (name, direction) VALUES (?, ?) \
             ON CONFLICT (name, direction) DO UPDATE SET applied_at = CURRENT_TIMESTAMP"
        }
    }
}

/// Plain insert, used when a legacy table lacks the unique constraint.
pub(super) fn plain_insert(connection: &DatabaseConnection) -> &'static str {
    dialect::pick(
        connection,
        "INSERT INTO _rustyroad_migrations (name, direction) VALUES ($1, $2)",
        "INSERT INTO _rustyroad_migrations (name, direction) VALUES (?, ?)",
    )
}

/// Deletes a single `(name, direction)` row.
pub(super) fn delete_direction(connection: &DatabaseConnection) -> &'static str {
    dialect::pick(
        connection,
        "DELETE FROM _rustyroad_migrations WHERE name = $1 AND direction = $2",
        "DELETE FROM _rustyroad_migrations WHERE name = ? AND direction = ?",
    )
}
