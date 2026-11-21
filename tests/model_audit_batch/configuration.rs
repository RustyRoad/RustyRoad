//! Project defaults and explicit overrides keep schema selection observable.
use super::fixture::{audit, audit_with_config};

#[test]
fn configured_custom_mapping_is_loaded_and_cli_mapping_wins() {
    let source = "pub struct Payload {pub value:DomainValue}";
    let config = "[project]\nname='fixture'\n[models.type_mappings]\nDomainValue='text'\n";
    // Storage mappings do not establish table ownership; this fixture does.
    let table = ["--table", "Payload=payload"];
    let (_dir, configured, code) = audit_with_config(source, &table, Some(config));
    assert_eq!(code, 0, "{configured:#}");
    let columns = &configured["candidates"][0]["table_audit"]["missing_columns"];
    assert_eq!(columns[0]["sql_type"], "TEXT");
    let (_dir, overridden, code) = audit_with_config(
        source,
        &[
            "--table",
            "Payload=payload",
            "--type-map",
            "DomainValue=jsonb",
        ],
        Some(config),
    );
    assert_eq!(code, 0, "{overridden:#}");
    let columns = &overridden["candidates"][0]["table_audit"]["missing_columns"];
    assert_eq!(columns[0]["sql_type"], "JSONB");
}

#[test]
fn invalid_or_conflicting_custom_mappings_fail_before_generation() {
    let source = "pub struct Payload {pub value:DomainValue}";
    for options in [
        vec!["--type-map", "DomainValue=text; DROP TABLE rows"],
        vec![
            "--type-map",
            "DomainValue=text",
            "--type-map",
            "DomainValue=jsonb",
        ],
    ] {
        let (dir, report, code) = audit(source, &options);
        assert_eq!(code, 1, "{report:#}");
        assert!(report["error"]["message"].as_str().is_some(), "{report:#}");
        assert!(!dir.path().join("config/database/migrations").exists());
    }
}

#[test]
fn explicit_model_selection_restores_narrow_generation() {
    let source = "pub struct Row {pub id:i32,pub note:Option<String>}\n\
                  pub struct Bookmark {pub id:i64}";
    let (_dir, report, code) = audit(source, &["--name", "Row"]);
    assert_eq!(code, 0, "{report:#}");
    assert_eq!(report["candidates"].as_array().unwrap().len(), 1);
    assert_eq!(report["migration_files"].as_array().unwrap().len(), 1);
    assert_eq!(report["candidates"][0]["table_audit"]["table"], "rows");
}

#[test]
fn group_scope_does_not_generate_the_other_group() {
    let source = "pub struct Row {pub id:i32,pub note:Option<String>}\n\
                  pub struct Bookmark {pub id:i64}";
    for (scope, table) in [("existing", "rows"), ("new", "bookmark")] {
        let (_dir, report, code) = audit(
            source,
            &["--table", "Bookmark=bookmark", "--migration-scope", scope],
        );
        assert_eq!(code, 0, "{report:#}");
        let migrations = report["migration_files"].as_array().unwrap();
        assert_eq!(migrations.len(), 1);
        let candidates = report["candidates"].as_array().unwrap();
        let selected = candidates
            .iter()
            .find(|candidate| candidate["model"]["id"] == migrations[0]["model_id"])
            .unwrap();
        assert_eq!(selected["table_audit"]["table"], table);
        assert_eq!(candidates.len(), 2);
    }
}
