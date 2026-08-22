//! Canonical flat Rust API writer behavior.

use super::support::{schema, scratch};
use crate::generators::rust::write_api;

#[test]
fn canonical_api_rewrites_generated_files_and_preserves_scaffolds() {
    let out = scratch("canonical-api");
    let first = write_api(&out, &schema(), false).expect("first write should succeed");

    assert_eq!(first.len(), 5);
    for name in [
        "models.rs",
        "repositories.rs",
        "procedures.rs",
        "api.rs",
        "mod.rs",
    ] {
        assert!(out.join(name).is_file(), "{name} should be written");
    }

    std::fs::write(out.join("models.rs"), "// stale generated file\n").unwrap();
    std::fs::write(out.join("api.rs"), "// developer-owned composition\n").unwrap();

    write_api(&out, &schema(), false).expect("second write should succeed");

    assert!(std::fs::read_to_string(out.join("models.rs"))
        .unwrap()
        .contains("pub struct Product"));
    assert_eq!(
        std::fs::read_to_string(out.join("api.rs")).unwrap(),
        "// developer-owned composition\n"
    );

    let _ = std::fs::remove_dir_all(&out);
}
