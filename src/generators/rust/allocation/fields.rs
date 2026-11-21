//! Allocate each full table before create/patch renderers filter its fields.

use super::namespace;
use crate::database::introspection::Schema;
use crate::generators::rust::naming::snake;
use std::collections::BTreeMap;

pub(super) fn allocate(schema: &Schema) -> BTreeMap<String, BTreeMap<String, String>> {
    schema
        .tables
        .iter()
        .map(|table| {
            let candidates = table
                .columns
                .iter()
                .map(|column| (column.name.clone(), snake(&column.name)))
                .collect();
            (
                table.name.clone(),
                namespace::allocate(candidates, &[], "_"),
            )
        })
        .collect()
}
