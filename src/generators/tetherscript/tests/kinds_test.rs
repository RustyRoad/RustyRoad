//! Value-kind mapping for arrays, JSON, and enums, plus the kind vocabulary.

use super::support::{enum_type, required, with_enums};
use super::types_test::map;
use crate::database::introspection::Table;
use crate::generators::tetherscript::types::{self, Kind};

#[test]
fn json_and_arrays_get_their_own_kinds() {
    assert_eq!(map("jsonb"), Kind::Json);
    assert_eq!(map("json"), Kind::Json);
    assert_eq!(map("text[]"), Kind::List);
    assert_eq!(map("integer[]"), Kind::List);
    // The catalog spelling must resolve the same way as the format_type spelling.
    assert_eq!(map("_int4"), Kind::List);
}

#[test]
fn an_enum_is_a_string_because_it_arrives_as_its_label() {
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

    assert_eq!(
        types::map(&required("status", "product_status"), &schema),
        Kind::Str
    );
}

#[test]
fn each_kind_names_a_type_of_result_and_a_zero() {
    // The name must match what `type_of` reports, or every generated check fails.
    assert_eq!(Kind::Int.name(), "int");
    assert_eq!(Kind::Str.name(), "str");
    assert_eq!(Kind::List.name(), "list");
    assert_eq!(Kind::Json.name(), "map");

    assert_eq!(Kind::Int.zero(), "0");
    assert_eq!(Kind::Str.zero(), "\"\"");
    assert_eq!(Kind::List.zero(), "[]");
    assert_eq!(Kind::Json.zero(), "map()");
}
