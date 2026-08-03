//! Invalid lifecycle transitions are rejected.

use super::support::sqlite;
use crate::database::versions::lifecycle;

#[tokio::test]
async fn complete_without_active_migration_is_rejected() {
    let connection = sqlite().await;

    let error = lifecycle::complete(&connection, "main")
        .await
        .expect_err("complete with nothing in progress should fail");
    assert!(error.to_string().contains("no migration is in progress"));
}

#[tokio::test]
async fn rollback_without_active_migration_is_rejected() {
    let connection = sqlite().await;

    let error = lifecycle::rollback(&connection, "main", &[])
        .await
        .expect_err("rollback with nothing in progress should fail");
    assert!(error.to_string().contains("no migration is in progress"));
}
