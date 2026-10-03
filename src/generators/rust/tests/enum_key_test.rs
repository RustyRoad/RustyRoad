//! A table keyed by an enum column must import that enum in its CRUD files.

use super::variant_fixture::{file, keyed, keyed_schema};

#[test]
fn keyed_files_import_the_enum_key_type() {
    let table = keyed();
    let schema = keyed_schema();

    // The key appears in every keyed signature, so the submodules have to import it from the
    // shared module. Importing only the struct leaves it unresolved.
    for name in ["read.rs", "update.rs", "delete.rs"] {
        let contents = file(&table, &schema, name);

        assert!(
            contents.contains("use super::super::enums::NodeKind;"),
            "{name} must import the enum key type:\n{contents}"
        );
    }
}

#[test]
fn the_insert_names_only_the_struct() {
    let create = file(&keyed(), &keyed_schema(), "create.rs");

    // `create` takes a whole row, so the key's type never appears in its signature.
    assert!(
        create.contains("use super::Catalog;"),
        "create should not import the key type:\n{create}"
    );
    assert!(!create.contains("enums::"), "spurious import:\n{create}");
}

#[test]
fn a_scalar_key_imports_nothing_extra() {
    use super::support::products;
    use super::support::schema;

    // `String` and `i32` look like generated types to a naive check; importing them from
    // `super` produced 141 unresolved-import errors on the live schema.
    let read = file(&products(), &schema(), "read.rs");

    assert!(
        read.contains("use super::Product;"),
        "expected a lone struct import:\n{read}"
    );
    assert!(!read.contains("String}"), "String was imported:\n{read}");
    assert!(!read.contains("enums::"), "spurious import:\n{read}");
}
