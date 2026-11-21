//! Shared layout fixtures; no model generation or database connection is performed.

use super::model_paths;

#[test]
fn generator_and_auditor_share_default_and_workspace_locations() {
    let dir = tempfile::tempdir().unwrap();
    assert_eq!(
        model_paths::model_file(dir.path(), "row").unwrap(),
        dir.path().join("src/models/row.rs")
    );
    assert_eq!(
        model_paths::rust_output(dir.path()).unwrap(),
        dir.path().join("src/db")
    );
    for path in ["crates/models/src", "crates/db/src"] {
        std::fs::create_dir_all(dir.path().join(path)).unwrap();
    }
    let layout = model_paths::resolve(dir.path()).unwrap();
    let file = model_paths::model_file(dir.path(), "row").unwrap();
    assert_eq!(file.parent().unwrap(), layout.source);
    assert_eq!(
        model_paths::rust_output(dir.path()).unwrap(),
        dir.path().join("crates/db/src")
    );
}

#[test]
fn configured_destinations_are_shared_and_escape_names_are_rejected() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("rustyroad.toml"),
        "[models]\nsource='domain/models'\ngenerated='storage/models.rs'\n",
    )
    .unwrap();
    let layout = model_paths::resolve(dir.path()).unwrap();
    assert_eq!(
        model_paths::model_file(dir.path(), "row")
            .unwrap()
            .parent()
            .unwrap(),
        layout.source
    );
    assert_eq!(
        model_paths::rust_output(dir.path()).unwrap(),
        dir.path().join("storage")
    );
    assert!(model_paths::model_file(dir.path(), "../../other").is_err());
}

#[test]
fn ambiguous_inventory_requires_config_or_explicit_override() {
    let dir = tempfile::tempdir().unwrap();
    for path in ["crates/db/src", "src/db"] {
        std::fs::create_dir_all(dir.path().join(path)).unwrap();
        std::fs::write(dir.path().join(path).join("models.rs"), "").unwrap();
    }
    assert!(model_paths::resolve(dir.path()).is_err());
    let explicit =
        model_paths::resolve_with(dir.path(), Some(std::path::Path::new("src/db/models.rs")))
            .unwrap();
    assert_eq!(
        explicit.generated.unwrap(),
        dir.path().join("src/db/models.rs")
    );
}
