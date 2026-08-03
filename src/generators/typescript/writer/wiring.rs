//! Detecting tables the composition file has not wired up.
//!
//! `api.ts` is preserved across runs, so a table added after it was written will
//! not appear there and its procedures will be unreachable. Silently generating
//! them is worse than saying so, since the omission is invisible otherwise.

use crate::database::introspection::{Schema, Table};
use crate::generators::typescript::casing::{binding, Casing};
use std::path::Path;

/// Returns the tables whose procedures `api.ts` does not reference.
///
/// Matching is textual rather than syntactic: a developer may restructure the file
/// however they like, so the only reliable signal is whether the generated
/// namespace member is mentioned at all.
pub(super) fn unwired(path: &Path, schema: &Schema, casing: Casing) -> Vec<String> {
    let Ok(contents) = std::fs::read_to_string(path) else {
        return Vec::new();
    };

    schema
        .tables
        .iter()
        .filter(|table| table.has_simple_key())
        .filter(|table| !is_wired(&contents, table, casing))
        .map(|table| table.name.clone())
        .collect()
}

/// Returns `true` when the file appears to reference a table's procedures.
///
/// A spread of the whole namespace counts, since that wires up every table at once.
fn is_wired(contents: &str, table: &Table, casing: Casing) -> bool {
    let member = binding(&table.name, casing);

    contents.contains("...generated")
        || contents.contains(&format!("generated.{member}"))
        || contents.contains(&format!("generated[\"{}\"]", table.name))
}
