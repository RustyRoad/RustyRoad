//! JSON Schema type mapping for OpenAPI components.

mod formats;

use crate::database::introspection::Column;

/// Renders one property's JSON Schema type.
///
/// A nullable column is a type union rather than an omitted optional, so the
/// generated client sees `T | null` instead of `T | undefined`.
pub(super) fn property(column: &Column) -> String {
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
