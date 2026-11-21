//! Mocked-local defaults/config resolution checks. No test connects to PostgreSQL.

use serde_json::Value;
use std::{
    path::Path,
    process::{Command, Output},
};

/// Workspace-style fixture with deliberately malformed non-model Rust sources.
fn fixture() -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    for folder in [
        "crates/models/src",
        "crates/db/src",
        "src/models",
        "crates/controllers/src",
    ] {
        std::fs::create_dir_all(dir.path().join(folder)).unwrap();
    }
    std::fs::write(
        dir.path().join("crates/models/src/row.rs"),
        "pub struct Row { pub id: i32 }",
    )
    .unwrap();
    std::fs::write(
        dir.path().join("crates/db/src/models.rs"),
        "pub struct Rows { pub id: i32 }",
    )
    .unwrap();
    std::fs::write(
        dir.path().join("src/models/mod.rs"),
        "pub broken legacy facade",
    )
    .unwrap();
    std::fs::write(
        dir.path().join("crates/controllers/src/lib.rs"),
        "pub broken controller",
    )
    .unwrap();
    for file in [
        "rustyroad.toml",
        "rustyroad.staging.toml",
        "rustyroad.prod.toml",
    ] {
        std::fs::write(
            dir.path().join(file),
            "[database]\ndatabase_type = 'mysql'\n",
        )
        .unwrap();
    }
    dir
}

/// Remove ambient connection settings so a developer's credentials cannot affect fixtures.
fn command(root: &Path) -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_rustyroad"));
    for key in [
        "DATABASE_URL",
        "DATABASE_HOST",
        "DATABASE_PORT",
        "DATABASE_USER",
        "DATABASE_PASSWORD",
        "DATABASE_NAME",
        "RUSTYROAD_CONFIG_PATH",
        "ENVIRONMENT",
        "ENV",
    ] {
        command.env_remove(key);
    }
    command
        .current_dir(root)
        .env("PATH", root.join("empty-tool-path"))
        .args(["--format", "json", "model", "audit"]);
    command
}

/// Decode a successful report with its diagnostic in assertion output.
fn report(output: Output) -> Value {
    assert!(
        output.status.success(),
        "stdout={} stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).unwrap()
}

#[test]
fn default_sources_match_workspace_model_generator_not_all_crates() {
    let dir = fixture();
    let report = report(command(dir.path()).arg("--offline").output().unwrap());
    assert_eq!(report["discovered"], 1);
    assert_eq!(
        report["context"]["sources"][0],
        dir.path().join("crates/models/src").to_str().unwrap()
    );
    assert_eq!(report["context"]["catalog_mode"], "offline");
    assert!(report["diagnostics"].as_array().unwrap().is_empty());
    let broad = command(dir.path())
        .args(["--offline", "--all-sources"])
        .output()
        .unwrap();
    assert_eq!(broad.status.code(), Some(2));
}

#[test]
fn environment_flag_and_variable_precedence_do_not_mutate_process_state() {
    let dir = fixture();
    let flag = report(
        command(dir.path())
            .env("ENVIRONMENT", "staging")
            .env("ENV", "dev")
            .args(["--offline", "--env", "prod"])
            .output()
            .unwrap(),
    );
    assert_eq!(flag["context"]["environment"], "prod");
    assert!(flag["context"]["config_file"]
        .as_str()
        .unwrap()
        .ends_with("rustyroad.prod.toml"));
    let environment = report(
        command(dir.path())
            .env("ENVIRONMENT", "staging")
            .env("ENV", "prod")
            .arg("--offline")
            .output()
            .unwrap(),
    );
    assert_eq!(environment["context"]["environment"], "staging");
    let alias = report(
        command(dir.path())
            .env("ENV", "prod")
            .arg("--offline")
            .output()
            .unwrap(),
    );
    assert_eq!(alias["context"]["environment"], "prod");
}

#[test]
fn config_auto_enables_catalog_and_missing_environment_does_not_fall_back() {
    let dir = fixture();
    // Unsupported backend errors before connecting; proves config detection without --database.
    let detected = command(dir.path()).output().unwrap();
    assert_eq!(detected.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&detected.stdout).contains("requires database_type = postgres"));
    let missing = command(dir.path())
        .args(["--env", "missing"])
        .output()
        .unwrap();
    assert_eq!(missing.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&missing.stdout).contains("no fallback"));
}

#[test]
fn config_and_generated_overrides_are_explicit_and_credentials_stay_redacted() {
    let dir = fixture();
    std::fs::write(
        dir.path().join("alternate.toml"),
        "[database]\ndatabase_type='postgres'\ndatabase_password='fixture-private-value'\n",
    )
    .unwrap();
    let output = command(dir.path())
        .args(["--config", "alternate.toml"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    assert!(!String::from_utf8_lossy(&output.stdout).contains("fixture-private-value"));
    let selected = report(
        command(dir.path())
            .env("RUSTYROAD_CONFIG_PATH", "rustyroad.prod.toml")
            .args(["--offline", "--config", "alternate.toml"])
            .output()
            .unwrap(),
    );
    assert!(selected["context"]["config_file"]
        .as_str()
        .unwrap()
        .ends_with("alternate.toml"));
}

#[test]
fn configuration_sets_shared_source_and_generated_locations() {
    let dir = fixture();
    std::fs::create_dir_all(dir.path().join("domain")).unwrap();
    std::fs::create_dir_all(dir.path().join("data")).unwrap();
    std::fs::write(
        dir.path().join("domain/item.rs"),
        "pub struct Item { pub id: i32 }",
    )
    .unwrap();
    std::fs::write(
        dir.path().join("data/models.rs"),
        "pub struct Items { pub id: i32 }",
    )
    .unwrap();
    std::fs::write(
        dir.path().join("rustyroad.toml"),
        "[models]\nsource='domain'\ngenerated='data/models.rs'\n",
    )
    .unwrap();
    let report = report(command(dir.path()).arg("--offline").output().unwrap());
    assert_eq!(report["candidates"][0]["model"]["name"], "Item");
    assert_eq!(
        report["generated"],
        dir.path().join("data/models.rs").to_str().unwrap()
    );
}

#[test]
fn project_root_is_found_from_a_model_subdirectory() {
    let dir = fixture();
    let report = report(
        command(&dir.path().join("crates/models/src"))
            .arg("--offline")
            .output()
            .unwrap(),
    );
    assert_eq!(report["root"], dir.path().to_str().unwrap());
}
