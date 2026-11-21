//! Explicit key changes preserve observed rollback facts in reviewable drafts.
use super::{
    fixture::{audit, catalog, project, sql},
    key_fixture::{replacement, table},
};
use serde_json::json;

#[test]
fn explicit_redesign_repairs_nullable_key_and_retains_original_constraint() {
    let dir = replacement(table());
    let args = ["--generate-migrations", "--primary-key", "rows=replacement"];
    let (report, code) = audit(dir.path(), &args);
    assert_eq!(code, 0, "{report}");
    let up = sql(&report, "up.sql");
    let down = sql(&report, "down.sql");
    for direction in [&up, &down] {
        assert_eq!(direction.matches("ALTER TABLE").count(), 1);
        assert_eq!(direction.matches(';').count(), 1);
        assert!(direction.contains("DROP CONSTRAINT \"original_rows_key\" RESTRICT"));
        assert!(!direction.contains("CASCADE"));
    }
    assert!(up.contains("ALTER COLUMN \"id\" TYPE bigint USING (\"id\")::bigint"));
    assert!(up.contains("ALTER COLUMN \"id\" DROP NOT NULL"));
    assert!(up.contains("PRIMARY KEY (\"replacement\") NOT DEFERRABLE"));
    assert!(down.contains("ALTER COLUMN \"id\" TYPE integer USING (\"id\")::integer"));
    assert!(down.contains("ALTER COLUMN \"id\" SET NOT NULL"));
    assert!(down.contains("PRIMARY KEY (\"id\") DEFERRABLE INITIALLY DEFERRED"));
    let (retry, code) = audit(dir.path(), &args);
    assert_eq!(code, 0, "{retry}");
    assert_eq!(
        report["migration_files"][0]["directory"],
        retry["migration_files"][0]["directory"]
    );
}

#[test]
fn explicit_key_only_change_and_addition_generate_without_column_drift() {
    for existing in [true, false] {
        let dir = project("struct Rows { id: i32, replacement: i64 }");
        let mut facts = table();
        if !existing {
            facts["primary_key"] = json!([]);
            facts["primary_key_details"] = json!(null);
        }
        catalog(dir.path(), json!([facts]));
        let (report, code) = audit(
            dir.path(),
            &["--generate-migrations", "--primary-key", "rows=replacement"],
        );
        assert_eq!(code, 0, "{report}");
        let up = sql(&report, "up.sql");
        let down = sql(&report, "down.sql");
        assert!(up.contains("PRIMARY KEY (\"replacement\") NOT DEFERRABLE"));
        assert!(!up.contains("ALTER COLUMN"));
        assert_eq!(down.contains("PRIMARY KEY (\"id\")"), existing);
    }
}

#[test]
fn key_validation_uses_the_union_of_shared_table_projections() {
    let dir = project("struct Rows { id: Option<i32> } struct Row { replacement: i64 }");
    catalog(dir.path(), json!([table()]));
    let (report, code) = audit(
        dir.path(),
        &["--generate-migrations", "--primary-key", "rows=replacement"],
    );
    assert_eq!(code, 0, "{report}");
    assert_eq!(report["migration_files"].as_array().unwrap().len(), 1);
    assert!(sql(&report, "up.sql").contains("PRIMARY KEY (\"replacement\")"));
}
