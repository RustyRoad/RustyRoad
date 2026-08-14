//! Enum resolution and the `ts_rs` overrides.

use super::support::{enum_type, from_tables, required, with_enums};
use crate::database::introspection::Table;
use crate::generators::rust::types;

#[test]
fn an_enum_column_resolves_to_its_generated_type() {
    let schema = with_enums(
        vec![Table {
            name: "products".to_string(),
            columns: vec![required("status", "product_status")],
            primary_key: Vec::new(),
            foreign_keys: Vec::new(),
            uniques: Vec::new(),
            indexes: Vec::new(),
            view: false,
        }],
        vec![enum_type("product_status", &["active", "archived"])],
    );

    // Without the enum lookup this falls through to String and the variants are lost.
    let mapped = types::map(&required("status", "product_status"), &schema);
    assert_eq!(mapped.rust, "ProductStatus");
}

#[test]
fn types_ts_rs_cannot_derive_declare_an_override() {
    let schema = from_tables(Vec::new());

    assert_eq!(
        types::map(&required("at", "timestamp"), &schema).ts,
        Some("string")
    );
    assert_eq!(
        types::map(&required("payload", "jsonb"), &schema).ts,
        Some("any")
    );
    // A plain scalar needs no override; emitting one would be noise.
    assert_eq!(types::map(&required("id", "integer"), &schema).ts, None);
}
