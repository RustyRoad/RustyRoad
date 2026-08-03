//! Rollback of a started-but-incomplete migration.

use super::support::{history_rows, sqlite};
use crate::database::versions::{lifecycle, query};

#[tokio::test]
async fn rollback_discards_an_incomplete_migration() {
    let connection = sqlite().await;

    lifecycle::start(&connection, "main", "01_first", &[])
        .await
        .unwrap();
    lifecycle::complete(&connection, "main").await.unwrap();
    lifecycle::start(&connection, "main", "02_second", &[])
        .await
        .unwrap();

    let rolled_back = lifecycle::rollback(&connection, "main", &[]).await.unwrap();

    assert_eq!(rolled_back, "02_second");
    assert_eq!(query::active(&connection).await.unwrap(), None);
    // The completed predecessor remains the served version.
    assert_eq!(
        query::current_version(&connection).await.unwrap().as_deref(),
        Some("01_first")
    );
    assert_eq!(history_rows(&connection).await, 1);
}

#[tokio::test]
async fn failed_start_does_not_leave_a_dangling_active_migration() {
    let connection = sqlite().await;

    lifecycle::start(
        &connection,
        "main",
        "01_broken",
        &["THIS IS NOT VALID SQL".to_string()],
    )
    .await
    .expect_err("invalid DDL should fail the start");

    // The history row is withdrawn, so the next migration can start.
    assert_eq!(query::active(&connection).await.unwrap(), None);
    assert_eq!(history_rows(&connection).await, 0);
}
