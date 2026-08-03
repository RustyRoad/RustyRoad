//! `relations.ts` generation from foreign keys.

use super::support::{from_tables, schema, users};
use crate::generators::typescript::relations::render;
use crate::generators::typescript::Casing;

#[test]
fn referencing_side_gets_a_one_relation() {
    let ts = render(&schema(), Casing::Camel);

    // posts.author_id -> users.id, so a post has one user.
    assert!(ts.contains("export const postsRelations = relations(posts,"));
    assert!(ts.contains("fields: [posts.authorId]"));
    assert!(ts.contains("references: [users.id]"));
}

#[test]
fn referenced_side_gets_a_many_relation() {
    let ts = render(&schema(), Casing::Camel);

    assert!(ts.contains("export const usersRelations = relations(users,"));
    assert!(ts.contains("posts: many(posts)"));
}

#[test]
fn helpers_are_declared_only_when_used() {
    let ts = render(&schema(), Casing::Camel);

    // users only has a many side; posts only has a one side.
    assert!(ts.contains("relations(users, ({ many })"));
    assert!(ts.contains("relations(posts, ({ one })"));
}

#[test]
fn schema_import_lists_referenced_tables() {
    let ts = render(&schema(), Casing::Camel);

    assert!(ts.contains("import { relations } from \"drizzle-orm\";"));
    assert!(ts.contains("from \"./schema\";"));
}

#[test]
fn a_schema_without_foreign_keys_yields_no_relations() {
    let ts = render(&from_tables(vec![users()]), Casing::Camel);

    // Emitting an empty relations file would be misleading, so it says why.
    assert!(ts.contains("No foreign keys"));
    assert!(!ts.contains("relations("));
}
