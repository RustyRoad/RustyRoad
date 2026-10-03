//! Server wiring and key coercion for the oRPC router.

use super::support::{column, enum_key_schema, from_tables, schema};
use crate::database::introspection::Table;
use crate::generators::typescript::orpc::{openapi_script, render, server};
use crate::generators::typescript::Casing;

#[test]
fn numeric_keys_are_coerced() {
    let ts = render(&schema(), Casing::Camel, "/api");

    // A REST path parameter arrives as a string, so coercion is required even
    // though a direct RPC call may already pass a number.
    assert!(ts.contains("z.coerce.number().int()"));
}

#[test]
fn nonnumeric_keys_reuse_the_derived_column_schema() {
    let table = Table {
        name: "sessions".to_string(),
        columns: vec![column("id", "uuid")],
        primary_key: vec!["id".to_string()],
        foreign_keys: Vec::new(),
        uniques: Vec::new(),
        indexes: Vec::new(),
        view: false,
    };

    let ts = render(&from_tables(vec![table]), Casing::Camel, "/api");
    assert!(ts.contains(r#"z.object({ id: sessionsSelectSchema.shape["id"] })"#));
    assert!(!ts.contains("z.coerce"));
}

#[test]
fn enum_keys_keep_the_enum_schema_in_every_keyed_router_input() {
    let ts = render(&enum_key_schema(), Casing::Camel, "/api");
    let key_schema =
        r#"z.object({ id: campaignWorkflowNodeBindingsSelectSchema.shape["nodeKind"] })"#;

    // get, update, and delete all use the exact enum schema. This keeps input.id
    // assignable to the indexed row type expected by each repository method.
    assert_eq!(ts.matches(key_schema).count(), 3);
    assert!(!ts.contains("z.object({ id: z.string() })"));
}

#[test]
fn server_mounts_both_handlers() {
    let ts = server();

    // The same router serves the typed client over RPC and REST over the declared
    // method and path.
    assert!(ts.contains("new OpenAPIHandler(router)"));
    assert!(ts.contains("new RPCHandler(router)"));
    assert!(ts.contains("export async function mountRouter"));
}

#[test]
fn server_disables_body_parsing() {
    let ts = server();

    // The handlers need the raw request; Fastify would otherwise consume the body.
    assert!(ts.contains("removeAllContentTypeParsers"));
    assert!(ts.contains("addContentTypeParser"));
}

#[test]
fn server_uses_request_init_members_instead_of_dom_only_aliases() {
    let ts = server();

    assert!(ts.contains(r#"request.headers as RequestInit["headers"]"#));
    assert!(ts.contains(r#"request.raw as unknown as RequestInit["body"]"#));
    assert!(!ts.contains("HeadersInit"));
    assert!(!ts.contains("BodyInit"));
}

#[test]
fn openapi_script_generates_from_the_router() {
    let ts = openapi_script();

    // Deriving the document from the router means it cannot describe an endpoint
    // the server does not serve.
    assert!(ts.contains("new OpenAPIGenerator("));
    assert!(ts.contains("ZodToJsonSchemaConverter"));
    assert!(ts.contains("generator.generate(router"));
}
