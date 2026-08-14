//! Ownership: derived files are rewritten, the parent module is yours.

use super::support::{schema, scratch};
use crate::generators::rust::write;

#[test]
fn hand_written_models_survive_a_second_run() {
    let out = scratch("preserve");
    write(&out, &schema(), false).expect("first run should succeed");

    // Stand in for a project declaring a hand-written model beside the generated one.
    let root = out.join("mod.rs");
    let edited = "pub mod product;\npub mod billing;\n\npub use product::Product;\n";
    std::fs::write(&root, edited).unwrap();

    let layout = write(&out, &schema(), false).expect("second run should succeed");

    // Rewriting this file would delete the `billing` declaration.
    assert_eq!(std::fs::read_to_string(&root).unwrap(), edited);
    assert!(layout
        .outcomes
        .iter()
        .any(|outcome| outcome.is_preserved() && outcome.path().ends_with("mod.rs")));

    let _ = std::fs::remove_dir_all(&out);
}

#[test]
fn derived_files_are_always_rewritten() {
    let out = scratch("derived");
    write(&out, &schema(), false).expect("first run should succeed");

    let create = out.join("product").join("create.rs");
    std::fs::write(&create, "// stale\n").unwrap();

    write(&out, &schema(), false).expect("second run should succeed");

    // Editing a derived file is always a mistake, so the next run must revert it.
    assert!(std::fs::read_to_string(&create)
        .unwrap()
        .contains("INSERT INTO products"));

    let _ = std::fs::remove_dir_all(&out);
}

#[test]
fn a_model_the_preserved_parent_does_not_declare_is_reported() {
    let out = scratch("undeclared");
    write(&out, &schema(), false).expect("first run should succeed");

    // A parent that no longer declares the model: its files exist but are not compiled,
    // and nothing errors, so the run has to say so.
    std::fs::write(out.join("mod.rs"), "// mine\n").unwrap();

    let layout = write(&out, &schema(), false).expect("second run should succeed");

    assert_eq!(layout.undeclared, vec!["product".to_string()]);

    let _ = std::fs::remove_dir_all(&out);
}
