//! Migration identity as recorded in the ledger.

use std::path::Path;

/// Metadata read from one ledger row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LedgerRecord {
    pub name: String,
    pub recorded_at: String,
    pub direction: String,
    pub provenance: String,
    pub checksum: Option<String>,
    pub verified_at: Option<String>,
}

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
    let (prefix, name) = ledger_id.split_once('-')?;
    (!name.is_empty() && prefix.bytes().all(|byte| byte.is_ascii_digit())).then_some(name)
}

/// Returns the human-readable suffix while preserving hyphenated bare names.
pub fn display_name(ledger_id: &str) -> &str {
    legacy_bare_name(ledger_id).unwrap_or(ledger_id)
}

/// Whether full and legacy identities refer to the same migration.
pub fn identities_match(left: &str, right: &str) -> bool {
    left == right || legacy_bare_name(left) == Some(right) || legacy_bare_name(right) == Some(left)
}

/// Returns the latest ledger status for one migration directory identity.
///
/// Full identities are authoritative. A bare-name row is consulted only when no
/// full row exists, matching the execution gate in [`super::is_applied`].
/// Ledger rows must be supplied in oldest-to-newest order.
pub fn latest_status<'a>(
    migration_id: &str,
    rows: &'a [(String, String, String)],
) -> Option<(&'a str, &'a str)> {
    let exact = rows.iter().rev().find(|(name, _, _)| name == migration_id);
    let row = exact.or_else(|| {
        let bare = legacy_bare_name(migration_id)?;
        rows.iter().rev().find(|(name, _, _)| name == bare)
    })?;

    Some((row.1.as_str(), row.2.as_str()))
}

/// Returns the latest rich ledger record for one migration identity.
///
/// Exact timestamped identities remain authoritative over legacy bare-name rows.
pub fn latest_record<'a>(migration_id: &str, rows: &'a [LedgerRecord]) -> Option<&'a LedgerRecord> {
    let exact = rows.iter().rev().find(|record| record.name == migration_id);
    exact.or_else(|| {
        let bare = legacy_bare_name(migration_id)?;
        rows.iter().rev().find(|record| record.name == bare)
    })
}
