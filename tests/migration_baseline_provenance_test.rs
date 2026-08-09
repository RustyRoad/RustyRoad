use sha2::{Digest, Sha256};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use std::fs;
use std::process::Command;

#[tokio::test]
async fn baseline_records_provenance_without_claiming_or_creating_effects() {
    let root = tempfile::tempdir().unwrap();
    let migration_id = "20260801000000-create_baseline_marker";
    let migration = root
        .path()
        .join("config/database/migrations")
        .join(migration_id);
    fs::create_dir_all(&migration).unwrap();
    let up_sql = b"CREATE TABLE baseline_marker (id INTEGER PRIMARY KEY);\n";
    fs::write(migration.join("up.sql"), up_sql).unwrap();
    fs::write(migration.join("down.sql"), "DROP TABLE baseline_marker;\n").unwrap();
    fs::write(
        root.path().join("rustyroad.toml"),
        "[database]\ndatabase_name = \"baseline\"\ndatabase_user = \"\"\n\
         database_password = \"\"\ndatabase_host = \"\"\ndatabase_port = \"0\"\n\
         database_type = \"sqlite\"\n",
    )
    .unwrap();

    let database_path = root.path().join("baseline.db");
    let setup_options = SqliteConnectOptions::new()
        .filename(&database_path)
        .create_if_missing(true);
    SqlitePoolOptions::new()
        .connect_with(setup_options)
        .await
        .unwrap()
        .close()
        .await;

    let output = Command::new(env!("CARGO_BIN_EXE_rustyroad"))
        .current_dir(root.path())
        .args(["migration", "baseline", "--yes"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "baseline failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("BASELINED IN LEDGER"));
    assert!(stdout.contains("Effects: UNVERIFIED"));

    let options = SqliteConnectOptions::new().filename(database_path);
    let pool = SqlitePoolOptions::new()
        .connect_with(options)
        .await
        .unwrap();
    let effect_exists: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name = 'baseline_marker'",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(effect_exists, 0, "baseline must not execute up.sql");

    let row: (String, Option<String>, Option<String>) = sqlx::query_as(
        "SELECT provenance, checksum, CAST(verified_at AS TEXT) \
         FROM _rustyroad_migrations WHERE name = ? AND direction = 'up'",
    )
    .bind(migration_id)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(row.0, "baselined");
    assert_eq!(row.1, Some(format!("{:x}", Sha256::digest(up_sql))));
    assert_eq!(row.2, None);
    pool.close().await;

    let list = Command::new(env!("CARGO_BIN_EXE_rustyroad"))
        .current_dir(root.path())
        .args(["--format", "json", "migration", "list"])
        .output()
        .unwrap();
    assert!(list.status.success());
    let list_stdout = String::from_utf8_lossy(&list.stdout);
    assert!(list_stdout.contains("\"status\": \"RECORDED IN LEDGER\""));
    assert!(list_stdout.contains("\"provenance\": \"baselined\""));
    assert!(list_stdout.contains("\"checksum_status\": \"MATCH\""));
    assert!(list_stdout.contains("\"effects\": \"UNVERIFIED\""));
    assert!(!list_stdout.contains("\"status\": \"Applied\""));

    fs::write(migration.join("up.sql"), "SELECT 2;\n").unwrap();
    let drifted = Command::new(env!("CARGO_BIN_EXE_rustyroad"))
        .current_dir(root.path())
        .args(["--format", "json", "migration", "list"])
        .output()
        .unwrap();
    assert!(drifted.status.success());
    let drifted_stdout = String::from_utf8_lossy(&drifted.stdout);
    assert!(drifted_stdout.contains("\"checksum_status\": \"MISMATCH\""));
    assert!(drifted_stdout.contains("\"effects\": \"UNVERIFIED\""));
}

#[tokio::test]
async fn executed_migration_records_checksum_without_claiming_effect_verification() {
    let root = tempfile::tempdir().unwrap();
    let migration_id = "20260802000000-create_executed_marker";
    let migration = root
        .path()
        .join("config/database/migrations")
        .join(migration_id);
    fs::create_dir_all(&migration).unwrap();
    let up_sql = b"CREATE TABLE executed_marker (id INTEGER PRIMARY KEY);\n";
    fs::write(migration.join("up.sql"), up_sql).unwrap();
    fs::write(migration.join("down.sql"), "DROP TABLE executed_marker;\n").unwrap();
    fs::write(
        migration.join("extra.sql"),
        "CREATE TABLE unchecksummed_marker (id INTEGER);\n",
    )
    .unwrap();
    fs::write(
        root.path().join("rustyroad.toml"),
        "[database]\ndatabase_name = \"executed\"\ndatabase_user = \"\"\n\
         database_password = \"\"\ndatabase_host = \"\"\ndatabase_port = \"0\"\n\
         database_type = \"sqlite\"\n",
    )
    .unwrap();
    let database_path = root.path().join("executed.db");
    let setup_options = SqliteConnectOptions::new()
        .filename(&database_path)
        .create_if_missing(true);
    SqlitePoolOptions::new()
        .connect_with(setup_options)
        .await
        .unwrap()
        .close()
        .await;

    let output = Command::new(env!("CARGO_BIN_EXE_rustyroad"))
        .current_dir(root.path())
        .args(["migration", "run", migration_id])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "migration failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let pool = SqlitePoolOptions::new()
        .connect_with(SqliteConnectOptions::new().filename(database_path))
        .await
        .unwrap();
    let effect_exists: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name = 'executed_marker'",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(effect_exists, 1);
    let unchecked_effect_exists: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM sqlite_master \
         WHERE type = 'table' AND name = 'unchecksummed_marker'",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(unchecked_effect_exists, 0);
    let row: (String, Option<String>, Option<String>) = sqlx::query_as(
        "SELECT provenance, checksum, CAST(verified_at AS TEXT) \
         FROM _rustyroad_migrations WHERE name = ? AND direction = 'up'",
    )
    .bind(migration_id)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(row.0, "executed");
    assert_eq!(row.1, Some(format!("{:x}", Sha256::digest(up_sql))));
    assert_eq!(row.2, None);
}
