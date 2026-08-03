//! Zod schema generation.

use super::support::schema;
use crate::generators::typescript::zod::render;
use crate::generators::typescript::Casing;

#[test]
fn schemas_are_derived_from_the_drizzle_tables() {
    let ts = render(&schema(), Casing::Camel);

    // Deriving keeps validation in lockstep with the database rather than
    // introducing a second, hand-maintained source of truth.
    assert!(ts.contains("export const usersSelectSchema = createSelectSchema(users);"));
    assert!(ts.contains("export const usersInsertSchema = createInsertSchema(users);"));
    assert!(ts.contains("export const usersUpdateSchema = createUpdateSchema(users);"));
}

#[test]
fn drizzle_zod_helpers_are_imported() {
    let ts = render(&schema(), Casing::Camel);

    assert!(ts.contains("from \"drizzle-zod\""));
    assert!(ts.contains("createSelectSchema"));
    assert!(ts.contains("createInsertSchema"));
    assert!(ts.contains("createUpdateSchema"));
    assert!(ts.contains("import { z } from \"zod\";"));
}

#[test]
fn row_types_are_inferred_from_the_zod_schemas() {
    let ts = render(&schema(), Casing::Camel);

    assert!(ts.contains("export type Users = z.infer<typeof usersSelectSchema>;"));
    assert!(ts.contains("export type NewUsers = z.infer<typeof usersInsertSchema>;"));
    assert!(ts.contains("export type PatchUsers = z.infer<typeof usersUpdateSchema>;"));
}

#[test]
fn tables_are_imported_from_the_generated_schema() {
    let ts = render(&schema(), Casing::Camel);

    assert!(ts.contains("import { users, posts } from \"./schema\";"));
}

#[test]
fn a_shared_error_schema_is_exported() {
    let ts = render(&schema(), Casing::Camel);

    // Referenced by every 404 response, so it must be defined once.
    assert!(ts.contains("export const errorSchema = z.object({ error: z.string() });"));
}

#[test]
fn every_table_gets_schemas() {
    let ts = render(&schema(), Casing::Camel);

    assert!(ts.contains("postsSelectSchema"));
    assert!(ts.contains("export type Posts ="));
}
