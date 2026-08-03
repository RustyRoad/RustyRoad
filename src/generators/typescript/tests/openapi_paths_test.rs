//! Request bodies, responses, and path parameters in the OpenAPI document.

use super::openapi_support::document;

#[test]
fn bodies_and_responses_reference_components() {
    let parsed = document();
    let post = &parsed["paths"]["/api/users"]["post"];

    let body = &post["requestBody"]["content"]["application/json"]["schema"]["$ref"];
    assert_eq!(body, "#/components/schemas/NewUsers");

    let created = &post["responses"]["201"]["content"]["application/json"]["schema"]["$ref"];
    assert_eq!(created, "#/components/schemas/Users");
}

#[test]
fn patch_bodies_reference_the_partial_schema() {
    let parsed = document();
    let body = &parsed["paths"]["/api/users/{id}"]["patch"]["requestBody"]["content"]
        ["application/json"]["schema"]["$ref"];

    assert_eq!(body, "#/components/schemas/PatchUsers");
}

#[test]
fn collections_are_typed_as_arrays() {
    let parsed = document();
    let schema = &parsed["paths"]["/api/users"]["get"]["responses"]["200"]["content"]
        ["application/json"]["schema"];

    assert_eq!(schema["type"], "array");
    assert_eq!(schema["items"]["$ref"], "#/components/schemas/Users");
}

#[test]
fn path_parameters_carry_their_type() {
    let parsed = document();
    let parameter = &parsed["paths"]["/api/users/{id}"]["get"]["parameters"][0];

    assert_eq!(parameter["name"], "id");
    assert_eq!(parameter["in"], "path");
    assert_eq!(parameter["required"], true);
    // users.id is serial, so the client sends a number.
    assert_eq!(parameter["schema"]["type"], "integer");
}

#[test]
fn missing_rows_declare_a_404() {
    let parsed = document();
    let error = &parsed["paths"]["/api/users/{id}"]["get"]["responses"]["404"];

    assert_eq!(
        error["content"]["application/json"]["schema"]["$ref"],
        "#/components/schemas/ApiError"
    );
}

#[test]
fn deletes_declare_a_bodyless_204() {
    let parsed = document();
    let response = &parsed["paths"]["/api/users/{id}"]["delete"]["responses"]["204"];

    assert!(response["description"].is_string());
    assert!(response.get("content").is_none());
}
