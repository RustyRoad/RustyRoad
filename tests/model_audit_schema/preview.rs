//! Preview proposals use mocked catalog facts without filesystem mutations.
use super::fixture::{audit, project};

#[test]
fn report_only_proposes_supported_type_and_nullability_repairs() {
    let dir = project("pub struct Row {pub id:i64,pub note:Option<String>}");
    let (report, code) = audit(dir.path(), &["--report-only"]);
    assert_eq!(code, 0, "{report:#}");
    assert_eq!(report["read_only"], true);
    assert!(report["migration_files"].as_array().unwrap().is_empty());
    let up = report["candidates"][0]["table_audit"]["proposal"]["up"]
        .as_str()
        .unwrap();
    assert!(up.contains("TYPE bigint"), "{up}");
    assert!(up.contains("ADD COLUMN"), "{up}");
    assert!(up.contains("note"), "{up}");
    assert!(!dir.path().join("config").exists());
}

#[test]
fn generation_dry_run_renders_proposal_without_files() {
    let dir = project("pub struct Row {pub id:i64}");
    let (report, code) = audit(dir.path(), &["--generate-migrations", "--dry-run"]);
    assert_eq!(code, 0, "{report:#}");
    let up = report["candidates"][0]["table_audit"]["proposal"]["up"]
        .as_str()
        .unwrap();
    assert!(up.contains("ALTER TABLE"), "{up}");
    assert!(up.contains("TYPE bigint"), "{up}");
    assert_eq!(report["read_only"], true);
    assert!(!dir.path().join("config").exists());
    assert!(report["migration_files"].as_array().unwrap().is_empty());
}
