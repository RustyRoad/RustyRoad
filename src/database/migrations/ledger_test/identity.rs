//! Ledger identity: full directory names and legacy bare-name compatibility.

use super::support::sqlite_ledger;
use crate::database::migrations::ledger;
use crate::database::migrations::MigrationDirection;

const LEGACY_NAME: &str = "add_targeting_metadata_columns";
const FULL_ID: &str = "20251114211514-add_targeting_metadata_columns";

#[test]
fn ledger_id_uses_full_directory_name() {
    assert_eq!(
        ledger::ledger_id_for_dir("./config/database/migrations/20251114211514-add_columns"),
        Some("20251114211514-add_columns")
    );
    // A trailing separator must not yield an empty identity.
    assert_eq!(
        ledger::ledger_id_for_dir("./config/database/migrations/20251114211514-add_columns/"),
        Some("20251114211514-add_columns")
    );
}

#[tokio::test]
async fn legacy_bare_name_entry_counts_as_applied() {
    let connection = sqlite_ledger().await;

    // Simulate a ledger written by an older version, which stored bare names.
    ledger::record(&connection, LEGACY_NAME, MigrationDirection::Up)
        .await
        .unwrap();

    // The runner now identifies migrations by full directory name. It must still
    // recognise the historical entry, otherwise the entire history replays.
    assert!(ledger::is_applied(&connection, FULL_ID).await.unwrap());
    assert!(
        ledger::should_skip(&connection, FULL_ID, MigrationDirection::Up)
            .await
            .unwrap()
    );
}

#[tokio::test]
async fn pending_migration_is_not_masked_by_legacy_lookup() {
    let connection = sqlite_ledger().await;

    ledger::record(&connection, LEGACY_NAME, MigrationDirection::Up)
        .await
        .unwrap();

    // A genuinely pending migration must still run despite an unrelated legacy entry.
    let pending = "20251120000000-zone_id_nullable";
    assert!(!ledger::is_applied(&connection, pending).await.unwrap());
    assert!(
        !ledger::should_skip(&connection, pending, MigrationDirection::Up)
            .await
            .unwrap()
    );
}
