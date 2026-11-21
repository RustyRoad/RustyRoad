//! CLI-level fixtures for AST auditing without language-server/toolchain subprocesses.

#[path = "model_audit_cli/compatibility.rs"]
mod compatibility;
#[path = "model_audit_cli/fixtures.rs"]
mod fixtures;
use fixtures::{fixture, run};
use serde_json::Value;

#[test]
fn json_selection_and_source_preservation() {
    let dir = fixture();
    let before = std::fs::read(dir.path().join("src/models.rs")).unwrap();
    let result = run(dir.path(), &["--select", "src/models.rs::Row"]);
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let report: Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(report["read_only"], true);
    assert_eq!(report["selected"], 1);
    assert_eq!(report["candidates"][0]["classification"], "db_superset");
    assert_eq!(
        report["candidates"][0]["differences"]["generated_only"][0],
        "active"
    );
    assert_eq!(report["version"], 5);
    assert_eq!(report["analysis"], "ast");
    assert_eq!(report["candidates"][0]["usage"], "not_run");
    assert!(report["candidates"][0].get("semantic").is_none());
    assert_eq!(
        std::fs::read(dir.path().join("src/models.rs")).unwrap(),
        before
    );
    assert!(!dir.path().join("Cargo.lock").exists());
    assert!(!dir.path().join("BUILD_SCRIPT_RAN").exists());
}

#[test]
fn bad_source_reports_partial_audit_and_nonzero_exit() {
    let dir = fixture();
    std::fs::write(dir.path().join("src/broken.rs"), "pub struct {").unwrap();
    let result = run(dir.path(), &["--symbols", "syntax", "--all-sources"]);
    assert_eq!(result.status.code(), Some(2));
    let report: Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(report["diagnostics"].as_array().unwrap().len(), 1);
}
