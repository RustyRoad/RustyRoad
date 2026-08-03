//! Enum imports, chaining, and OpenAPI output.

use super::enums_test::orders;
use crate::generators::typescript::heyapi::render as heyapi_render;
use crate::generators::typescript::schema::render;
use crate::generators::typescript::Casing;

#[test]
fn pg_enum_is_imported_only_when_used() {
    let with = render(&orders(), Casing::Camel);
    assert!(with.lines().next().unwrap().contains("pgEnum"));

    let without = render(&super::support::schema(), Casing::Camel);
    assert!(!without.lines().next().unwrap().contains("pgEnum"));
}

#[test]
fn nullability_and_defaults_still_chain() {
    let ts = render(&orders(), Casing::Camel);

    assert!(ts.contains(r#"status: orderStatus("status").notNull()"#));
    // The enum-cast default stays SQL, since it is not a plain literal.
    assert!(ts.contains("sql`'pending'::order_status`"));
}

#[test]
fn allowed_values_reach_the_openapi_document() {
    let document = heyapi_render(&orders(), Casing::Camel, "/api")
        .into_iter()
        .find(|file| file.name == "openapi.json")
        .expect("document rendered")
        .contents;

    let parsed: serde_json::Value = serde_json::from_str(&document).expect("valid JSON");
    let status = &parsed["components"]["schemas"]["Orders"]["properties"]["status"];

    // The values are the useful part of an enum, so a client must see them.
    assert_eq!(status["type"], "string");
    assert_eq!(status["enum"][0], "pending");
    assert_eq!(status["enum"][2], "delivered");
}

#[test]
fn a_nullable_enum_permits_null() {
    let document = heyapi_render(&orders(), Casing::Camel, "/api")
        .into_iter()
        .find(|file| file.name == "openapi.json")
        .expect("document rendered")
        .contents;

    let parsed: serde_json::Value = serde_json::from_str(&document).expect("valid JSON");
    let fulfilment = &parsed["components"]["schemas"]["Orders"]["properties"]["fulfilment"];

    // A validator checks membership before nullability, so null must be a value too.
    assert_eq!(fulfilment["type"][1], "null");
    assert!(fulfilment["enum"]
        .as_array()
        .expect("enum list")
        .iter()
        .any(|value| value.is_null()));
}
