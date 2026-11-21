//! Pair selected timestamp names with database time while preserving bound inputs.

use super::super::fields;
use super::policy;
use crate::database::introspection::{Column, Table};
use crate::generators::rust::{allocation::Names, naming::sql};

/// Emit matching insert column/value fragments in catalog order.
/// Explicit Some values use normal binds; only omitted selected clocks use SQL time.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out ./generated-db
/// ```
pub(in crate::generators::rust) fn render(
    table: &Table,
    columns: &[&Column],
    names: &Names,
) -> (String, String) {
    let (columns, values): (Vec<_>, Vec<_>) = columns.iter().map(|column| {
        if !policy::eligible(table, column) {
            return fields::insert(table, &[*column], names);
        }
        let field = names.field(table, &column.name);
        let name = &column.name;
        let column_sql = sql(name);
        let column = format!(
            "if input.{field}.is_some() || clocks.contains(&{name:?}) {{ columns.push({column_sql:?}); }}"
        );
        let value = format!(
            "if let Some(value) = input.{field}.as_ref() {{ values.push_bind(value); }} \
             else if clocks.contains(&{name:?}) {{ values.push(\"CURRENT_TIMESTAMP\"); }}"
        );
        (column, value)
    }).unzip();
    (columns.join("\n"), values.join("\n"))
}
