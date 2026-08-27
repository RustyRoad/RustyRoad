//! Builder call rendering for a column.

use super::super::casing::{binding, db_name_argument, Casing};
use super::super::enums;
use super::super::json_schema;
use super::super::types::{self, Builder};
use crate::database::introspection::{Column, Schema};

/// Renders the builder call for a column, without its modifier chain.
///
/// An enum column references its `pgEnum` declaration rather than a column builder,
/// so its values are not lost to a plain text mapping.
pub(super) fn call(schema: &Schema, column: &Column, casing: Casing) -> String {
    let name = db_name_argument(&column.name, casing);

    // An enum's values live on its declaration, so the builder takes only the
    // database name and needs no type annotation.
    if let Some(item) = enums::lookup(schema, &column.sql_type) {
        return format!("{}({name})", binding(&item.name, casing));
    }

    let builder = types::map(&column.sql_type, column.auto_increment);
    format!(
        "{}({}){}",
        builder.import,
        arguments(&builder, &name),
        type_annotation(&builder, column)
    )
}

/// Renders a builder's arguments, combining the database name with its options.
fn arguments(builder: &Builder, name: &str) -> String {
    match (&builder.options, name.is_empty()) {
        (Some(options), true) => options.clone(),
        (Some(options), false) => format!("{name}, {options}"),
        (None, true) => String::new(),
        (None, false) => name.to_string(),
    }
}

/// Renders an explicit `$type` annotation for builders that need one.
///
/// `json`/`jsonb` default to `unknown`, which does not match the recursive `Json`
/// union `drizzle-zod` infers. Stating the type keeps the Drizzle row type and the
/// Zod schema assignable to each other.
fn type_annotation(builder: &Builder, column: &Column) -> String {
    match builder.import {
        "json" | "jsonb" => format!(
            ".$type<{}>()",
            column
                .json_schema
                .as_ref()
                .map(json_schema::typescript)
                .unwrap_or_else(|| "Record<string, unknown>".to_string())
        ),
        _ => String::new(),
    }
}
