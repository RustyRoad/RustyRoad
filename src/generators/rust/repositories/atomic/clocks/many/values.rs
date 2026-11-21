use super::super::policy;
use crate::database::introspection::{Column, Table};
use crate::generators::rust::allocation::Names;
use crate::generators::rust::repositories::inputs;

/// Emits a bound value, explicit SQL NULL, selected clock, or database DEFAULT.
/// Omission rules mirror the generated clock-aware input without synthesizing values.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn render(table: &Table, column: &Column, names: &Names) -> String {
    let field = names.field(table, &column.name);
    if policy::eligible(table, column) {
        let name = &column.name;
        format!(
            "if let Some(value) = input.{field}.as_ref() {{ values.push_bind(value); }}\n\
             else if insert_clocks.contains(&{name:?}) {{ values.push(\"CURRENT_TIMESTAMP\"); }}\n\
             else {{ values.push(\"DEFAULT\"); }}"
        )
    } else if inputs::optional(column) {
        format!(
            "if let Some(value) = input.{field}.as_ref() {{ values.push_bind(value); }}\n\
             else {{ values.push(\"DEFAULT\"); }}"
        )
    } else {
        format!("values.push_bind(&input.{field});")
    }
}
