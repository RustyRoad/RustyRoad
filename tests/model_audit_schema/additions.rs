//! Required additions are reviewable drafts, not inferred backfills or defaults.
use super::fixture::{audit, project, sql};

#[test]
fn required_addition_warns_without_inventing_a_backfill() {
    let dir = project("pub struct Row {pub id:i32,pub note:String}");
    let (report, code) = audit(dir.path(), &["--generate-migrations"]);
    assert_eq!(code, 0, "{report:#}");
    let up = sql(&report, "up.sql");
    assert!(up.contains("ADD COLUMN"), "{up}");
    assert!(up.contains("NOT NULL"), "{up}");
    assert!(!up.contains("DEFAULT"), "{up}");
    assert!(!up.contains("UPDATE "), "{up}");
    assert!(report.to_string().contains("backfill"), "{report:#}");
    assert!(sql(&report, "down.sql").contains("DROP COLUMN"));
}

#[test]
fn unsupported_contract_does_not_suppress_independent_table() {
    let dir = project(
        "pub struct Row {pub id:i32,pub value:UnknownCodec}\n\
        pub struct Bookmark {pub id:i64}",
    );
    let (report, code) = audit(
        dir.path(),
        &["--generate-migrations", "--table", "Bookmark=bookmark"],
    );
    assert_eq!(code, 2, "{report:#}");
    let files = report["migration_files"].as_array().unwrap();
    assert_eq!(files.len(), 1);
    assert_eq!(files[0]["model_id"], "src/models.rs::Bookmark");
    assert!(report["candidates"]
        .as_array()
        .unwrap()
        .iter()
        .any(|candidate| {
            candidate["table_audit"]["table"] == "rows"
                && !candidate["table_audit"]["blockers"]
                    .as_array()
                    .unwrap()
                    .is_empty()
        }));
    assert!(sql(&report, "up.sql").contains("CREATE TABLE"));
    assert!(!sql(&report, "up.sql").contains("UnknownCodec"));
    assert!(sql(&report, "down.sql").contains("DROP TABLE"));
}
