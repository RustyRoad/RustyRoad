//! Ledger integrity: uniqueness, upsert, and parameter binding.

use super::support::{row_count, sqlite_ledger};
use crate::database::migrations::ledger;
use crate::database::migrations::MigrationDirection;
use crate::database::DatabaseConnection;

#[tokio::test]
async fn ensure_table_is_idempotent() {
    let connection = sqlite_ledger().await;
    // `migration all` calls this on every run.
    ledger::ensure_table(&connection)
        .await
        .expect("ensure_table should be safe to call repeatedly");
}

#[tokio::test]
async fn repeated_record_does_not_duplicate_rows() {
    let connection = sqlite_ledger().await;
    let id = "20251114211514-add_columns";

    for _ in 0..3 {
        ledger::record(&connection, id, MigrationDirection::Up)
            .await
            .unwrap();
    }

    // The upsert keeps one authoritative row per (name, direction), instead of the
    // duplicate-per-run growth an insert-only ledger produces.
    assert_eq!(row_count(&connection, id).await, 1);
}

#[tokio::test]
async fn rollback_replaces_rather_than_appends() {
    let connection = sqlite_ledger().await;
    let id = "20251114211514-add_columns";

    ledger::record(&connection, id, MigrationDirection::Up)
        .await
        .unwrap();
    ledger::record(&connection, id, MigrationDirection::Down)
        .await
        .unwrap();

    // One row total: the `up` row is cleared so state stays unambiguous.
    assert_eq!(row_count(&connection, id).await, 1);
}

#[tokio::test]
async fn migration_names_are_bound_not_interpolated() {
    let connection = sqlite_ledger().await;

    // A quote in the name must round-trip rather than break the statement.
    let awkward = "20251114211514-create_o'brien_table";
    ledger::record(&connection, awkward, MigrationDirection::Up)
        .await
        .unwrap();

    assert!(ledger::is_applied(&connection, awkward).await.unwrap());
    assert_eq!(row_count(&connection, awkward).await, 1);
}

#[tokio::test]
async fn repair_keeps_latest_identity_state_and_restores_uniqueness() {
    let connection = sqlite_ledger().await;
    let DatabaseConnection::Sqlite(pool) = &connection else {
        panic!("expected sqlite connection");
    };
    sqlx::query("DROP TABLE _rustyroad_migrations")
        .execute(pool.as_ref())
        .await
        .unwrap();
    sqlx::query(
        "CREATE TABLE _rustyroad_migrations (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            applied_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            direction TEXT NOT NULL
        )",
    )
    .execute(pool.as_ref())
    .await
    .unwrap();
    for (direction, applied_at) in [
        ("up", "2026-01-01 00:00:00"),
        ("down", "2026-02-01 00:00:00"),
        ("up", "2026-03-01 00:00:00"),
    ] {
        sqlx::query(
            "INSERT INTO _rustyroad_migrations (name, direction, applied_at) VALUES (?, ?, ?)",
        )
        .bind("20260101000000-add_columns")
        .bind(direction)
        .bind(applied_at)
        .execute(pool.as_ref())
        .await
        .unwrap();
    }

    let report = ledger::repair(&connection).await.unwrap();

    assert_eq!(report.rows_before, 3);
    assert_eq!(report.rows_after, 1);
    assert_eq!(report.rows_removed, 2);
    assert!(
        ledger::is_applied(&connection, "20260101000000-add_columns")
            .await
            .unwrap()
    );
    assert!(
        sqlx::query("INSERT INTO _rustyroad_migrations (name, direction) VALUES (?, ?)")
            .bind("20260101000000-add_columns")
            .bind("up")
            .execute(pool.as_ref())
            .await
            .is_err()
    );
}
