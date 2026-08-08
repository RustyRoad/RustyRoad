//! Rollback behaviour: `down` state and re-applying afterwards.

use super::support::sqlite_ledger;
use crate::database::migrations::ledger;
use crate::database::migrations::MigrationDirection;

#[tokio::test]
async fn rollback_of_unapplied_migration_is_skipped() {
    let connection = sqlite_ledger().await;

    assert!(
        ledger::should_skip(&connection, "create_users_table", MigrationDirection::Down)
            .await
            .unwrap()
    );
}

#[tokio::test]
async fn reapplied_after_rollback() {
    let connection = sqlite_ledger().await;
    let name = "create_users_table";

    ledger::record(&connection, name, MigrationDirection::Up)
        .await
        .unwrap();
    ledger::record(&connection, name, MigrationDirection::Down)
        .await
        .unwrap();

    // Rolled back, so `up` must run again rather than being skipped. Row ordering
    // cannot answer this once recording upserts, so state is read by row presence.
    assert!(!ledger::is_applied(&connection, name).await.unwrap());
    assert!(
        !ledger::should_skip(&connection, name, MigrationDirection::Up)
            .await
            .unwrap()
    );

    ledger::record(&connection, name, MigrationDirection::Up)
        .await
        .unwrap();
    assert!(ledger::is_applied(&connection, name).await.unwrap());
}
