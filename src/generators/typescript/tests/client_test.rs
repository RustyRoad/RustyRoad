//! Typed client generation.

use super::support::{enum_key_schema, schema};
use crate::generators::typescript::client::render;
use crate::generators::typescript::Casing;

#[test]
fn row_types_are_inferred_from_the_drizzle_tables() {
    let ts = render(&schema(), Casing::Camel);

    // The repository returns what the driver produced, and Drizzle and drizzle-zod
    // disagree on some column types (jsonb especially), so the repository is typed
    // by Drizzle while the Zod schemas govern the HTTP boundary.
    assert!(ts.contains("export type UsersRow = InferSelectModel<typeof users>;"));
    assert!(ts.contains("export type NewUsersRow = InferInsertModel<typeof users>;"));
}

#[test]
fn repository_signatures_use_the_row_types() {
    let ts = render(&schema(), Casing::Camel);

    assert!(ts.contains(
        r#"export type RepositoryDatabase = Pick<Database, "select" | "insert" | "update" | "delete">;"#
    ));
    assert!(ts.contains("list(db: RepositoryDatabase): Promise<UsersRow[]>"));
    assert!(ts.contains("Promise<UsersRow[]>"));
    assert!(ts.contains("values: NewUsersRow"));
    // A PATCH carries any subset of columns.
    assert!(ts.contains("values: Partial<NewUsersRow>"));
}

#[test]
fn repository_covers_full_crud() {
    let ts = render(&schema(), Casing::Camel);

    assert!(ts.contains("export const usersRepository = {"));
    for method in ["list(", "find(", "create(", "update(", "remove("] {
        assert!(ts.contains(method), "missing method: {method}");
    }
}

#[test]
fn client_factory_is_emitted() {
    let ts = render(&schema(), Casing::Camel);

    assert!(ts.contains("export type Database = ReturnType<typeof drizzle>;"));
    assert!(ts.contains("export function createClient(connectionString: string): Database"));
}

#[test]
fn enum_primary_keys_use_the_inferred_column_type_for_every_keyed_query() {
    let ts = render(&enum_key_schema(), Casing::Camel);
    let key_type = "id: CampaignWorkflowNodeBindingsRow[\"nodeKind\"]";

    // find, update, and remove must pass the enum union to Drizzle's `eq`
    // overload rather than widening it to an incompatible plain string.
    assert_eq!(ts.matches(key_type).count(), 3);
    assert!(ts.contains("eq(campaignWorkflowNodeBindings.nodeKind, id)"));
    assert!(!ts.contains("async find(db: RepositoryDatabase, id: string"));
    assert!(!ts.contains("async update(db: RepositoryDatabase, id: string"));
    assert!(!ts.contains("async remove(db: RepositoryDatabase, id: string"));
}
