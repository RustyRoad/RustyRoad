//! Eligibility decisions for new-table drafts.
use super::fixture::audit;

#[test]
fn new_tables_are_generated_without_inventing_primary_keys() {
    let source = "pub struct Bookmark { pub id:i64,pub label:Option<String> }";
    let (dir, report, code) = audit(source, &["--table", "Bookmark=bookmarks"]);
    assert_eq!(code, 0, "{report:#}");
    assert_eq!(report["summary"]["create_migrations"], 1);
    assert_eq!(report["migration_files"][0]["state"], "generated");
    let path = report["migration_files"][0]["directory"].as_str().unwrap();
    let up = std::fs::read_to_string(std::path::Path::new(path).join("up.sql")).unwrap();
    assert!(up.contains("CREATE TABLE"));
    assert!(!up.contains("PRIMARY KEY"));
    assert!(path.starts_with(dir.path().to_str().unwrap()));
}

#[test]
fn shared_table_contract_conflict_skips_only_that_group() {
    let source = "pub struct Row {pub id:i32,pub note:Option<String>}\n\
                  pub struct Rows {pub id:i32,pub note:Option<i32>}\n\
                  pub struct Bookmark {pub id:i64}";
    let (_dir, report, code) = audit(source, &["--table", "Bookmark=bookmarks"]);
    assert_eq!(code, 2, "{report:#}");
    let candidates = report["candidates"].as_array().unwrap();
    assert!(candidates.iter().any(|candidate| {
        let audit = &candidate["table_audit"];
        audit["table"] == "rows" && !audit["blockers"].as_array().unwrap().is_empty()
    }));
    let migrations = report["migration_files"].as_array().unwrap();
    assert_eq!(migrations.len(), 1);
    assert_eq!(migrations[0]["model_id"], "src/models.rs::Bookmark");
    assert_eq!(migrations[0]["state"], "generated");
}

#[test]
fn explicit_custom_type_mapping_unblocks_creation_without_source_edits() {
    let source = "pub struct CoverageArea {pub id:i64,pub boundary:Option<DbPolygon>}";
    let (dir, report, code) = audit(
        source,
        &[
            "--table",
            "CoverageArea=coverage_areas",
            "--type-map",
            "DbPolygon=geometry(Polygon,4326)",
        ],
    );
    assert_eq!(code, 0, "{report:#}");
    assert_eq!(report["migration_files"][0]["state"], "generated");
    let path = report["migration_files"][0]["directory"].as_str().unwrap();
    let up = std::fs::read_to_string(std::path::Path::new(path).join("up.sql")).unwrap();
    assert!(up.contains("\"boundary\" geometry(Polygon, 4326)"), "{up}");
    assert!(path.starts_with(dir.path().to_str().unwrap()));
    let warnings = &report["candidates"][0]["table_audit"]["proposal"]["warnings"];
    assert!(warnings.as_array().unwrap().iter().any(|warning| {
        warning
            .as_str()
            .is_some_and(|warning| warning.contains("extensions"))
    }));
    let (blocked_dir, blocked, code) = audit(source, &["--table", "CoverageArea=coverage_areas"]);
    assert_eq!(code, 2, "{blocked:#}");
    assert!(blocked["migration_files"].as_array().unwrap().is_empty());
    assert!(!blocked_dir
        .path()
        .join("config/database/migrations")
        .exists());
}
