//! Enum columns must resolve to a type that actually exists.
//!
//! The type is declared once in the shared `enums` module and imported by each model that
//! references it — per-model copies were incompatible Rust types.

use super::enum_fixture::{root, shared_enums};

#[test]
fn an_enum_column_gets_a_type_that_is_actually_declared() {
    // The field names the type...
    let root = root();
    assert!(
        root.contains("pub status: ProductStatus,"),
        "field should use the enum type: {root}"
    );
    // ...imported from the shared module...
    assert!(
        root.contains("use super::enums::ProductStatus;"),
        "the model must import its enum:\n{root}"
    );

    // ...and the shared module must declare it, or the generated crate does not compile.
    let shared = shared_enums();
    assert!(
        shared.contains("pub enum ProductStatus"),
        "ProductStatus is referenced but never declared:\n{shared}"
    );
}

#[test]
fn a_required_enum_field_has_a_default_it_can_actually_use() {
    // `Default` seeds the field with `Default::default()`, so the enum has to implement it.
    // Without the derive the model compiles everywhere except its own Default impl.
    let root = root();
    assert!(
        root.contains("status: Default::default()"),
        "expected the default to delegate: {root}"
    );

    let shared = shared_enums();
    assert!(
        shared.contains("Default,"),
        "ProductStatus must derive Default:\n{shared}"
    );
    // A derived Default needs one variant marked, or the derive itself fails.
    assert!(
        shared.contains("#[default]"),
        "a variant must be marked #[default]:\n{shared}"
    );
}

#[test]
fn a_variant_keeps_the_label_postgres_stores() {
    let shared = shared_enums();

    assert!(shared.contains("#[sqlx(type_name = \"product_status\")]"));
    // sqlx and serde both derive the wire value from the identifier, so a converted variant
    // needs telling or it round-trips as the wrong label.
    assert!(shared.contains("#[sqlx(rename = \"archived\")]"));
    assert!(shared.contains("#[serde(rename = \"archived\")]"));
}
