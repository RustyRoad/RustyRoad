//! Snapshot-backed dry runs plan repairs but never create migration files.
use serde_json::json;
use std::process::Command;

#[test]
fn supported_type_repair_dry_run_reports_proposals_without_writes() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir(dir.path().join("src")).unwrap();
    let source = "pub struct Row {pub id:i64}\npub struct NewRecord {pub id:i64}";
    std::fs::write(dir.path().join("src/models.rs"), source).unwrap();
    let catalog = json!({"version":1,"schema":"public","complete":true,
        "tables":[{"name":"rows","kind":"table","primary_key":["id"],
            "columns":[{"name":"id","sql_type":"integer","nullable":false}]}],
        "reserved_names":[]});
    std::fs::write(dir.path().join("catalog.json"), catalog.to_string()).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rustyroad"))
        .args(["--format", "json", "model", "audit", "--root"])
        .arg(dir.path())
        .args(["--offline", "--schema-snapshot", "catalog.json"])
        .args(["--generate-migrations", "--dry-run"])
        // NewRecord is explicitly persisted, not inferred from a helper name.
        .args(["--table", "NewRecord=new_record"])
        .env("PATH", dir.path().join("no-tools"))
        .output()
        .unwrap();
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(output.status.code(), Some(0), "{report:#}");
    assert_eq!(report["read_only"], true);
    assert_eq!(report["summary"]["blocked"], 0);
    assert_eq!(report["summary"]["update_migrations"], 1);
    assert_eq!(report["summary"]["create_migrations"], 1);
    assert!(report["migration_files"].as_array().unwrap().is_empty());
    let candidates = report["candidates"].as_array().unwrap();
    assert!(candidates.iter().any(|candidate| {
        candidate["table_audit"]["proposal"]["up"]
            .as_str()
            .is_some_and(|sql| sql.contains("ALTER TABLE") && sql.contains("bigint"))
    }));
    assert!(candidates.iter().any(|candidate| {
        candidate["table_audit"]["proposal"]["up"]
            .as_str()
            .is_some_and(|sql| sql.contains("CREATE TABLE"))
    }));
    assert!(!dir.path().join("config/database/migrations").exists());
    assert_eq!(
        std::fs::read_to_string(dir.path().join("src/models.rs")).unwrap(),
        source
    );
}
