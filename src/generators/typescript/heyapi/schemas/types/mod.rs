//! JSON Schema type mapping for OpenAPI components.

mod formats;

use crate::database::introspection::{Column, Schema};
use crate::generators::typescript::json_schema;

/// Renders one property's JSON Schema type.
///
/// A nullable column is a type union rather than an omitted optional, so the
/// generated client sees `T | null` instead of `T | undefined`.
pub(super) fn property(schema: &Schema, column: &Column) -> String {
    // An enum's allowed values are the useful part of its type, so they are stated
    // rather than collapsing the column to a plain string.
    if let Some(item) = schema.enum_type(column.sql_type.trim().trim_matches('"')) {
        return enum_property(item, column.nullable);
    }

    if let Some(schema) = column.json_schema.as_ref() {
        return json_schema::openapi(schema, column.nullable);
    }

    let base = formats::base_type(&column.sql_type);
    let types = if column.nullable {
        format!("[\"{base}\", \"null\"]")
    } else {
        format!("\"{base}\"")
    };

    match formats::format_of(&column.sql_type) {
        Some(format) => format!("{{ \"type\": {types}, \"format\": \"{format}\" }}"),
        None => format!("{{ \"type\": {types} }}"),
    }
}

/// Renders an enum property, listing its allowed values.
///
/// `null` is added to the value list as well as the type, since a validator checks
/// membership before nullability.
fn enum_property(item: &crate::database::introspection::Enum, nullable: bool) -> String {
    let mut values = item
        .values
        .iter()
        .map(|value| format!("\"{}\"", value.replace('"', "\\\"")))
        .collect::<Vec<_>>();

    let types = if nullable {
        values.push("null".to_string());
        "[\"string\", \"null\"]"
    } else {
        "\"string\""
    };

    format!("{{ \"type\": {types}, \"enum\": [{}] }}", values.join(", "))
}
