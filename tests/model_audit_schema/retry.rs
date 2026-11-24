//! Create-only writes reject edited owned drafts and overlapping unowned SQL.
use super::fixture::{audit, project, sql};
use std::{fs, path::Path};

#[test]
fn creates_new_table_once_and_rejects_edited_retry() {
    let dir = project("pub struct Bookmark {pub id:i64}");
    let args = ["--generate-migrations", "--table", "Bookmark=bookmark"];
    let (first, code) = audit(dir.path(), &args);
    assert_eq!(code, 0, "{first:#}");
    let (second, code) = audit(dir.path(), &args);
    assert_eq!(code, 0, "{second:#}");
    assert_eq!(second["migration_files"][0]["state"], "already_generated");
    assert_eq!(
        first["migration_files"][0]["directory"],
        second["migration_files"][0]["directory"]
    );
    let path = Path::new(first["migration_files"][0]["directory"].as_str().unwrap());
    fs::write(path.join("up.sql"), "-- user edit\n").unwrap();
    let (retry, code) = audit(dir.path(), &args);
    assert_eq!(code, 2, "{retry:#}");
    assert!(
        retry.to_string().contains("edited or incomplete"),
        "{retry:#}"
    );
    assert!(retry["migration_files"].as_array().unwrap().is_empty());
    assert_eq!(sql(&first, "up.sql"), "-- user edit\n");
}

#[test]
fn quoted_unowned_history_blocks_only_the_affected_table() {
    let dir = project(
        "pub struct Row {pub id:i32,pub note:Option<String>}\n\
        pub struct Bookmark {pub id:i64}",
    );
    let history = dir.path().join("config/database/migrations/manual");
    fs::create_dir_all(&history).unwrap();
    let up = "ALTER TABLE \"public\".\"rows\" ADD COLUMN \"note\" TEXT;\n";
    fs::write(history.join("up.sql"), up).unwrap();
    fs::write(history.join("down.sql"), "-- retained\n").unwrap();
    let (report, code) = audit(
        dir.path(),
        &["--generate-migrations", "--table", "Bookmark=bookmark"],
    );
    assert_eq!(code, 2, "{report:#}");
    assert!(report.to_string().contains("overlapping DDL"), "{report:#}");
    let files = report["migration_files"].as_array().unwrap();
    assert_eq!(files.len(), 1);
    assert_eq!(files[0]["model_id"], "src/models.rs::Bookmark");
    assert_eq!(fs::read_to_string(history.join("up.sql")).unwrap(), up);
    assert_eq!(fs::read_dir(history.parent().unwrap()).unwrap().count(), 2);
}
