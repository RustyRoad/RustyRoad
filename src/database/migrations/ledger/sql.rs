//! Insert and delete statements for ledger rows.

use super::dialect;
use crate::database::DatabaseConnection;

/// Upsert that keeps one row per `(name, direction)`.
///
/// The conflict clause is backend-specific, so this cannot use [`dialect::pick`].
pub(super) fn upsert(connection: &DatabaseConnection) -> &'static str {
    match connection {
        DatabaseConnection::Pg(_) => {
            "INSERT INTO _rustyroad_migrations (name, direction, provenance, checksum) \
             VALUES ($1, $2, $3, NULLIF($4, '')) ON CONFLICT (name, direction) DO UPDATE SET \
             applied_at = CURRENT_TIMESTAMP, provenance = EXCLUDED.provenance, \
             checksum = EXCLUDED.checksum, verified_at = NULL"
        }
        DatabaseConnection::MySql(_) => {
            "INSERT INTO _rustyroad_migrations (name, direction, provenance, checksum) \
             VALUES (?, ?, ?, NULLIF(?, '')) ON DUPLICATE KEY UPDATE \
             applied_at = CURRENT_TIMESTAMP, provenance = VALUES(provenance), \
             checksum = VALUES(checksum), verified_at = NULL"
        }
        DatabaseConnection::Sqlite(_) => {
            "INSERT INTO _rustyroad_migrations (name, direction, provenance, checksum) \
             VALUES (?, ?, ?, NULLIF(?, '')) ON CONFLICT (name, direction) DO UPDATE SET \
             applied_at = CURRENT_TIMESTAMP, provenance = excluded.provenance, \
             checksum = excluded.checksum, verified_at = NULL"
        }
    }
}

/// Plain insert, used when a legacy table lacks the unique constraint.
pub(super) fn plain_insert(connection: &DatabaseConnection) -> &'static str {
    dialect::pick(
        connection,
        "INSERT INTO _rustyroad_migrations (name, direction, provenance, checksum) \
         VALUES ($1, $2, $3, NULLIF($4, ''))",
        "INSERT INTO _rustyroad_migrations (name, direction, provenance, checksum) \
         VALUES (?, ?, ?, NULLIF(?, ''))",
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
