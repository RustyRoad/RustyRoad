//! OpenAPI component schemas.

use super::openapi_support::document;

#[test]
fn three_variants_are_emitted_per_table() {
    let parsed = document();
    let schemas = &parsed["components"]["schemas"];

    // Select, insert, and patch differ in required fields and generated columns.
    assert!(schemas["Users"].is_object());
    assert!(schemas["NewUsers"].is_object());
    assert!(schemas["PatchUsers"].is_object());
}

#[test]
fn generated_keys_are_absent_from_input_schemas() {
    let parsed = document();
    let insert = &parsed["components"]["schemas"]["NewUsers"]["properties"];

    // A serial key is assigned by the database, so a client cannot supply it.
    assert!(insert.get("id").is_none());
    assert!(insert.get("emailAddress").is_some());
}

#[test]
fn nullable_columns_become_type_unions() {
    let parsed = document();
    let display = &parsed["components"]["schemas"]["Users"]["properties"]["displayName"];

    // The client should see `T | null`, not an omitted optional.
    assert_eq!(display["type"][0], "string");
    assert_eq!(display["type"][1], "null");
}

#[test]
fn types_and_formats_are_declared() {
    let parsed = document();
    let properties = &parsed["components"]["schemas"]["Users"]["properties"];

    assert_eq!(properties["isActive"]["type"], "boolean");
    assert_eq!(properties["id"]["type"], "integer");
    // numeric is a string to avoid float precision loss, matching Drizzle.
    assert_eq!(properties["balance"]["type"][0], "string");
    assert_eq!(properties["externalId"]["format"], "uuid");
    assert_eq!(properties["createdAt"]["format"], "date-time");
}

#[test]
fn defaulted_columns_are_not_required_on_insert() {
    let parsed = document();
    let required = parsed["components"]["schemas"]["NewUsers"]["required"]
        .as_array()
        .expect("required list");

    assert!(required.iter().any(|value| value == "emailAddress"));
    // is_active has a default, so omitting it is valid.
    assert!(!required.iter().any(|value| value == "isActive"));
}

#[test]
fn patch_requires_nothing() {
    let parsed = document();

    // A partial update may carry any subset of columns.
    assert!(parsed["components"]["schemas"]["PatchUsers"]
        .get("required")
        .is_none());
}
