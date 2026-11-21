//! Independent table groups remain eligible when another contract is unknown.
use super::fixture::audit;

#[test]
fn blocked_missing_table_does_not_suppress_eligible_tables() {
    let source = "pub struct UnknownRecord {pub value:UnknownCodec}\n\
                  pub struct Row {pub id:i32,pub note:Option<String>}\n\
                  pub struct Bookmark {pub id:i64}";
    let (dir, report, code) = audit(source, &["--table", "Bookmark=bookmarks"]);
    assert_eq!(code, 2, "{report:#}");
    let candidates = report["candidates"].as_array().unwrap();
    assert!(candidates.iter().any(|candidate| {
        !candidate["table_audit"]["blockers"]
            .as_array()
            .unwrap()
            .is_empty()
    }));
    let migrations = report["migration_files"].as_array().unwrap();
    assert_eq!(migrations.len(), 2);
    for migration in migrations {
        assert_eq!(migration["state"], "generated");
        let path = std::path::Path::new(migration["directory"].as_str().unwrap());
        assert!(path.starts_with(dir.path()));
        for file in ["up.sql", "down.sql", "model-audit.json"] {
            assert!(path.join(file).is_file());
        }
    }
}
