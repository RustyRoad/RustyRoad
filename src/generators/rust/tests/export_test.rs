//! How the parent declares and re-exports what was generated.

use super::enum_fixture;
use super::support::scratch;
use crate::generators::rust::write;

#[test]
fn the_shared_enums_module_is_declared_and_globbed() {
    let out = scratch("enum-export");
    write(&out, &enum_fixture::schema(), false).expect("write should succeed");
    let parent = std::fs::read_to_string(out.join("mod.rs")).unwrap();

    // One declaration per Postgres type, reachable at models::ProductStatus where a caller
    // holding a Product looks for it.
    assert!(
        parent.contains("pub mod enums;"),
        "the shared module must be declared: {parent}"
    );
    assert!(
        parent.contains("pub use enums::*;"),
        "the shared module carries the glob: {parent}"
    );

    let _ = std::fs::remove_dir_all(&out);
}

#[test]
fn a_model_is_re_exported_by_name_never_by_glob() {
    let out = scratch("model-export");
    write(&out, &enum_fixture::schema(), false).expect("write should succeed");
    let parent = std::fs::read_to_string(out.join("mod.rs")).unwrap();

    // The model module exports exactly one name now that its enums live in the shared
    // module; a glob here is what produced eighteen ambiguity warnings.
    assert!(
        parent.contains("pub use product::Product;"),
        "expected a named re-export: {parent}"
    );
    assert!(
        !parent.contains("pub use product::*;"),
        "a model must not glob: {parent}"
    );

    let _ = std::fs::remove_dir_all(&out);
}

#[test]
fn a_model_without_enums_is_unchanged() {
    let out = scratch("plain-export");
    write(&out, &super::support::schema(), false).expect("write should succeed");
    let parent = std::fs::read_to_string(out.join("mod.rs")).unwrap();

    assert!(
        parent.contains("pub use product::Product;"),
        "expected a named re-export: {parent}"
    );
    // No enum anywhere in the schema, so no shared module is written or declared.
    assert!(!parent.contains("pub mod enums;"), "{parent}");
    assert!(!out.join("enums").exists());

    let _ = std::fs::remove_dir_all(&out);
}
