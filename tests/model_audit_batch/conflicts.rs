//! Include unchanged declarations when comparing shared storage contracts.
use super::fixture::audit;

#[test]
fn unchanged_declaration_cannot_hide_conflicting_type_repair() {
    let source = "pub struct Row {pub id:i32}\n\
                  pub struct Rows {pub id:i64}\n\
                  pub struct Bookmark {pub id:i64}";
    let (_dir, report, code) = audit(source, &["--table", "Bookmark=bookmarks"]);
    assert_eq!(code, 2, "{report:#}");
    assert!(report["candidates"]
        .as_array()
        .unwrap()
        .iter()
        .any(|candidate| {
            let audit = &candidate["table_audit"];
            audit["table"] == "rows" && !audit["blockers"].as_array().unwrap().is_empty()
        }));
    let migrations = report["migration_files"].as_array().unwrap();
    assert_eq!(migrations.len(), 1);
    assert_eq!(migrations[0]["model_id"], "src/models.rs::Bookmark");
}

#[test]
fn primary_key_nullability_requires_a_key_decision() {
    let source = "pub struct Row {pub id:Option<i32>}";
    let (dir, report, code) = audit(source, &[]);
    assert_eq!(code, 2, "{report:#}");
    assert!(report["migration_files"].as_array().unwrap().is_empty());
    assert!(report.to_string().contains("primary key"));
    assert!(!dir.path().join("config/database/migrations").exists());
}
