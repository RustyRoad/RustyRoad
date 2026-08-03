//! Detecting tables the preserved composition file does not serve.
//!
//! Preservation means a table added later will not appear in `api.ts`, so its
//! procedures are generated but unreachable. That is invisible unless reported.

use super::support::{column, from_tables, users};
use super::writer_support::{pull, scratch};
use crate::database::introspection::{Schema, Table};
use std::path::PathBuf;

/// A second table, standing in for one added after `api.ts` was written.
pub(super) fn orders() -> Table {
    Table {
        name: "orders".to_string(),
        columns: vec![column("id", "integer"), column("total", "numeric(10,2)")],
        primary_key: vec!["id".to_string()],
        foreign_keys: Vec::new(),
        uniques: Vec::new(),
        indexes: Vec::new(),
    }
}

/// Writes `schema` into `out` and returns the unwired table names.
pub(super) fn unwired(out: &PathBuf, schema: &Schema) -> Vec<String> {
    pull(out, schema, false).unwired
}

#[test]
fn a_fresh_composition_wires_everything() {
    let out = scratch("wiring-fresh");

    // The first run writes api.ts listing every table, so nothing is unwired.
    assert!(unwired(&out, &from_tables(vec![users(), orders()])).is_empty());

    let _ = std::fs::remove_dir_all(&out);
}

#[test]
fn a_table_added_after_the_composition_is_reported() {
    let out = scratch("wiring-added");
    unwired(&out, &from_tables(vec![users()]));

    // api.ts now exists and mentions only users; orders arrives later.
    let reported = unwired(&out, &from_tables(vec![users(), orders()]));

    assert_eq!(reported, vec!["orders".to_string()]);

    let _ = std::fs::remove_dir_all(&out);
}
