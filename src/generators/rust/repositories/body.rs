//! Selects applicable repository operations for each introspected relation.
use super::{
    atomic, create_clock, create_many, delete_all, equality, inputs, read_only, spatial, table,
    Names, Schema,
};

/// Renders table/view bodies and reports whether generated code uses QueryBuilder.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn render(schema: &Schema, names: &Names, relationship_reads: bool) -> (String, bool) {
    let mut body = String::new();
    let mut needs_builder = false;
    for table in &schema.tables {
        body.push_str(&spatial::render(table, schema, names));
        let equality = equality::render(table, schema, names, relationship_reads);
        needs_builder |= !equality.is_empty();
        body.push_str(&equality);
        if table.view || !table.has_simple_key() {
            body.push_str(&read_only::render(table, names));
        }
        if table.view {
            continue;
        }
        body.push_str(&delete_all::render(table, names));
        if let Some(bulk) = create_many::render(table, names) {
            body.push_str(&bulk);
            needs_builder = true;
        }
        if let Some(clock) = create_clock::render(table, names) {
            body.push_str(&clock);
            needs_builder = true;
        }
        if table.has_simple_key() {
            body.push_str(&table::render(table, schema, names));
            needs_builder |=
                !inputs::new_columns(table).is_empty() || !inputs::patch_columns(table).is_empty();
        }
        let (extensions, builder) = atomic::render(table, schema, names);
        body.push_str(&extensions);
        needs_builder |= builder;
    }
    (body, needs_builder)
}
