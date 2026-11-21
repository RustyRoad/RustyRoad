//! Isolated CLI fixture using a catalog snapshot and an empty tool PATH.
use serde_json::{json, Value};
use std::process::Command;

/// Run a snapshot audit without project configuration, e.g. `audit(source, &[])`.
pub(super) fn audit(source: &str, extra: &[&str]) -> (tempfile::TempDir, Value, i32) {
    audit_with_config(source, extra, None)
}

/// Run with optional TOML defaults while checking that source bytes are preserved.
pub(super) fn audit_with_config(
    source: &str,
    extra: &[&str],
    config: Option<&str>,
) -> (tempfile::TempDir, Value, i32) {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("Cargo.toml"),
        "[package]\nname = \"audit-fixture\"\nversion = \"0.1.0\"\n\
         edition = \"2021\"\n[lib]\npath = \"src/models.rs\"\n",
    )
    .unwrap();
    if let Some(config) = config {
        std::fs::write(dir.path().join("rustyroad.toml"), config).unwrap();
    }
    std::fs::create_dir(dir.path().join("src")).unwrap();
    std::fs::write(dir.path().join("src/models.rs"), source).unwrap();
    let tables: Vec<_> = ["rows", "items"]
        .iter()
        .map(|name| {
            json!({
                "name":name,"kind":"table","primary_key":["id"],
                "columns":[{"name":"id","sql_type":"integer","nullable":false}]
            })
        })
        .collect();
    let catalog = json!({"version":1,"schema":"public","complete":true,
        "tables":tables,"reserved_names":[]});
    std::fs::write(dir.path().join("catalog.json"), catalog.to_string()).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rustyroad"))
        .args(["--format", "json", "model", "audit", "--root"])
        .arg(dir.path())
        .args([
            "--offline",
            "--schema-snapshot",
            "catalog.json",
            "--generate-migrations",
        ])
        .args(extra)
        .env("PATH", dir.path().join("no-tools"))
        .output()
        .unwrap();
    let report = serde_json::from_slice(&output.stdout).unwrap_or_else(|_| {
        panic!(
            "{} {}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        )
    });
    assert_eq!(
        std::fs::read_to_string(dir.path().join("src/models.rs")).unwrap(),
        source
    );
    (dir, report, output.status.code().unwrap())
}
