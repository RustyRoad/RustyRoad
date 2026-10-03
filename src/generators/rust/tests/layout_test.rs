//! Folder layout, and forcing the parent module.

use super::support::{schema, scratch};
use crate::generators::rust::write;

#[test]
fn writing_lays_out_one_folder_per_table() {
    let out = scratch("layout");
    let layout = write(&out, &schema(), false).expect("write should succeed");

    for name in ["mod.rs", "create.rs", "read.rs", "update.rs", "delete.rs"] {
        assert!(out.join("product").join(name).exists(), "missing {name}");
    }
    assert!(out.join("mod.rs").exists());
    assert!(layout.undeclared.is_empty());

    let _ = std::fs::remove_dir_all(&out);
}

#[test]
fn force_overwrites_the_parent_module() {
    let out = scratch("force");
    write(&out, &schema(), false).expect("first run should succeed");
    std::fs::write(out.join("mod.rs"), "// mine\n").unwrap();

    write(&out, &schema(), true).expect("forced run should succeed");

    assert!(std::fs::read_to_string(out.join("mod.rs"))
        .unwrap()
        .contains("pub mod product;"));

    let _ = std::fs::remove_dir_all(&out);
}
