//! New-table key declarations must resolve to the selected table contract.
use super::fixture::{audit, project, sql};

#[test]
fn new_table_omits_unspecified_primary_key() {
    let dir = project("pub struct Bookmark {pub id:i64}");
    let (report, code) = audit(
        dir.path(),
        &["--generate-migrations", "--table", "Bookmark=bookmark"],
    );
    assert_eq!(code, 0, "{report:#}");
    assert!(!sql(&report, "up.sql").contains("PRIMARY KEY"));
}

#[test]
fn explicit_new_table_primary_key_is_rendered() {
    let dir = project("pub struct Bookmark {pub id:i64}");
    let (report, code) = audit(
        dir.path(),
        &[
            "--generate-migrations",
            "--table",
            "Bookmark=bookmark",
            "--primary-key",
            "bookmark=id",
        ],
    );
    assert_eq!(code, 0, "{report:#}");
    assert!(sql(&report, "up.sql").contains("PRIMARY KEY"));
}

#[test]
fn filtered_primary_key_contract_fails_before_any_files() {
    let dir = project(
        "pub struct Row {pub id:i32}\n\
        pub struct Bookmark {pub id:i64}",
    );
    let (report, code) = audit(
        dir.path(),
        &[
            "--name",
            "Row",
            "--generate-migrations",
            "--primary-key",
            "bookmark=id",
        ],
    );
    assert_eq!(code, 1, "{report:#}");
    assert!(
        report["error"]["message"]
            .as_str()
            .unwrap()
            .contains("did not resolve to an audited table"),
        "{report:#}"
    );
    assert!(!dir.path().join("config").exists());
}
