//! Zod schema generation.

use super::support::schema;
use super::writer_support::scratch;
use crate::generators::typescript::zod::render;
use crate::generators::typescript::{write, Casing, Outputs};

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
fn annotated_jsonb_columns_refine_all_schema_variants() {
    let ts = render(&super::support::annotated_json_schema(), Casing::Camel);
    let shape =
        "z.object({\"city\": z.string(), \"population\": z.number().int().optional()}).strict()";

    assert!(ts.contains(&format!(
        "createSelectSchema(users, {{ metadata: {shape}.nullable() }})"
    )));
    assert!(ts.contains(&format!(
        "createInsertSchema(users, {{ metadata: {shape}.nullable().optional() }})"
    )));
    assert!(ts.contains(&format!(
        "createUpdateSchema(users, {{ metadata: {shape}.nullable().optional() }})"
    )));
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

#[test]
fn standalone_target_writes_only_zod_and_its_local_dependency() {
    let out = scratch("zod-only");

    let report = write(&out, &schema(), Casing::Camel, Outputs::zod_only(), false)
        .expect("standalone Zod output should write");

    assert_eq!(report.outcomes.len(), 2);
    assert!(out.join("schema.ts").exists());
    assert!(out.join("zod.ts").exists());
    for unrelated in [
        "relations.ts",
        "client.ts",
        "router.ts",
        "api.ts",
        "server.ts",
    ] {
        assert!(!out.join(unrelated).exists(), "unexpected {unrelated}");
    }

    let full = scratch("zod-full-equivalence");
    write(&full, &schema(), Casing::Camel, Outputs::all(), false)
        .expect("full TypeScript output should write");
    assert_eq!(
        std::fs::read_to_string(out.join("zod.ts")).unwrap(),
        std::fs::read_to_string(full.join("zod.ts")).unwrap()
    );

    let _ = std::fs::remove_dir_all(out);
    let _ = std::fs::remove_dir_all(full);
}
