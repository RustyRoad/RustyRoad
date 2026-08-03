//! Collecting relation entries from foreign keys.

use super::super::casing::{binding, identifier, Casing};
use crate::database::introspection::Schema;
use std::collections::BTreeMap;

/// Builds the relation entries for every table, keyed by table binding.
///
/// Each foreign key yields two sides: the referencing table gains a `one` relation
/// to its parent, and the referenced table gains a `many` relation back.
pub(super) fn collect(schema: &Schema, casing: Casing) -> BTreeMap<String, Vec<String>> {
    let mut sides: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for table in &schema.tables {
        let local = binding(&table.name, casing);

        for key in &table.foreign_keys {
            let parent = binding(&key.foreign_table, casing);

            let fields = column_list(&local, &key.columns, casing);
            let references = column_list(&parent, &key.foreign_columns, casing);
            sides.entry(local.clone()).or_default().push(format!(
                "{parent}: one({parent}, {{\n\t\tfields: [{fields}],\n\t\treferences: [{references}]\n\t}})"
            ));

            sides
                .entry(parent)
                .or_default()
                .push(format!("{local}: many({local})"));
        }
    }

    sides
}

/// Returns the helper parameters a table's relations require.
pub(super) fn helpers(entries: &[String]) -> String {
    let mut helpers = Vec::new();
    if entries.iter().any(|entry| entry.contains("one(")) {
        helpers.push("one");
    }
    if entries.iter().any(|entry| entry.contains("many(")) {
        helpers.push("many");
    }
    helpers.join(", ")
}

/// Renders a `table.column` list for one side of a relation.
fn column_list(table: &str, columns: &[String], casing: Casing) -> String {
    columns
        .iter()
        .map(|column| format!("{table}.{}", identifier(column, casing)))
        .collect::<Vec<_>>()
        .join(", ")
}
