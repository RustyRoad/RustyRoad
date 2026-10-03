//! Shared selective Patch fragments; absent fields never become assignments.

use super::super::naming::pascal;
use super::{fields, inputs};
use crate::database::introspection::Table;

/// Patch emission plan; e.g. `Some(None)` binds SQL NULL rather than omitting a field.
pub(super) struct Patch {
    pub parameter: String,
    pub changed: String,
    pub assignments: String,
}

/// Reads generated Patch fields; e.g. `render(table, "patch")` preserves insert inputs.
/// Primary keys stay excluded. No clocks, defaults or EXCLUDED values are added.
pub(super) fn render(table: &Table, binding: &str) -> Patch {
    let columns = inputs::patch_columns(table);
    let model = pascal(&table.name);
    let parameter = if columns.is_empty() {
        format!("Patch{model} {{}}: Patch{model}")
    } else {
        format!("{binding}: Patch{model}")
    };
    let changed = if columns.is_empty() {
        "false".to_string()
    } else {
        inputs::present_from(&columns, binding)
    };
    let assignments = columns.iter()
        .map(|column| fields::assignment_from(column, binding))
        .collect::<Vec<_>>().join("\n");
    Patch { parameter, changed, assignments }
}
