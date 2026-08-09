//! Baselining records migrations in the ledger without executing them.

use super::support::sqlite_ledger;
use crate::database::migrations::{baseline, ledger, MigrationDirection};
use crate::database::DatabaseConnection;

#[tokio::test]
async fn baselined_migrations_are_then_skipped() {
    let connection = sqlite_ledger().await;
    let id = "20251120000000-zone_id_nullable";

    // Before: genuinely pending, so it would run against a schema that has
    // already moved on, which is the failure mode baseline exists to prevent.
    assert!(
        !ledger::should_skip(&connection, id, MigrationDirection::Up)
            .await
            .unwrap()
    );

    baseline::record_ids(&connection, &[id.to_string()])
        .await
        .unwrap();

    // After: recorded as baselined without executing any SQL.
    assert!(ledger::should_skip(&connection, id, MigrationDirection::Up)
        .await
        .unwrap());

    let DatabaseConnection::Sqlite(pool) = &connection else {
        panic!("expected sqlite connection");
    };
    let (provenance, checksum, verified_at): (String, Option<String>, Option<String>) =
        sqlx::query_as(
            "SELECT provenance, checksum, CAST(verified_at AS TEXT) \
             FROM _rustyroad_migrations WHERE name = ? AND direction = 'up'",
        )
        .bind(id)
        .fetch_one(pool.as_ref())
        .await
        .unwrap();
    assert_eq!(provenance, "baselined");
    assert_eq!(checksum, None);
    assert_eq!(verified_at, None);
}

#[tokio::test]
async fn baseline_reports_only_newly_recorded_migrations() {
    let connection = sqlite_ledger().await;
    let already = "20251114211514-add_columns";
    let pending = "20251120000000-zone_id_nullable";

    ledger::record(&connection, already, MigrationDirection::Up)
        .await
        .unwrap();

    let recorded = baseline::record_ids(&connection, &[already.to_string(), pending.to_string()])
        .await
        .unwrap();

    // The already-applied migration is not re-recorded.
    assert_eq!(recorded, vec![pending.to_string()]);
}
