use std::fs;
use std::process::{Command, Output};

pub fn project() -> tempfile::TempDir {
    let root = tempfile::tempdir().unwrap();
    let migration = root
        .path()
        .join("config/database/migrations/20260722-change_customer_id");
    fs::create_dir_all(&migration).unwrap();
    fs::write(
        migration.join("up.sql"),
        "ALTER TABLE orders ALTER COLUMN customer_id TYPE BIGINT USING customer_id::BIGINT;",
    )
    .unwrap();
    fs::write(
        root.path().join("rustyroad.toml"),
        "[rustyroad_project]\nname = \"breaking_change_test\"\n",
    )
    .unwrap();
    root
}

pub fn run(root: &tempfile::TempDir, arguments: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_rustyroad"))
        .current_dir(root.path())
        .args(arguments)
        .output()
        .unwrap()
}

pub fn assert_blocked(output: &Output) {
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(2));
    assert!(stderr.contains("BREAKING MIGRATION WARNING"));
    assert!(stderr.contains("foreign-key column type"));
    assert!(stderr.contains("--allow-breaking"));
}
