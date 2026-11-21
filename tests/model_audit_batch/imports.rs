//! Mocked local CLI proof: external imports must not borrow local SQLx contracts.
use super::fixture::audit;

const SOURCE: &str = r#"
mod codec {
    #[derive(sqlx::Type)]
    #[sqlx(type_name = "state")]
    pub enum State { Ready }
}
pub struct Row { pub id: i32, pub state: Option<State> }
"#;

#[test]
fn absolute_imports_block_generation_despite_matching_local_codecs() {
    for import in [
        "use ::codec::State;",
        "use ::codec::{State};",
        "use ::codec::{State as Status}; type State = Status;",
        "use ::codec::{self as external}; use external::State;",
    ] {
        let source = format!("{import}\n{SOURCE}");
        let (directory, report, code) = audit(&source, &[]);
        assert_eq!(code, 2, "{import}: {report:#}");
        assert!(report["migration_files"].as_array().unwrap().is_empty());
        let row = report["candidates"]
            .as_array()
            .unwrap()
            .iter()
            .find(|candidate| candidate["table_audit"]["table"] == "rows")
            .unwrap_or_else(|| panic!("Row missing: {report:#}"));
        assert!(
            row["table_audit"]["blockers"]
                .as_array()
                .unwrap()
                .iter()
                .any(|blocker| blocker
                    .as_str()
                    .unwrap()
                    .contains("no PostgreSQL storage mapping")),
            "{report:#}"
        );
        assert!(!directory.path().join("config/database/migrations").exists());
    }
}

#[test]
fn wildcard_imports_cannot_authorize_builtin_column_migrations() {
    for import in ["use unknown::*;", "use unknown::{nested::*};"] {
        let source = format!("{import} pub struct Row {{ pub id: i32, pub value: String }}");
        let (directory, report, code) = audit(&source, &[]);
        assert_eq!(code, 2, "{import}: {report:#}");
        assert!(report["migration_files"].as_array().unwrap().is_empty());
        assert!(!directory.path().join("config/database/migrations").exists());
    }
}

#[test]
fn relative_imports_keep_their_local_codec_and_generate_a_draft() {
    for import in [
        "use codec::State;",
        "use codec::{State};",
        "use codec::{self as local}; use local::State;",
    ] {
        let source = format!("{import}\n{SOURCE}");
        let (_directory, report, code) = audit(&source, &[]);
        assert_eq!(code, 0, "{import}: {report:#}");
        let migrations = report["migration_files"].as_array().unwrap();
        assert_eq!(migrations.len(), 1, "{report:#}");
        assert_eq!(migrations[0]["state"], "generated");
        let path = std::path::Path::new(migrations[0]["directory"].as_str().unwrap());
        let up = std::fs::read_to_string(path.join("up.sql")).unwrap();
        assert!(up.contains("\"state\""), "{up}");
    }
}
