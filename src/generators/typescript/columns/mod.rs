//! Column declaration rendering for `schema.ts`.

mod builder;
mod defaults;

use super::casing::{identifier, Casing};
use crate::database::introspection::{Column, Schema, Table};

/// Renders one column line, including its modifier chain.
///
/// `schema` is needed to recognize an enum column, which arrives as a bare type
/// name and must reference its `pgEnum` declaration rather than degrade to text.
pub(super) fn render(schema: &Schema, table: &Table, column: &Column, casing: Casing) -> String {
    format!(
        "\t{}: {}{}",
        identifier(&column.name, casing),
        builder::call(schema, column, casing),
        chain(table, column)
    )
}

/// Renders the modifier chain applied to a column builder.
fn chain(table: &Table, column: &Column) -> String {
    let mut chain = String::new();

    // A single-column primary key is expressed inline; a composite one becomes a
    // table-level constraint instead.
    if table.has_simple_key() && table.primary_key.first() == Some(&column.name) {
        chain.push_str(".primaryKey()");
    }

    if !column.nullable {
        chain.push_str(".notNull()");
    }

    if let Some(default) = defaults::call(column) {
        chain.push_str(&default);
    }

    chain
}

/// Returns `true` when any column needs the `sql` template import.
pub(super) fn needs_sql_import(table: &Table) -> bool {
    table.columns.iter().any(|column| {
        defaults::call(column)
            .map(|call| call.contains("sql`"))
            .unwrap_or(false)
    })
}
