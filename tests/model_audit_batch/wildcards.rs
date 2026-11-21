//! Mocked local CLI regressions for explicit contracts in wildcard scopes.
use super::fixture::audit;

#[test]
fn absolute_builtin_imports_remain_eligible_with_unknown_wildcards() {
    let source = "use unknown::*; use ::std::primitive::i32 as Id; \
        use ::std::string::String as Text; \
        pub struct Row { pub id: Id, pub value: Text }";
    let (_directory, report, code) = audit(source, &[]);
    assert_eq!(code, 0, "{report:#}");
    assert_eq!(report["migration_files"][0]["state"], "generated");
    assert_eq!(report["migration_files"].as_array().unwrap().len(), 1);
    assert_eq!(report["summary"]["create_migrations"], 0);
    assert_eq!(report["summary"]["update_migrations"], 1);
}

#[test]
fn absolute_sqlx_source_contract_remains_eligible_with_unknown_wildcards() {
    let source = "use unknown::*; \
        #[derive(::sqlx::Type)] #[sqlx(type_name = \"state\")] \
        pub enum State { Ready } \
        pub struct Row { pub state: State }";
    let (_directory, report, code) = audit(source, &[]);
    assert_eq!(code, 0, "{report:#}");
    let migrations = report["migration_files"].as_array().unwrap();
    assert_eq!(migrations.len(), 1, "{report:#}");
    let path = std::path::Path::new(migrations[0]["directory"].as_str().unwrap());
    let up = std::fs::read_to_string(path.join("up.sql")).unwrap();
    assert!(up.contains("\"state\" \"state\""), "{up}");
}

#[test]
fn explicit_type_mapping_remains_eligible_with_unknown_wildcards() {
    let source = "use unknown::*; pub struct Row { pub value: String }";
    let (_directory, report, code) = audit(source, &["--type-map", "String=text"]);
    assert_eq!(code, 0, "{report:#}");
    assert_eq!(report["migration_files"][0]["state"], "generated");
}

#[test]
fn relative_sqlx_names_cannot_authorize_codecs_with_unknown_wildcards() {
    let source = "use unknown::*; \
        #[derive(sqlx::Type)] #[sqlx(type_name = \"state\")] \
        pub enum State { Ready } pub struct Row { pub state: State }";
    let (directory, report, code) = audit(source, &[]);
    assert_eq!(code, 2, "{report:#}");
    assert!(report["migration_files"].as_array().unwrap().is_empty());
    assert!(!directory.path().join("config/database/migrations").exists());
}

/// Exercise fail-closed inference through the files-only CLI, not just the AST.
#[test]
fn wildcard_imports_cannot_generate_from_unproven_builtin_or_external_types() {
    for import in [
        "use unknown::*;",
        "#[cfg(feature = \"unknown\")] use unknown::*;",
    ] {
        for ty in ["String", "Option<i32>", "Vec<i32>", "uuid::Uuid"] {
            let source = format!("{import} pub struct Row {{ pub value: {ty} }}");
            let (directory, report, code) = audit(&source, &[]);
            assert_eq!(code, 2, "{import} {ty}: {report:#}");
            assert!(
                report["migration_files"].as_array().unwrap().is_empty(),
                "{import} {ty}: {report:#}"
            );
            assert!(
                !directory.path().join("config/database/migrations").exists(),
                "{import} {ty}"
            );
        }
    }
}

/// Absolute single-segment names are crate identities, not prelude contracts.
#[test]
fn bare_absolute_names_cannot_generate_from_prelude_codecs() {
    for ty in ["::String", "::Option<i32>", "::Vec<i32>", "::i32"] {
        let source = format!("pub struct Row {{ pub value: {ty} }}");
        let (directory, report, code) = audit(&source, &[]);
        assert_eq!(code, 2, "{ty}: {report:#}");
        assert!(
            report["migration_files"].as_array().unwrap().is_empty(),
            "{ty}: {report:#}"
        );
        assert!(
            !directory.path().join("config/database/migrations").exists(),
            "{ty}"
        );
    }
}
