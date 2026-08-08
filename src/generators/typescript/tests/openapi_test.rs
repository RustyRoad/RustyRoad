//! OpenAPI document structure.

use super::openapi_support::{document, file};

#[test]
fn document_is_valid_json() {
    let parsed = document();

    assert_eq!(parsed["openapi"], "3.1.0");
    assert!(parsed["paths"].is_object());
    assert!(parsed["components"]["schemas"].is_object());
}

#[test]
fn paths_match_the_generated_routes() {
    let parsed = document();
    let paths = parsed["paths"].as_object().unwrap();

    // The client's URLs must match where the Fastify routes are mounted.
    assert!(paths.contains_key("/api/users"));
    assert!(paths.contains_key("/api/users/{id}"));
    assert!(paths.contains_key("/api/posts"));
}

#[test]
fn verbs_sharing_a_url_are_merged() {
    let parsed = document();
    let collection = &parsed["paths"]["/api/users"];
    let single = &parsed["paths"]["/api/users/{id}"];

    // OpenAPI nests verbs under one path key, so they cannot be emitted twice.
    assert!(collection["get"].is_object());
    assert!(collection["post"].is_object());
    assert!(single["get"].is_object());
    assert!(single["patch"].is_object());
    assert!(single["delete"].is_object());
}

#[test]
fn operation_ids_match_the_server() {
    let parsed = document();

    // Hey API derives SDK function names from operationId.
    assert_eq!(
        parsed["paths"]["/api/users"]["get"]["operationId"],
        "listUsers"
    );
    assert_eq!(
        parsed["paths"]["/api/users"]["post"]["operationId"],
        "createUsers"
    );
    assert_eq!(
        parsed["paths"]["/api/users/{id}"]["delete"]["operationId"],
        "deleteUsers"
    );
}

#[test]
fn config_selects_the_flat_sdk_style() {
    let config = file("openapi-ts.config.ts");

    // asClass: false yields `listUsers({...})` rather than a class method.
    assert!(config.contains("\"@hey-api/sdk\", asClass: false"));
    assert!(config.contains("@hey-api/typescript"));
    // The TypeScript 7 incompatibility is worth stating where it will be read.
    assert!(config.contains("TypeScript 5"));
}
