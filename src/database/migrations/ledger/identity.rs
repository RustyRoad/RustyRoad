//! Migration identity as recorded in the ledger.

use std::path::Path;

/// Returns the ledger identity for a migration directory path.
///
/// The ledger stores the full `<timestamp>-<name>` directory name so ordering is
/// reconstructable from the ledger alone and two migrations sharing a bare name
/// cannot collide.
pub fn ledger_id_for_dir(migration_dir: &str) -> Option<&str> {
    Path::new(migration_dir).file_name()?.to_str()
}

/// Returns the legacy bare-name form of a ledger identity.
///
/// Ledgers written before full-directory-name recording stored `add_columns` rather
/// than `20251114211514-add_columns`. Returns `None` when `ledger_id` carries no
/// timestamp prefix and is therefore already a bare name.
pub(super) fn legacy_bare_name(ledger_id: &str) -> Option<&str> {
    ledger_id.split_once('-').map(|(_, name)| name)
}
