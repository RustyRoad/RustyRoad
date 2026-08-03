//! Column declaration rendering for `schema.ts`.

mod defaults;

use super::casing::{db_name_argument, identifier, Casing};
use super::types::{self, Builder};
use crate::database::introspection::{Column, Table};

/// Renders one column line, including its modifier chain.
pub(super) fn render(table: &Table, column: &Column, casing: Casing) -> String {
    let builder = types::map(&column.sql_type, column.auto_increment);
    let name = db_name_argument(&column.name, casing);

    let arguments = match (&builder.options, name.is_empty()) {
        (Some(options), true) => options.clone(),
        (Some(options), false) => format!("{name}, {options}"),
        (None, true) => String::new(),
        (None, false) => name,
    };

    format!(
        "\t{}: {}({arguments}){}{}",
        identifier(&column.name, casing),
        builder.import,
        type_annotation(&builder),
        chain(table, column)
    )
}

/// Renders an explicit `$type` annotation for builders that need one.
///
/// `json`/`jsonb` default to `unknown`, which does not match the recursive `Json`
/// union `drizzle-zod` infers. Stating the type keeps the Drizzle row type and the
/// Zod schema assignable to each other.
fn type_annotation(builder: &Builder) -> &'static str {
    match builder.import {
        "json" | "jsonb" => ".$type<Record<string, unknown>>()",
        _ => "",
    }
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
