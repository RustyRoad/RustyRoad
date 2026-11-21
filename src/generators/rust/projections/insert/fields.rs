//! Matching insert columns and bound values for tri-state generated inputs.
use super::{codecs, configuration::InsertInput};
use crate::database::introspection::Table;
use crate::generators::rust::{allocation::Names, naming::sql, repositories::inputs};

/// Only outer None omits a column; Some(None) always produces a typed NULL bind.
/// Parameter casts run in PostgreSQL, before target-column constraints/precision.
/// Clock fallback is reached only when the entire input field is omitted.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn render(table: &Table, names: &Names, input: &InsertInput) -> (String, String) {
    let (columns, values): (Vec<_>, Vec<_>) = inputs::new_columns(table).iter().map(|column| {
        let field = names.field(table, &column.name);
        let column_sql = sql(&column.name);
        let clock = format!("clocks.contains(&{:?})", column.name);
        let suffix = codecs::cast(column, input).map(|(_, suffix)| suffix).unwrap_or("");
        let cast = if suffix.is_empty() { String::new() }
            else { format!(".push_unseparated({suffix:?})") };
        if inputs::optional(column) {
            (
                format!("if input.{field}.is_some() || {clock} {{ columns.push({column_sql:?}); }}"),
                format!(
                    "if let Some(value) = input.{field}.as_ref() {{ values.push_bind(value){cast}; }} \
                     else if {clock} {{ values.push(\"CURRENT_TIMESTAMP\"); }}"
                ),
            )
        } else {
            (format!("columns.push({column_sql:?});"),
                format!("values.push_bind(&input.{field}){cast};"))
        }
    }).unzip();
    (columns.join("\n"), values.join("\n"))
}
