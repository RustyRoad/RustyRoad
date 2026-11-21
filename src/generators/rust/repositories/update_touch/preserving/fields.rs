//! Assignment fragments retain normal patch behavior unless explicitly selected.
use super::{sql, Names, Table};
use crate::database::introspection::Column;
use crate::generators::rust::repositories::fields;

/// Builds one validated nullable-column preservation or an ordinary assignment.
/// The stored operand comes first in COALESCE and the supplied value stays bound.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out ./generated-db
/// ```
pub(super) fn assignment(table: &Table, column: &Column, clock: &str, names: &Names) -> String {
    if !column.nullable || column.name == clock {
        return fields::assignment(table, column, names);
    }
    let field = names.field(table, &column.name);
    let prefix = format!("{} = ", sql(&column.name));
    let preserved = format!("COALESCE({}, ", sql(&column.name));
    let column_name = &column.name;
    format!(
        "if let Some(value) = input.{field}.as_ref() {{\n\
             set.push({prefix:?});\n\
             if preserve_existing.contains(&{column_name:?}) {{\n\
                 set.push_unseparated({preserved:?}).push_bind_unseparated(value).push_unseparated(\")\");\n\
             }} else {{ set.push_bind_unseparated(value); }}\n\
         }}"
    )
}
