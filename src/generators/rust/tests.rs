//! Rust generator coverage.

use super::{models, procedures, repositories, write, Outputs};
use crate::database::introspection::{Column, Enum, Schema, Table};
use crate::generators::report::Outcome;

fn schema() -> Schema {
    Schema {
        enums: vec![Enum {
            name: "user_role".to_string(),
            values: vec![
                "admin".to_string(),
                "read_only".to_string(),
                "read-only".to_string(),
            ],
        }],
        tables: vec![Table {
            name: "users".to_string(),
            columns: vec![
                Column {
                    name: "id".to_string(),
                    sql_type: "integer".to_string(),
                    nullable: false,
                    default: Some("nextval('users_id_seq'::regclass)".to_string()),
                    auto_increment: true,
                },
                Column {
                    name: "email_address".to_string(),
                    sql_type: "character varying(255)".to_string(),
                    nullable: false,
                    default: None,
                    auto_increment: false,
                },
                Column {
                    name: "display_name".to_string(),
                    sql_type: "text".to_string(),
                    nullable: true,
                    default: None,
                    auto_increment: false,
                },
                Column {
                    name: "role".to_string(),
                    sql_type: "user_role".to_string(),
                    nullable: false,
                    default: Some("'read-only'::user_role".to_string()),
                    auto_increment: false,
                },
                Column {
                    name: "account_id".to_string(),
                    sql_type: "uuid".to_string(),
                    nullable: false,
                    default: None,
                    auto_increment: false,
                },
                Column {
                    name: "settings".to_string(),
                    sql_type: "jsonb".to_string(),
                    nullable: false,
                    default: None,
                    auto_increment: false,
                },
                Column {
                    name: "balance".to_string(),
                    sql_type: "numeric(12, 2)".to_string(),
                    nullable: false,
                    default: None,
                    auto_increment: false,
                },
                Column {
                    name: "created_at".to_string(),
                    sql_type: "timestamp with time zone".to_string(),
                    nullable: false,
                    default: None,
                    auto_increment: false,
                },
                Column {
                    name: "network".to_string(),
                    sql_type: "inet".to_string(),
                    nullable: true,
                    default: None,
                    auto_increment: false,
                },
                Column {
                    name: "hardware_address".to_string(),
                    sql_type: "macaddr".to_string(),
                    nullable: true,
                    default: None,
                    auto_increment: false,
                },
                Column {
                    name: "session_length".to_string(),
                    sql_type: "interval".to_string(),
                    nullable: true,
                    default: None,
                    auto_increment: false,
                },
                Column {
                    name: "tags".to_string(),
                    sql_type: "text[]".to_string(),
                    nullable: false,
                    default: None,
                    auto_increment: false,
                },
            ],
            primary_key: vec!["id".to_string()],
            foreign_keys: Vec::new(),
            uniques: Vec::new(),
            indexes: Vec::new(),
        }],
    }
}

#[test]
fn models_derive_rows_and_separate_create_and_patch_inputs() {
    let rust = models::render(&schema());

    assert!(rust.contains("pub struct Users"));
    assert!(rust.contains("pub struct NewUsers"));
    assert!(rust.contains("pub struct PatchUsers"));
    assert!(!section(&rust, "pub struct NewUsers", "pub struct PatchUsers").contains("pub id:"));
    let patch = section(&rust, "pub struct PatchUsers", "\n}\n");
    assert!(!patch.contains("pub id:"));
    assert!(patch.contains("pub display_name: Option<Option<String>>"));
    assert!(rust.contains("pub display_name: Option<String>"));
    assert!(rust.contains("pub enum UserRole"));
    assert!(rust.contains("#[sqlx(rename = \"read-only\")]"));
    assert!(rust.contains("pub network: Option<sqlx::types::ipnetwork::IpNetwork>"));
}

#[test]
fn colliding_enum_labels_receive_unique_rust_variants() {
    let rust = models::render(&schema());

    assert!(rust.contains(
        "#[sqlx(rename = \"read_only\")]\n    #[serde(rename = \"read_only\")]\n    ReadOnly,"
    ));
    assert!(rust.contains(
        "#[sqlx(rename = \"read-only\")]\n    #[serde(rename = \"read-only\")]\n    ReadOnlyVariant2,"
    ));
}

#[test]
fn repositories_generate_typed_crud_with_bound_values() {
    let rust = repositories::render(&schema());

    assert!(rust.contains("pub async fn list_users"));
    assert!(rust.contains("pub async fn find_users"));
    assert!(rust.contains("pub async fn create_users"));
    assert!(rust.contains("pub async fn update_users"));
    assert!(rust.contains("pub async fn delete_users"));
    assert!(rust.contains("push_bind"));
    assert!(!rust.contains("format!(\"INSERT"));
}

#[test]
fn procedures_expose_actix_routes_for_all_five_operations() {
    let rust = procedures::render(&schema());

    assert!(rust.contains("web::scope(\"/api\")"));
    assert!(rust.contains("web::resource(\"/users\")"));
    assert!(rust.contains("web::resource(\"/users/{id}\")"));
    assert!(rust.contains("web::get().to(list_users)"));
    assert!(rust.contains("web::post().to(create_users)"));
    assert!(rust.contains("web::patch().to(update_users)"));
    assert!(rust.contains("web::delete().to(delete_users)"));
}

#[test]
fn every_generated_rust_file_is_valid_rust_syntax() {
    for (name, contents) in [
        ("models.rs", models::render(&schema())),
        ("repositories.rs", repositories::render(&schema())),
        ("procedures.rs", procedures::render(&schema())),
        ("api.rs", procedures::composition()),
        ("mod.rs", procedures::module()),
    ] {
        syn::parse_file(&contents).unwrap_or_else(|error| panic!("{name} did not parse: {error}"));
    }
}

#[test]
fn writer_preserves_developer_owned_composition_unless_forced() {
    let out = scratch("ownership");
    write(&out, &schema(), Outputs::all(), false).expect("initial write should succeed");
    std::fs::write(out.join("api.rs"), "// mine\n").expect("custom api should write");

    let report =
        write(&out, &schema(), Outputs::all(), false).expect("second write should succeed");
    assert!(report
        .outcomes
        .iter()
        .any(|outcome| matches!(outcome, Outcome::Preserved(path) if path.ends_with("api.rs"))));
    assert_eq!(
        std::fs::read_to_string(out.join("api.rs")).expect("api should read"),
        "// mine\n"
    );

    write(&out, &schema(), Outputs::all(), true).expect("forced write should succeed");
    assert_ne!(
        std::fs::read_to_string(out.join("api.rs")).expect("api should read"),
        "// mine\n"
    );
}

#[test]
fn schema_only_emits_only_models() {
    let out = scratch("schema-only");
    let report =
        write(&out, &schema(), Outputs::schema_only(), false).expect("write should succeed");

    assert_eq!(report.outcomes.len(), 1);
    assert!(out.join("models.rs").exists());
    assert!(!out.join("procedures.rs").exists());
}

fn section<'a>(contents: &'a str, start: &str, end: &str) -> &'a str {
    let contents = &contents[contents.find(start).expect("start should exist")..];
    &contents[..contents.find(end).expect("end should exist")]
}

fn scratch(label: &str) -> std::path::PathBuf {
    let path = std::env::temp_dir().join(format!(
        "rustyroad-rust-generator-{label}-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&path);
    std::fs::create_dir_all(&path).expect("scratch directory should exist");
    path
}
