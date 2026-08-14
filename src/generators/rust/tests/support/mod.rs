//! Shared fixtures for Rust generation tests.

mod columns;
mod tables;

use crate::database::introspection::{Enum, Schema, Table};

pub(in crate::generators::rust::tests) use columns::column;
pub(super) use columns::{required, serial_key};
pub(super) use tables::{composite, products};

/// Wraps tables in a schema with no enum types.
pub(super) fn from_tables(tables: Vec<Table>) -> Schema {
    Schema {
        tables,
        enums: Vec::new(),
    }
}

/// Wraps tables and enum types in a schema.
pub(super) fn with_enums(tables: Vec<Table>, enums: Vec<Enum>) -> Schema {
    Schema { tables, enums }
}

/// Builds an enum type.
pub(super) fn enum_type(name: &str, values: &[&str]) -> Enum {
    Enum {
        name: name.to_string(),
        values: values.iter().map(|value| value.to_string()).collect(),
    }
}

/// A schema holding just the `products` fixture.
pub(super) fn schema() -> Schema {
    from_tables(vec![products()])
}

/// Resolves one table's model, running the naming pass the writer would.
///
/// Tests that build a single-table schema still go through `naming::resolve`, so they
/// exercise the same path production does rather than a shortcut.
pub(super) fn resolve(
    table: &crate::database::introspection::Table,
    schema: &Schema,
) -> crate::generators::rust::model::Model {
    let names = crate::generators::naming::resolve(schema);
    let index = schema
        .tables
        .iter()
        .position(|candidate| candidate.name == table.name)
        .expect("the table should be in the schema");

    crate::generators::rust::model::Model::resolve(table, schema, &names[index])
}

/// Creates a unique scratch directory.
///
/// Rooted in the OS temp directory rather than `target/`, which is not guaranteed to
/// exist for a fresh checkout. A per-process counter joins the label because two parallel
/// tests sharing a label would otherwise delete each other's directories mid-test.
pub(super) fn scratch(label: &str) -> std::path::PathBuf {
    use std::sync::atomic::{AtomicU32, Ordering};
    static COUNTER: AtomicU32 = AtomicU32::new(0);

    let path = std::env::temp_dir().join(format!(
        "rustyroad-rust-model-test-{label}-{}-{}",
        std::process::id(),
        COUNTER.fetch_add(1, Ordering::Relaxed)
    ));
    let _ = std::fs::remove_dir_all(&path);
    std::fs::create_dir_all(&path).expect("failed to create scratch directory");
    path
}
