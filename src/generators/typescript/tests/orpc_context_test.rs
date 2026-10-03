//! Context, error handling, and key coercion in the generated router.

use super::support::{column, from_tables, schema};
use crate::database::introspection::Table;
use crate::generators::typescript::orpc::render;
use crate::generators::typescript::Casing;

#[test]
fn routers_are_built_on_a_typed_context() {
    let ts = render(&schema(), Casing::Camel, "/api");

    assert!(ts.contains("export interface RouterContext"));
    assert!(ts.contains("const base = os.$context<RouterContext>();"));
    assert!(ts.contains("context.db"));
}

#[test]
fn missing_rows_raise_a_typed_error() {
    let ts = render(&schema(), Casing::Camel, "/api");

    // An ORPCError keeps the failure typed for RPC callers while still mapping to
    // the right HTTP status over REST.
    assert!(ts.contains("new ORPCError(\"NOT_FOUND\""));
    assert!(ts.contains("found(await usersRepository.find"));
}

#[test]
fn numeric_keys_are_coerced() {
    let ts = render(&schema(), Casing::Camel, "/api");

    // A REST path parameter arrives as a string, so coercion is required even
    // though a direct RPC call may already pass a number.
    assert!(ts.contains("z.coerce.number().int()"));
}

#[test]
fn nonnumeric_keys_are_not_coerced() {
    let table = Table {
        name: "sessions".to_string(),
        columns: vec![column("id", "uuid")],
        primary_key: vec!["id".to_string()],
        ..Table::default()
    };

    let ts = render(&from_tables(vec![table]), Casing::Camel, "/api");
    assert!(ts.contains(r#"z.object({ id: sessionsSelectSchema.shape["id"] })"#));
    assert!(!ts.contains("z.coerce"));
}

#[test]
fn composite_keys_are_skipped() {
    let table = Table {
        name: "memberships".to_string(),
        columns: vec![column("user_id", "integer"), column("group_id", "integer")],
        primary_key: vec!["user_id".to_string(), "group_id".to_string()],
        ..Table::default()
    };

    let ts = render(&from_tables(vec![table]), Casing::Camel, "/api");

    // A composite key has no single `{id}` form, so no procedures are invented.
    assert!(ts.contains("No tables with a single-column primary key"));
}
