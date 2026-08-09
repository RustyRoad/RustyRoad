use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::Executor;
use std::fs;
use std::process::Command;

#[tokio::test]
async fn list_matches_full_ledger_identity_to_disk_migration() {
    let root = tempfile::tempdir().unwrap();
    let migration = root
        .path()
        .join("config/database/migrations/20260723120000-change_customer_id");
    fs::create_dir_all(&migration).unwrap();
    fs::write(migration.join("up.sql"), "SELECT 1;").unwrap();
    fs::write(
        root.path().join("rustyroad.toml"),
        "[database]\ndatabase_name = \"ledger\"\ndatabase_user = \"\"\n\
         database_password = \"\"\ndatabase_host = \"\"\ndatabase_port = \"0\"\n\
         database_type = \"sqlite\"\n",
    )
    .unwrap();
    let options = SqliteConnectOptions::new()
        .filename(root.path().join("ledger.db"))
        .create_if_missing(true);
    let pool = SqlitePoolOptions::new()
        .connect_with(options)
        .await
        .unwrap();
    pool.execute(
        "CREATE TABLE _rustyroad_migrations (id INTEGER PRIMARY KEY, name TEXT NOT NULL, \
         applied_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP, direction TEXT NOT NULL)",
    )
    .await
    .unwrap();
    sqlx::query("INSERT INTO _rustyroad_migrations (name, direction) VALUES (?, 'up')")
        .bind("20260723120000-change_customer_id")
        .execute(&pool)
        .await
        .unwrap();
    pool.close().await;
    let output = Command::new(env!("CARGO_BIN_EXE_rustyroad"))
        .current_dir(root.path())
        .args(["--format", "json", "migration", "list"])
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success());
    assert!(stdout.contains("\"name\": \"change_customer_id\""));
    assert!(stdout.contains("\"status\": \"RECORDED IN LEDGER\""));
    assert!(stdout.contains("\"provenance\": \"legacy\""));
    assert!(stdout.contains("\"effects\": \"UNVERIFIED\""));
    assert!(!stdout.contains("\"status\": \"PENDING\""));
}
