//! Fixtures for the filter tests.

use crate::database::introspection::{Schema, Table};

/// Builds a bare table with the given name.
fn table(name: &str) -> Table {
    Table {
        name: name.to_string(),
        ..Table::default()
    }
}

/// A schema with a model family, a junk family, and a stray duplicate.
pub(super) fn schema() -> Schema {
    Schema {
        tables: vec![
            table("customers"),
            table("orders"),
            table("ab_test_archive"),
            table("ab_test_queue"),
            table("user"),
            table("users"),
        ],
        enums: Vec::new(),
    }
}

/// Runs the filter with the given command-line arguments.
pub(super) fn run(args: &[&str]) -> Vec<String> {
    let command = crate::database::introspection::cli::pull_command();
    let matches = command
        .try_get_matches_from([&["pull"], args].concat())
        .expect("arguments should parse");

    super::filter::apply(&matches, schema())
        .tables
        .into_iter()
        .map(|table| table.name)
        .collect()
}
