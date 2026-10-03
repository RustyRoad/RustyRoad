//! Shared dynamic INSERT prefix for atomic writes; uses the existing New input rules.

use super::super::naming::sql;
use super::{fields, inputs};
use crate::database::introspection::Table;

/// Emits insert construction; e.g. omitted timestamp fields use database defaults.
/// Call only with a nonempty New shape; `atomic::render(table, schema)` enforces this.
pub(super) fn render(table: &Table) -> String {
    let columns = inputs::new_columns(table);
    let has_values = if columns.iter().any(|column| !inputs::optional(column)) {
        "true".to_string()
    } else {
        inputs::present(&columns)
    };
    let (column_pushes, value_pushes) = fields::insert(&columns);
    let prefix = format!("INSERT INTO {} AS {} ", sql(&table.name), sql("existing"));
    format!(
        "    let mut query = QueryBuilder::<Postgres>::new({prefix:?});\n\
             if {has_values} {{\n\
                 query.push(\"(\");\n\
                 {{\n\
                     let mut columns = query.separated(\", \");\n\
         {column_pushes}\n\
                 }}\n\
                 query.push(\") VALUES (\");\n\
                 {{\n\
                     let mut values = query.separated(\", \");\n\
         {value_pushes}\n\
                 }}\n\
                 query.push(\")\");\n\
             }} else {{\n\
                 query.push(\"DEFAULT VALUES\");\n\
             }}\n"
    )
}
