//! Baseline is repeatable and respects legacy ledger entries.

use super::support::sqlite_ledger;
use crate::database::migrations::{baseline, ledger, MigrationDirection};

#[tokio::test]
async fn baseline_is_idempotent() {
    let connection = sqlite_ledger().await;
    let ids = vec!["20251120000000-zone_id_nullable".to_string()];

    assert_eq!(
        baseline::record_ids(&connection, &ids).await.unwrap().len(),
        1
    );
    // A second run records nothing further.
    assert!(baseline::record_ids(&connection, &ids)
        .await
        .unwrap()
        .is_empty());
}

#[tokio::test]
async fn baseline_honours_legacy_bare_name_entries() {
    let connection = sqlite_ledger().await;

    // An older ledger stored bare names.
    ledger::record(&connection, "add_columns", MigrationDirection::Up)
        .await
        .unwrap();

    let recorded =
        baseline::record_ids(&connection, &["20251114211514-add_columns".to_string()])
            .await
            .unwrap();

    // Recognised as already applied, so not duplicated under the full identity.
    assert!(recorded.is_empty());
}
