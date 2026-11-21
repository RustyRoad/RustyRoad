//! Schema-derived extensions alongside primary-key CRUD; no application SQL wrappers.
mod clocks;
pub(in crate::generators::rust) mod increment;
pub(in crate::generators::rust) mod toggle;

use super::super::{allocation::Names, types};
use super::{delete_key, inputs, lookup, lookup_list, policy, relations, targets, upsert};
use super::{update_key, upsert_patch};
use crate::database::introspection::{Schema, Table};

/// Emits native writes and selectors; e.g. duplicate URL keys emit one policy pair.
/// Returns whether emitted functions need QueryBuilder; views emit no extensions.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out ./generated-db
/// ```
pub(super) fn render(table: &Table, schema: &Schema, names: &Names) -> (String, bool) {
    let mut output = String::new();
    let mut builder = false;
    if table.view {
        return (output, builder);
    }
    output.push_str(&clocks::declaration(table, schema, names));
    if table.has_simple_key() {
        let key = table
            .column(&table.primary_key[0])
            .expect("catalog primary key exists");
        let key_type = types::map_column(table, key, schema, names).rust;
        output.push_str(&toggle::render(table, &key_type, names));
        output.push_str(&increment::render(table, schema, &key_type, names));
    }
    for key in targets::keys(table) {
        output.push_str(&lookup::render(table, schema, &key, None, names));
        output.push_str(&delete_key::render(table, schema, &key, names));
        let (update, update_builder) = update_key::render(table, schema, &key, names, None);
        output.push_str(&update);
        builder |= update_builder;
        if let Some(clock) = policy::clock(table).filter(|clock| !key.contains(&clock.name)) {
            let (touch, touch_builder) =
                update_key::render(table, schema, &key, names, Some(clock));
            output.push_str(&touch);
            builder |= touch_builder;
        }
        if inputs::new_columns(table).is_empty() {
            continue;
        }
        output.push_str(&upsert_patch::render(table, &key, names, None));
        output.push_str(&clocks::render(table, &key, names));
        if let Some(clock) = policy::clock(table).filter(|clock| !key.contains(&clock.name)) {
            output.push_str(&upsert_patch::render(table, &key, names, Some(clock)));
        }
        builder = true;
        let snapshot = upsert::render(table, &key, None, names);
        builder |= !snapshot.is_empty();
        output.push_str(&snapshot);
        for column in policy::payload(table, &key) {
            if column.sql_type.trim() == "jsonb" {
                let merge = upsert::render(table, &key, Some(column), names);
                builder |= !merge.is_empty();
                output.push_str(&merge);
            }
        }
    }
    for key in relations::keys(table) {
        output.push_str(&lookup_list::render(table, schema, &key, names));
        output.push_str(&delete_key::render(table, schema, &key, names));
    }
    if let Some(clock) = policy::clock(table) {
        for column in table.columns.iter().filter(|column| policy::text(column)) {
            output.push_str(&lookup::render(
                table,
                schema,
                std::slice::from_ref(&column.name),
                Some(clock),
                names,
            ));
        }
    }
    (output, builder)
}
