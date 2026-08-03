//! Dependency ordering for generated table declarations.

use crate::database::introspection::{Schema, Table};

/// Returns tables ordered so a table appears after everything it references.
///
/// Foreign keys reference the parent's exported binding directly, and a `const` is
/// not hoisted, so a child declared first would fail at module evaluation.
pub(super) fn ordered(schema: &Schema) -> Vec<&Table> {
    let mut remaining: Vec<&Table> = schema.tables.iter().collect();
    remaining.sort_by(|a, b| a.name.cmp(&b.name));

    let mut ordered: Vec<&Table> = Vec::with_capacity(remaining.len());
    let mut emitted: Vec<String> = Vec::with_capacity(remaining.len());

    // Repeatedly take whichever remaining table has all its parents already out.
    while !remaining.is_empty() {
        let ready = remaining
            .iter()
            .position(|table| is_ready(table, &emitted))
            .unwrap_or(0); // A cycle has no ready table; break it by name order.

        let table = remaining.remove(ready);
        emitted.push(table.name.clone());
        ordered.push(table);
    }

    ordered
}

/// Returns `true` when every table `table` references has been emitted.
///
/// A self-reference is satisfied by the declaration itself.
fn is_ready(table: &Table, emitted: &[String]) -> bool {
    table.foreign_keys.iter().all(|key| {
        key.foreign_table == table.name || emitted.contains(&key.foreign_table)
    })
}

/// Returns `true` when a foreign key points at a table declared later.
///
/// Such a reference must be deferred, which Drizzle expresses with a callback.
pub(super) fn is_forward_reference(
    table: &Table,
    foreign_table: &str,
    ordered_names: &[String],
) -> bool {
    let local = ordered_names.iter().position(|name| name == &table.name);
    let remote = ordered_names.iter().position(|name| name == foreign_table);

    match (local, remote) {
        (Some(local), Some(remote)) => remote > local,
        _ => false,
    }
}
