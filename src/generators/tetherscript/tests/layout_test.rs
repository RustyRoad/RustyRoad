//! Folder layout and file ownership.

use super::support::{schema, scratch};
use crate::generators::tetherscript::write;

#[test]
fn writing_lays_out_one_folder_per_table() {
    let out = scratch("layout");
    let layout = write(&out, &schema(), false).expect("write should succeed");

    for name in [
        "mod.tether",
        "create.tether",
        "read.tether",
        "update.tether",
        "delete.tether",
    ] {
        assert!(out.join("product").join(name).exists(), "missing {name}");
    }
    assert!(out.join("mod.tether").exists());
    assert!(layout.undeclared.is_empty());

    let _ = std::fs::remove_dir_all(&out);
}

#[test]
fn a_model_the_preserved_parent_does_not_import_is_reported() {
    let out = scratch("undeclared");
    write(&out, &schema(), false).expect("first run should succeed");

    // A parent that no longer imports the model: its files exist but are never loaded,
    // and nothing errors, so the run has to say so.
    std::fs::write(out.join("mod.tether"), "// mine\n").unwrap();

    let layout = write(&out, &schema(), false).expect("second run should succeed");

    assert_eq!(layout.undeclared, vec!["product".to_string()]);

    let _ = std::fs::remove_dir_all(&out);
}

#[test]
fn hand_written_models_survive_a_second_run() {
    let out = scratch("preserve");
    write(&out, &schema(), false).expect("first run should succeed");

    let root = out.join("mod.tether");
    let edited = "import \"./product/mod.tether\" as product\n\
                  import \"./billing/mod.tether\" as billing\n\n\
                  export product\nexport billing\n";
    std::fs::write(&root, edited).unwrap();

    let layout = write(&out, &schema(), false).expect("second run should succeed");

    // Rewriting this file would delete the billing import.
    assert_eq!(std::fs::read_to_string(&root).unwrap(), edited);
    assert!(layout.undeclared.is_empty());

    let _ = std::fs::remove_dir_all(&out);
}
