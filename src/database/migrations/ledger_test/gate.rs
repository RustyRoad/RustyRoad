//! The core idempotency gate: applied migrations must be skipped.

use super::support::sqlite_ledger;
use crate::database::migrations::ledger;
use crate::database::migrations::MigrationDirection;

#[tokio::test]
async fn unrecorded_migration_runs() {
    let connection = sqlite_ledger().await;

    assert!(!ledger::is_applied(&connection, "create_users_table")
        .await
        .unwrap());
    assert!(!ledger::should_skip(
        &connection,
        "create_users_table",
        MigrationDirection::Up
    )
    .await
    .unwrap());
}

#[tokio::test]
async fn applied_migration_is_skipped() {
    let connection = sqlite_ledger().await;

    ledger::record(&connection, "create_users_table", MigrationDirection::Up)
        .await
        .unwrap();

    // This is the check that makes a repeated CI run a no-op.
    assert!(ledger::should_skip(
        &connection,
        "create_users_table",
        MigrationDirection::Up
    )
    .await
    .unwrap());
}

#[tokio::test]
async fn migrations_are_tracked_independently() {
    let connection = sqlite_ledger().await;

    ledger::record(&connection, "create_users_table", MigrationDirection::Up)
        .await
        .unwrap();

    assert!(ledger::is_applied(&connection, "create_users_table")
        .await
        .unwrap());
    assert!(!ledger::is_applied(&connection, "create_posts_table")
        .await
        .unwrap());
}
