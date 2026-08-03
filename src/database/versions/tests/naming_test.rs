//! Naming rules for versioned schemas and in-flight physical columns.

use crate::database::versions::naming::{
    deletion_name, is_internal, logical_name, temporary_name, versioned_schema,
};

#[test]
fn version_schema_is_schema_and_version() {
    assert_eq!(
        versioned_schema("public", "01_add_users"),
        "public_01_add_users"
    );
}

#[test]
fn temporary_columns_are_prefixed() {
    let physical = temporary_name("zone_id");
    assert_ne!(physical, "zone_id");
    assert!(is_internal(&physical));
    // The logical name must survive the round trip so views project correctly.
    assert_eq!(logical_name(&physical), "zone_id");
}

#[test]
fn deletion_columns_are_prefixed() {
    let physical = deletion_name("zone_id");
    assert!(is_internal(&physical));
    assert_eq!(logical_name(&physical), "zone_id");
}

#[test]
fn ordinary_columns_are_not_internal() {
    assert!(!is_internal("zone_id"));
    assert_eq!(logical_name("zone_id"), "zone_id");
}
