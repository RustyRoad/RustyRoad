//! Missing restoration or dependency facts cannot authorize a key redesign.
use super::{
    fixture::audit,
    key_fixture::{replacement, table},
};
use serde_json::json;

#[test]
fn unsafe_key_redesigns_fail_before_creating_migration_files() {
    for scenario in [
        "metadata",
        "foreign_key",
        "index",
        "partition",
        "deferral",
        "identity",
    ] {
        let mut facts = table();
        match scenario {
            "metadata" => facts.as_object_mut().unwrap().remove("primary_key_details"),
            "foreign_key" => {
                facts["primary_key_details"]["referenced_by"] = json!(["public.links.rows_fk"]);
                None
            }
            "index" => {
                facts["primary_key_details"]["standard_index"] = json!(false);
                None
            }
            "partition" => {
                facts["kind"] = json!("partitioned_table");
                None
            }
            "deferral" => {
                facts["primary_key_details"]["deferrable"] = json!(false);
                None
            }
            "identity" => {
                facts["primary_key_details"]["name"] = json!("unsafe;DROP");
                None
            }
            _ => unreachable!(),
        };
        let dir = replacement(facts);
        let (report, code) = audit(
            dir.path(),
            &["--generate-migrations", "--primary-key", "rows=replacement"],
        );
        assert_eq!(code, 2, "{scenario}: {report}");
        assert!(report["migration_files"].as_array().unwrap().is_empty());
        assert!(!dir.path().join("config/database/migrations").exists());
    }
}

#[test]
fn unchanged_key_does_not_authorize_nullable_primary_key_fields() {
    let dir = replacement(table());
    let (report, code) = audit(
        dir.path(),
        &["--generate-migrations", "--primary-key", "rows=id"],
    );
    assert_eq!(code, 2, "{report}");
    assert!(report["migration_files"].as_array().unwrap().is_empty());
    assert!(!dir.path().join("config/database/migrations").exists());
}
