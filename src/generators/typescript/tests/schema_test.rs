//! Table declarations and imports in `schema.ts`.

use super::support::schema;
use crate::generators::typescript::schema::render;
use crate::generators::typescript::Casing;

#[test]
fn tables_are_exported_with_their_database_name() {
    let ts = render(&schema(), Casing::Camel);

    assert!(ts.contains("export const users = pgTable(\"users\", {"));
    assert!(ts.contains("export const posts = pgTable(\"posts\", {"));
}

#[test]
fn parents_are_declared_before_their_children() {
    let ts = render(&schema(), Casing::Camel);

    // A `const` is not hoisted, so a foreign key referencing a later binding would
    // fail at module evaluation.
    let users_at = ts.find("export const users").expect("users declared");
    let posts_at = ts.find("export const posts").expect("posts declared");
    assert!(users_at < posts_at);
}

#[test]
fn imports_cover_every_builder_used() {
    let ts = render(&schema(), Casing::Camel);
    let header = ts.lines().next().unwrap_or_default();

    for symbol in ["pgTable", "serial", "varchar", "boolean", "integer", "text"] {
        assert!(header.contains(symbol), "missing import: {symbol}");
    }
}

#[test]
fn sql_is_imported_only_for_expression_defaults() {
    let ts = render(&schema(), Casing::Camel);

    // created_at defaults to now(), which must stay SQL.
    assert!(ts.contains("import { sql } from \"drizzle-orm\";"));
    assert!(ts.contains(".default(sql`now()`)"));
}
