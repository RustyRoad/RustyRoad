//! Schema-derived extensions alongside primary-key CRUD; no application SQL wrappers.

use super::{delete_key, inputs, lookup, policy, targets, upsert};
use super::{update_key, upsert_patch};
use crate::database::introspection::{Schema, Table};

/// Emits native writes and selectors; e.g. duplicate URL keys emit one policy pair.
/// Returns whether emitted functions need QueryBuilder; views emit no extensions.
pub(super) fn render(table: &Table, schema: &Schema) -> (String, bool) {
    let mut output = String::new();
    let mut builder = false;
    if table.view {
        return (output, builder);
    }
    for key in targets::keys(table) {
        output.push_str(&lookup::render(table, schema, &key, None));
        output.push_str(&delete_key::render(table, schema, &key));
        let (update, update_builder) = update_key::render(table, schema, &key);
        output.push_str(&update);
        builder |= update_builder;
        if inputs::new_columns(table).is_empty() {
            continue;
        }
        output.push_str(&upsert_patch::render(table, &key));
        builder = true;
        let snapshot = upsert::render(table, &key, None);
        builder |= !snapshot.is_empty();
        output.push_str(&snapshot);
        for column in policy::payload(table, &key) {
            if column.sql_type.trim() == "jsonb" {
                let merge = upsert::render(table, &key, Some(column));
                builder |= !merge.is_empty();
                output.push_str(&merge);
            }
        }
    }
    if let Some(clock) = policy::clock(table) {
        for column in table.columns.iter().filter(|column| policy::text(column)) {
            output.push_str(&lookup::render(table, schema, &[column.name.clone()], Some(clock)));
        }
    }
    (output, builder)
}