//! An audit column on a view must not import a clock it never uses.

use super::support::{required, resolve};
use super::view_test::view;
use crate::database::introspection::Schema;
use crate::generators::rust::render;

#[test]
fn a_view_with_audit_columns_does_not_import_an_unused_utc() {
    let mut table = view();
    table
        .columns
        .push(required("created_at", "timestamp with time zone"));

    let schema = Schema {
        tables: vec![table.clone()],
        enums: Vec::new(),
    };
    let model = resolve(&table, &schema);
    let root = render::files(&model, &schema)
        .into_iter()
        .find(|file| file.name == "mod.rs")
        .unwrap()
        .contents;

    // Only the constructors reach for the clock, and a view has none. An unused import is a
    // warning on every build of the generated crate.
    assert!(!root.contains("use chrono::Utc;"), "unused import:\n{root}");
    // The field itself still carries its resolved type.
    assert!(
        root.contains("pub created_at: chrono::DateTime<chrono::Utc>,"),
        "{root}"
    );
}
