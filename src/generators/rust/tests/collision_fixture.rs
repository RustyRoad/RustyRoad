//! Fixtures for the module-naming collision tests.

use super::support::{required, serial_key};
use crate::database::introspection::{Schema, Table};
use std::path::{Path, PathBuf};

/// Builds a minimal table with the given name.
fn table(name: &str) -> Table {
    Table {
        name: name.to_string(),
        columns: vec![serial_key("id"), required("label", "text")],
        primary_key: vec!["id".to_string()],
        foreign_keys: Vec::new(),
        uniques: Vec::new(),
        indexes: Vec::new(),
        view: false,
    }
}

/// A schema holding a singular/plural pair.
pub(super) fn pair() -> Schema {
    Schema {
        tables: vec![table("user"), table("users")],
        enums: Vec::new(),
    }
}

/// Returns the module directories written into `out`.
pub(super) fn directories(out: &Path) -> Vec<PathBuf> {
    std::fs::read_dir(out)
        .unwrap()
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.is_dir())
        .collect()
}
