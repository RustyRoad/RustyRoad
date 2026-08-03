//! oRPC procedure and router generation.

use super::support::schema;
use crate::generators::typescript::orpc::render;
use crate::generators::typescript::Casing;

/// Renders the router for the fixture schema.
fn router() -> String {
    render(&schema(), Casing::Camel, "/api")
}

#[test]
fn procedures_declare_method_and_path() {
    let ts = router();

    // The explicit route is what lets one definition serve RPC and REST, and is
    // what oRPC reads when generating OpenAPI.
    assert!(ts.contains(r#".route({ method: "GET", path: "/api/users" })"#));
    assert!(ts.contains(r#".route({ method: "GET", path: "/api/users/{id}" })"#));
    assert!(ts.contains(r#".route({ method: "POST", path: "/api/users" })"#));
    assert!(ts.contains(r#".route({ method: "PATCH", path: "/api/users/{id}" })"#));
    assert!(ts.contains(r#".route({ method: "DELETE", path: "/api/users/{id}" })"#));
}

#[test]
fn procedures_validate_with_the_derived_schemas() {
    let ts = router();

    assert!(ts.contains(".input(usersInsertSchema)"));
    assert!(ts.contains(".output(usersSelectSchema)"));
    assert!(ts.contains(".output(z.array(usersSelectSchema))"));
}

#[test]
fn updates_combine_the_key_with_the_patch_schema() {
    let ts = router();

    // A PATCH carries both the id from the path and the columns to set.
    assert!(ts.contains(".and(usersUpdateSchema)"));
    assert!(ts.contains("rest(input)"));
}

#[test]
fn delete_returns_an_object_literal() {
    let ts = router();

    // An arrow body starting with `{` would parse as a block and return void.
    assert!(ts.contains("({ deleted: await removed("));
}

#[test]
fn generated_namespace_exposes_every_table() {
    let ts = router();

    // Named `generated` because the root router lives in the developer-owned
    // api.ts, which composes this with hand-written procedures.
    assert!(ts.contains("export const generated = {"));
    assert!(ts.contains("users: usersRouter,"));
    assert!(ts.contains("posts: postsRouter,"));
}
