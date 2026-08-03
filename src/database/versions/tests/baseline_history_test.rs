//! Baseline entries in history.

use super::support::sqlite;
use crate::database::versions::{query, record};

#[tokio::test]
async fn baseline_entries_are_recorded_as_completed() {
    let connection = sqlite().await;

    record::insert_done(&connection, "00_existing_schema", true)
        .await
        .unwrap();

    // A baseline is never in progress; it adopts state that already exists.
    assert_eq!(query::active(&connection).await.unwrap(), None);
    assert_eq!(
        query::latest_baseline(&connection).await.unwrap().as_deref(),
        Some("00_existing_schema")
    );

    let entry = query::entry(&connection, "00_existing_schema")
        .await
        .unwrap()
        .expect("baseline entry should exist");
    assert!(entry.is_baseline());
    assert!(entry.done);
}

#[tokio::test]
async fn ordinary_migrations_are_not_baselines() {
    let connection = sqlite().await;

    record::insert_done(&connection, "01_add_users", false)
        .await
        .unwrap();

    assert_eq!(query::latest_baseline(&connection).await.unwrap(), None);

    let entry = query::entry(&connection, "01_add_users")
        .await
        .unwrap()
        .expect("entry should exist");
    assert!(!entry.is_baseline());
}

#[tokio::test]
async fn unknown_migrations_have_no_entry() {
    let connection = sqlite().await;
    assert_eq!(query::entry(&connection, "nope").await.unwrap(), None);
}
