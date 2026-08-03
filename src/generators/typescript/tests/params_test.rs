//! Path parameter typing and coercion.

use super::support::{column, schema};
use crate::database::introspection::{Schema, Table};
use crate::generators::typescript::fastify::render;
use crate::generators::typescript::Casing;

#[test]
fn numeric_path_params_are_coerced() {
    let ts = render(&schema(), Casing::Camel);

    // A path parameter always arrives as a string, so coercion is required.
    assert!(ts.contains("z.object({ id: z.coerce.number().int() })"));
}

#[test]
fn string_path_params_are_not_coerced() {
    let table = Table {
        name: "sessions".to_string(),
        columns: vec![column("id", "uuid")],
        primary_key: vec!["id".to_string()],
        foreign_keys: Vec::new(),
        uniques: Vec::new(),
        indexes: Vec::new(),
    };

    let ts = render(&Schema { tables: vec![table] }, Casing::Camel);
    assert!(ts.contains("z.object({ id: z.string() })"));
}

#[test]
fn composite_keys_are_skipped() {
    let table = Table {
        name: "memberships".to_string(),
        columns: vec![column("user_id", "integer"), column("group_id", "integer")],
        primary_key: vec!["user_id".to_string(), "group_id".to_string()],
        foreign_keys: Vec::new(),
        uniques: Vec::new(),
        indexes: Vec::new(),
    };

    let ts = render(&Schema { tables: vec![table] }, Casing::Camel);

    // A composite key has no single `/:id` form, so no routes are invented.
    assert!(ts.contains("No tables with a single-column primary key"));
}
