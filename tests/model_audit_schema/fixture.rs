//! Snapshot-only fixtures: no live PostgreSQL connection or SQL application.
use serde_json::{json, Value};
use std::{fs, path::Path, process::Command};

/// Create a model plus a complete catalog, e.g. `project("struct Row {id:i32}")`.
pub(super) fn project(source: &str) -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    fs::create_dir(dir.path().join("src")).unwrap();
    fs::write(dir.path().join("src/models.rs"), source).unwrap();
    catalog(
        dir.path(),
        json!([{
            "name":"rows", "kind":"table", "primary_key":["id"],
            "columns":[{"name":"id", "sql_type":"integer", "nullable":false}]
        }]),
    );
    dir
}

/// Replace fixture catalog facts, e.g. `catalog(root, json!([]))`.
pub(super) fn catalog(root: &Path, tables: Value) {
    let value = json!({"version":1, "schema":"public", "complete":true,
        "tables":tables, "reserved_names":[]});
    fs::write(root.join("catalog.json"), value.to_string()).unwrap();
}

/// Invoke the real CLI against mocked facts and preserve source bytes.
pub(super) fn audit(root: &Path, extra: &[&str]) -> (Value, i32) {
    let before = fs::read(root.join("src/models.rs")).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rustyroad"))
        .args(["--format", "json", "model", "audit", "--root"])
        .arg(root)
        .args(["--offline", "--schema-snapshot", "catalog.json"])
        .args(extra)
        .env("PATH", root.join("no-tools"))
        .output()
        .unwrap();
    assert_eq!(fs::read(root.join("src/models.rs")).unwrap(), before);
    let report = serde_json::from_slice(&output.stdout).unwrap_or_else(|error| {
        panic!(
            "{error}: {} {}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        )
    });
    (report, output.status.code().unwrap())
}

/// Read a generated direction, e.g. `sql(&report, "up.sql")`.
pub(super) fn sql(report: &Value, direction: &str) -> String {
    let path = report["migration_files"][0]["directory"].as_str().unwrap();
    fs::read_to_string(Path::new(path).join(direction)).unwrap()
}
