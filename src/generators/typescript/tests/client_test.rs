//! Typed client generation.

use super::support::schema;
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

    assert!(ts.contains("export function createClient(connectionString: string): Database"));
}
