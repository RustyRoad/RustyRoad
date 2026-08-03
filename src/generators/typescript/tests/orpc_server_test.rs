//! Server wiring and key coercion for the oRPC router.

use super::support::{column, from_tables, schema};
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
fn string_keys_are_not_coerced() {
    let table = Table {
        name: "sessions".to_string(),
        columns: vec![column("id", "uuid")],
        primary_key: vec!["id".to_string()],
        foreign_keys: Vec::new(),
        uniques: Vec::new(),
        indexes: Vec::new(),
    };

    let ts = render(&from_tables(vec![table]), Casing::Camel, "/api");
    assert!(ts.contains("z.object({ id: z.string() })"));
    assert!(!ts.contains("z.coerce"));
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
fn openapi_script_generates_from_the_router() {
    let ts = openapi_script();

    // Deriving the document from the router means it cannot describe an endpoint
    // the server does not serve.
    assert!(ts.contains("new OpenAPIGenerator("));
    assert!(ts.contains("ZodToJsonSchemaConverter"));
    assert!(ts.contains("generator.generate(router"));
}
