//! Start and complete: the two-phase transition.

use super::support::sqlite;
use crate::database::versions::{lifecycle, query};

#[tokio::test]
async fn start_leaves_migration_active_until_completed() {
    let connection = sqlite().await;

    lifecycle::start(&connection, "main", "01_add_users", &[])
        .await
        .unwrap();

    // Still in progress, so the previous version is still being served.
    assert_eq!(
        query::active(&connection).await.unwrap().as_deref(),
        Some("01_add_users")
    );
    assert_eq!(query::current_version(&connection).await.unwrap(), None);
}

#[tokio::test]
async fn complete_finalizes_the_active_migration() {
    let connection = sqlite().await;

    lifecycle::start(&connection, "main", "01_add_users", &[])
        .await
        .unwrap();
    let completed = lifecycle::complete(&connection, "main").await.unwrap();

    assert_eq!(completed, "01_add_users");
    assert_eq!(query::active(&connection).await.unwrap(), None);
    assert_eq!(
        query::current_version(&connection).await.unwrap().as_deref(),
        Some("01_add_users")
    );
}

#[tokio::test]
async fn only_one_migration_may_be_active() {
    let connection = sqlite().await;

    lifecycle::start(&connection, "main", "01_first", &[])
        .await
        .unwrap();

    // Starting a second migration must fail rather than fork history.
    let error = lifecycle::start(&connection, "main", "02_second", &[])
        .await
        .expect_err("second concurrent start should be rejected");
    assert!(error.to_string().contains("already in progress"));
}
