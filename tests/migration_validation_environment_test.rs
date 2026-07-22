#[path = "support/migration_validation_environment.rs"]
mod support;
use support::{project, validate};

#[test]
fn default_and_dev_use_default_config_without_touching_database() {
    let root = tempfile::tempdir().unwrap();
    project(root.path(), "rustyroad.toml", "configured_dev");
    for environment in [None, Some("dev")] {
        let output = validate(root.path(), environment);
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    assert!(!root.path().join("configured_dev.db").exists());
}

#[test]
fn prod_uses_prod_config_without_touching_configured_database() {
    let root = tempfile::tempdir().unwrap();
    project(root.path(), "rustyroad.prod.toml", "configured_prod");
    let output = validate(root.path(), Some("prod"));
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(!root.path().join("configured_prod.db").exists());
}
