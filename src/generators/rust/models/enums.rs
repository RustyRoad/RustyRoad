use super::super::{allocation::Names, naming::literal};
use crate::database::introspection::Enum;

/// Render the existing SQL and serde enum labels with allocated Rust names.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out ./generated-db
/// ```
pub(super) fn render(item: &Enum, names: &Names) -> String {
    let allocated = Names::variants(&item.values);
    let variants = item
        .values
        .iter()
        .map(|value| {
            let variant = &allocated[value];
            format!(
                "    #[sqlx(rename = {})]\n    #[serde(rename = {})]\n    {},",
                literal(value),
                literal(value),
                variant
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        "#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, sqlx::Type, utoipa::ToSchema)]\n\
         #[sqlx(type_name = {})]\n\
         pub enum {} {{\n{}\n}}\n\n",
        literal(&item.name),
        names.enum_type(&item.name),
        variants
    )
}
