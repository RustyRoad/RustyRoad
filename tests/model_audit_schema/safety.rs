//! Incomplete discovery permits eligible groups but output collisions fail early.
use super::fixture::{audit, project};
use std::fs;

#[test]
fn partial_source_audit_keeps_independent_eligible_group() {
    let dir = project("pub struct Bookmark {pub id:i64}");
    fs::write(dir.path().join("src/broken.rs"), "struct Broken {").unwrap();
    let (report, code) = audit(
        dir.path(),
        &[
            "--generate-migrations",
            "--source",
            "src",
            "--table",
            "Bookmark=bookmark",
        ],
    );
    assert_eq!(code, 2, "{report:#}");
    assert!(report["diagnostics"]
        .as_array()
        .unwrap()
        .iter()
        .any(|diagnostic| diagnostic.as_str().unwrap().contains("broken.rs")));
    assert_eq!(report["migration_files"].as_array().unwrap().len(), 1);
    let migration = report["migration_files"][0]["directory"].as_str().unwrap();
    assert!(std::path::Path::new(migration).join("up.sql").is_file());
    assert!(std::path::Path::new(migration).join("down.sql").is_file());
    assert_eq!(
        fs::read_to_string(dir.path().join("src/broken.rs")).unwrap(),
        "struct Broken {"
    );
}

#[test]
fn existing_output_report_is_preserved_without_generation() {
    let dir = project("pub struct Bookmark {pub id:i64}");
    let path = dir.path().join("report.json");
    fs::write(&path, "keep report").unwrap();
    let (report, code) = audit(
        dir.path(),
        &["--generate-migrations", "--output", path.to_str().unwrap()],
    );
    assert_eq!(code, 1, "{report:#}");
    assert_eq!(fs::read_to_string(&path).unwrap(), "keep report");
    assert!(!dir.path().join("config").exists());
}

#[test]
fn missing_table_requires_ownership_even_with_a_primary_key_contract() {
    for args in [
        vec!["--generate-migrations"],
        vec!["--generate-migrations", "--primary-key", "bookmark=id"],
    ] {
        let dir = project("pub struct Bookmark {pub id:i64}");
        let (report, code) = audit(dir.path(), &args);
        assert_eq!(code, 2, "{report:#}");
        assert_eq!(
            report["candidates"][0]["table_audit"]["mapping"],
            "unresolved_persistence_contract"
        );
        assert!(report["migration_files"].as_array().unwrap().is_empty());
        assert!(!dir.path().join("config").exists());
        assert_eq!(
            report["declaration_coverage"][0]["outcome"],
            "unresolved_contract"
        );
    }
}

#[test]
fn incomplete_catalog_cannot_prove_table_absence() {
    let dir = project("pub struct Bookmark {pub id:i64}");
    fs::write(
        dir.path().join("catalog.json"),
        r#"{"version":1,"schema":"public","complete":false,"tables":[],"reserved_names":[]}"#,
    )
    .unwrap();
    let (report, code) = audit(
        dir.path(),
        &["--generate-migrations", "--table", "Bookmark=bookmark"],
    );
    assert_eq!(code, 2, "{report:#}");
    assert!(report["migration_files"].as_array().unwrap().is_empty());
    assert!(!dir.path().join("config").exists());
}
