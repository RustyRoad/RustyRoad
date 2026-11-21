//! Mocked-local CLI checks against an isolated SQLite fixture, not a deployed database.
use rustyroad::Project;
use serde_json::{json, Value};
use std::process::{Command, Output};

async fn fixture() -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("catalog.db");
    std::fs::File::create(&path).unwrap();
    let pool = sqlx::SqlitePool::connect(path.to_str().unwrap())
        .await
        .unwrap();
    for sql in [
        "CREATE TABLE users (id INTEGER NOT NULL, email TEXT, active INTEGER NOT NULL)",
        "CREATE TABLE usage (total INTEGER)",
        "CREATE TABLE \"user events\" (event TEXT)",
        "CREATE TABLE \"users_%\" (literal TEXT)",
        "CREATE TABLE zones (name TEXT)",
    ] {
        sqlx::query(sqlx::AssertSqlSafe(sql.to_owned()))
            .execute(&pool)
            .await
            .unwrap();
    }
    pool.close().await;
    std::fs::write(dir.path().join("rustyroad.toml"),
        "[database]\ndatabase_name = 'catalog'\ndatabase_type = 'sqlite'\ndatabase_user = ''\ndatabase_password = ''\ndatabase_host = ''\ndatabase_port = '0'\n").unwrap();
    dir
}

fn run(dir: &tempfile::TempDir, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_rustyroad"))
        .current_dir(dir.path())
        .env_remove("ENVIRONMENT")
        .env_remove("ENV")
        .env_remove("DATABASE_URL")
        .env_remove("RUSTYROAD_CONFIG_PATH")
        .args(args)
        .output()
        .unwrap()
}

fn report(output: Output) -> Value {
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).unwrap()
}

#[test]
fn arguments_parse_and_conflicts_are_rejected() {
    for args in [
        vec!["rustyroad", "db", "schema"],
        vec!["rustyroad", "db", "schema", "users"],
        vec!["rustyroad", "db", "schema", "-i"],
        vec![
            "rustyroad",
            "--format",
            "json",
            "db",
            "tables",
            "--prefix",
            "us",
        ],
    ] {
        Project::cli().try_get_matches_from(args).unwrap();
    }
    assert!(Project::cli()
        .try_get_matches_from(["rustyroad", "db", "schema", "users", "--interactive"])
        .is_err());
}

#[tokio::test]
async fn discovery_is_sorted_literal_and_noninteractive() {
    let dir = fixture().await;
    let all = report(run(&dir, &["--format", "json", "db", "tables"]));
    assert_eq!(all["database_type"], "sqlite");
    assert_eq!(all["config_file"], "rustyroad.toml");
    assert_eq!(
        all["tables"],
        json!(["usage", "user events", "users", "users_%", "zones"])
    );
    let prefixed = report(run(
        &dir,
        &["--format", "json", "db", "tables", "--prefix", "us"],
    ));
    assert_eq!(
        prefixed["tables"],
        json!(["usage", "user events", "users", "users_%"])
    );
    let literal = report(run(
        &dir,
        &["--format", "json", "db", "tables", "--prefix", "users_"],
    ));
    assert_eq!(literal["tables"], json!(["users_%"]));
    for prefix in ["US", "missing", "' OR 1=1 --"] {
        let empty = report(run(
            &dir,
            &["--format", "json", "db", "tables", "--prefix", prefix],
        ));
        assert_eq!(empty["tables"], json!([]));
    }
}

#[tokio::test]
async fn one_table_preserves_column_order_types_and_nullability() {
    let dir = fixture().await;
    let schema = report(run(&dir, &["--format", "json", "db", "schema", "users"]));
    assert_eq!(schema["tables"].as_array().unwrap().len(), 1);
    assert_eq!(schema["tables"][0]["name"], "users");
    assert_eq!(
        schema["tables"][0]["columns"],
        json!([
            {"name":"id", "type":"INTEGER", "nullable":false},
            {"name":"email", "type":"TEXT", "nullable":true},
            {"name":"active", "type":"INTEGER", "nullable":false},
        ])
    );
    let all = report(run(&dir, &["--format", "json", "db", "schema"]));
    assert_eq!(all["tables"].as_array().unwrap().len(), 5);
    for name in ["user events", "users_%"] {
        let schema = report(run(&dir, &["--format", "json", "db", "schema", name]));
        assert_eq!(schema["tables"][0]["name"], name);
        assert_eq!(schema["tables"][0]["columns"].as_array().unwrap().len(), 1);
    }
    let text = run(&dir, &["db", "schema", "users"]);
    assert!(text.status.success());
    let text = String::from_utf8(text.stdout).unwrap();
    assert!(text.contains("id: INTEGER NOT NULL"));
    assert!(!text.contains("Table: zones"));
}

#[tokio::test]
async fn missing_table_is_a_clean_error_and_cannot_inject_sql() {
    let dir = fixture().await;
    for name in ["missing", "users); DROP TABLE users; --"] {
        let output = run(&dir, &["--format", "json", "db", "schema", name]);
        assert!(!output.status.success());
        assert!(output.stdout.is_empty());
        assert!(String::from_utf8_lossy(&output.stderr).contains("not found"));
    }
    let schema = report(run(&dir, &["--format", "json", "db", "schema", "users"]));
    assert_eq!(schema["tables"][0]["columns"].as_array().unwrap().len(), 3);
}

#[test]
fn interactive_rejects_scripts_before_connecting() {
    let dir = tempfile::tempdir().unwrap();
    for args in [
        vec!["db", "schema", "--interactive"],
        vec!["--format", "json", "db", "schema", "--interactive"],
    ] {
        let output = run(&dir, &args);
        assert!(!output.status.success());
        assert!(output.stdout.is_empty());
        assert!(
            String::from_utf8_lossy(&output.stderr).contains("requires a terminal and text output")
        );
    }
}
