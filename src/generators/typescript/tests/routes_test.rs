//! Route schemas and operation ids.

use super::support::schema;
use crate::generators::typescript::fastify::render;
use crate::generators::typescript::Casing;

#[test]
fn routes_validate_with_the_derived_schemas() {
    let ts = render(&schema(), Casing::Camel);

    assert!(ts.contains("body: usersInsertSchema"));
    assert!(ts.contains("body: usersUpdateSchema"));
    assert!(ts.contains("response: { 200: usersSelectSchema"));
    assert!(ts.contains("z.array(usersSelectSchema)"));
}

#[test]
fn operation_ids_are_declared_for_client_generation() {
    let ts = render(&schema(), Casing::Camel);

    // Hey API derives method names from operationId, so it must be stable.
    for id in ["listUsers", "getUsers", "createUsers", "updateUsers", "deleteUsers"] {
        assert!(ts.contains(&format!("operationId: \"{id}\"")), "missing {id}");
    }
}

#[test]
fn tags_group_operations_by_table() {
    let ts = render(&schema(), Casing::Camel);

    assert!(ts.contains("tags: [\"Users\"]"));
    assert!(ts.contains("tags: [\"Posts\"]"));
}

#[test]
fn missing_rows_reply_404_with_the_shared_error_schema() {
    let ts = render(&schema(), Casing::Camel);

    assert!(ts.contains("404: errorSchema"));
    assert!(ts.contains("reply.code(404)"));
}
