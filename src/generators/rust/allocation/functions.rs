//! Allocate entire function names, not just ambiguous `_by_`/`_and_` fragments.

mod candidates;
mod partial;
mod paths;
mod selectors;
mod spatial;

use super::{namespace, Names};
use crate::database::introspection::{PartialUniqueIndex, Schema, Table};
use std::collections::BTreeMap;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub(super) struct Key(String, String, Vec<String>, Option<String>);

fn key(table: &Table, verb: &str, columns: &[String], merge: Option<&str>) -> Key {
    Key(
        table.name.clone(),
        verb.into(),
        columns.to_vec(),
        merge.map(str::to_string),
    )
}

/// Allocates repository names while reserving configuration and transaction helpers.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn allocate(schema: &Schema) -> BTreeMap<Key, String> {
    allocate_with_partials(schema, &[])
}

/// Includes separately scoped partial conflicts without promoting them to unique keys.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn allocate_with_partials(
    schema: &Schema,
    indexes: &[PartialUniqueIndex],
) -> BTreeMap<Key, String> {
    namespace::allocate(
        partial::candidates(schema, indexes),
        &[
            "configure_generated",
            super::super::repositories::clock::FUNCTION,
            super::super::repositories::read_snapshot::FUNCTION,
        ],
        "_",
    )
}

impl Names {
    pub(in crate::generators::rust) fn function(&self, table: &Table, verb: &str) -> &str {
        self.selector(table, verb, &[])
    }

    pub(in crate::generators::rust) fn selector(
        &self,
        table: &Table,
        verb: &str,
        columns: &[String],
    ) -> &str {
        &self.functions[&key(table, verb, columns, None)]
    }

    pub(in crate::generators::rust) fn merge(
        &self,
        table: &Table,
        columns: &[String],
        column: &str,
    ) -> &str {
        &self.functions[&key(table, "upsert_merge", columns, Some(column))]
    }
}
