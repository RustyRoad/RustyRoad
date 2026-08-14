//! The module root's imports, exports, and seeded shape.

use super::rendered::file;

#[test]
fn the_module_root_imports_every_crud_module() {
    let root = file("mod.tether");

    for verb in ["create", "read", "update", "delete"] {
        assert!(
            root.contains(&format!("import \"./{verb}.tether\" as {verb}_mod")),
            "missing import for {verb}"
        );
    }
}

#[test]
fn the_module_root_exports_what_it_defines() {
    let root = file("mod.tether");

    // A name is reachable only when exported, so an omission makes the function
    // invisible rather than producing an error at the call site.
    for name in [
        "TABLE", "MODEL", "columns", "new", "validate", "create", "all", "find", "update", "delete",
    ] {
        assert!(
            root.contains(&format!("export {name}\n")),
            "missing export {name}"
        );
    }
}

#[test]
fn new_seeds_every_column_so_no_key_is_absent() {
    let root = file("mod.tether");

    // Required columns get their kind's zero.
    assert!(root.contains("row[\"name\"] = \"\""));
    assert!(root.contains("row[\"active\"] = false"));
    assert!(root.contains("row[\"attributes\"] = []"));
    // Database-assigned and nullable columns stay nil.
    assert!(root.contains("row[\"id\"] = nil"));
    assert!(root.contains("row[\"created_at\"] = nil"));
    assert!(root.contains("row[\"description\"] = nil"));
}
