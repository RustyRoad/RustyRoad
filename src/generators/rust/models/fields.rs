use super::super::{allocation::Names, naming::literal};
use super::inputs::is_optional_input;
use crate::database::introspection::{Column, Table};

/// Render field attributes and match network OpenAPI types to their string serialization.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out ./generated-db
/// ```
pub(super) fn render<F>(
    table: &Table,
    names: &Names,
    columns: &[Column],
    type_for: F,
    sqlx: bool,
    all_optional: bool,
) -> String
where
    F: Fn(&Column) -> String,
{
    columns
        .iter()
        .map(|column| {
            let field = names.field(table, &column.name);
            let mut attributes = Vec::new();
            if field != column.name {
                if sqlx {
                    attributes.push(format!("    #[sqlx(rename = {})]", literal(&column.name)));
                }
                attributes.push(format!("    #[serde(rename = {})]", literal(&column.name)));
            }
            if !sqlx && (all_optional || is_optional_input(column)) {
                attributes.push(
                    "    #[serde(default, skip_serializing_if = \"Option::is_none\")]".to_string(),
                );
            }
            let field_type = type_for(column);
            if let Some(wire_type) = names.column_schema(table, column, &field_type) {
                attributes.push(format!("    #[schema(value_type = {wire_type})]"));
            } else if names.column_codec(table, column).is_none()
                && field_type.contains("ipnetwork::IpNetwork")
            {
                let wire_type = field_type.replace("ipnetwork::IpNetwork", "String");
                attributes.push(format!("    #[schema(value_type = {wire_type})]"));
            }
            let prefix = if attributes.is_empty() {
                String::new()
            } else {
                format!("{}\n", attributes.join("\n"))
            };
            format!("{prefix}    pub {field}: {field_type},")
        })
        .collect::<Vec<_>>()
        .join("\n")
}
