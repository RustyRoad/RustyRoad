//! Shared-table plans remain deduplicated, ordered, and nonconflicting.
use super::fixture::audit;

#[test]
fn batch_auto_matches_orders_and_deduplicates_shared_tables() {
    let source = "pub struct Item { pub id:i32,pub note:Option<String> }\n\
                  pub struct Row { pub id:i32,pub note:Option<String> }\n\
                  pub struct Rows { pub id:i32,pub note:Option<String> }";
    let (dir, report, code) = audit(source, &[]);
    assert_eq!(code, 0, "{report:#}");
    let migrations = report["migration_files"].as_array().unwrap();
    assert_eq!(migrations.len(), 2);
    assert_eq!(migrations[0]["model_id"], "src/models.rs::Item");
    assert_eq!(migrations[1]["model_id"], "src/models.rs::Row");
    let shared: Vec<_> = report["candidates"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|candidate| candidate["table_audit"]["table"] == "rows")
        .collect();
    assert_eq!(shared.len(), 2);
    assert!(shared
        .iter()
        .all(|candidate| { candidate["table_audit"]["blockers"] == serde_json::json!([]) }));
    for migration in migrations {
        assert_eq!(migration["state"], "generated");
        let path = migration["directory"].as_str().unwrap();
        assert!(path.starts_with(dir.path().to_str().unwrap()));
        assert!(std::path::Path::new(path).join("up.sql").exists());
        assert!(std::path::Path::new(path).join("down.sql").exists());
    }
}

#[test]
fn conflicting_models_and_ambiguous_names_do_not_write() {
    let source = "pub struct Row { pub id:i32,pub note:Option<String> }\n\
                  pub struct Rows { pub id:i32,pub note:Option<i32> }";
    let (dir, report, code) = audit(source, &[]);
    assert_eq!(code, 2);
    assert!(report["candidates"]
        .as_array()
        .unwrap()
        .iter()
        .all(|candidate| {
            !candidate["table_audit"]["blockers"]
                .as_array()
                .unwrap()
                .is_empty()
        }));
    assert!(report["migration_files"].as_array().unwrap().is_empty());
    assert!(!dir.path().join("config/database/migrations").exists());
}

#[test]
fn conflicting_new_table_contracts_require_an_explicit_choice() {
    let source = "pub struct Bookmark { pub id:i32,pub key:i32,pub note:String }\n\
                  pub struct Bookmarks { pub id:i32,pub key:i32,pub note:i32 }";
    let (dir, report, code) = audit(
        source,
        &[
            "--table",
            "Bookmarks=bookmark",
            "--primary-key",
            "bookmark=id",
        ],
    );
    assert_eq!(code, 2, "{report:#}");
    assert!(report["candidates"]
        .as_array()
        .unwrap()
        .iter()
        .all(|candidate| {
            !candidate["table_audit"]["blockers"]
                .as_array()
                .unwrap()
                .is_empty()
        }));
    assert!(report["migration_files"].as_array().unwrap().is_empty());
    assert!(!dir.path().join("config/database/migrations").exists());
}

#[test]
fn edited_owned_history_fails_preflight_without_writes() {
    let source = "pub struct Item {pub id:i32,pub note:Option<String>}\n\
                  pub struct Row {pub id:i32,pub note:Option<String>}";
    let (dir, report, code) = audit(source, &[]);
    assert_eq!(code, 0, "{report:#}");
    let item_path =
        std::path::Path::new(report["migration_files"][0]["directory"].as_str().unwrap());
    std::fs::write(item_path.join("up.sql"), "-- user edit\n").unwrap();
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_rustyroad"))
        .args(["--format", "json", "model", "audit", "--root"])
        .arg(dir.path())
        .args([
            "--offline",
            "--schema-snapshot",
            "catalog.json",
            "--generate-migrations",
        ])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert!(report["migration_files"].as_array().unwrap().is_empty());
    assert!(
        report.to_string().contains("edited or incomplete"),
        "{report:#}"
    );
}
