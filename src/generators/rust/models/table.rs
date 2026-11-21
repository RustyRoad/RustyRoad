use super::super::{allocation::Names, types};
use super::fields::render as fields;
use super::inputs::{create_type, patch_type};
use crate::database::introspection::{Schema, Table};

/// Render canonical rows and their existing create/patch contracts.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out ./generated-db
/// ```
pub(super) fn render(table: &Table, schema: &Schema, names: &Names) -> String {
    let name = names.row(table);
    let create = names.create(table);
    let patch = names.patch(table);
    let row_fields = fields(
        table,
        names,
        &table.columns,
        |column| types::map_column(table, column, schema, names).rust,
        true,
        false,
    );
    let create_columns = table
        .columns
        .iter()
        .filter(|column| !column.auto_increment)
        .cloned()
        .collect::<Vec<_>>();
    let create_fields = fields(
        table,
        names,
        &create_columns,
        |column| create_type(table, column, schema, names),
        false,
        false,
    );
    let patch_columns = table
        .columns
        .iter()
        .filter(|column| !table.primary_key.contains(&column.name))
        .cloned()
        .collect::<Vec<_>>();
    let patch_fields = fields(
        table,
        names,
        &patch_columns,
        |column| patch_type(table, column, schema, names),
        false,
        true,
    );

    format!(
        "/// A row from the `{table_name}` table.\n\
         #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow, utoipa::ToSchema)]\n\
         pub struct {name} {{\n{row_fields}\n}}\n\n\
         /// Values accepted when creating a `{table_name}` row.\n\
         #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]\n\
         pub struct {create} {{\n{create_fields}\n}}\n\n\
         /// Values accepted when partially updating a `{table_name}` row.\n\
         #[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]\n\
         pub struct {patch} {{\n{patch_fields}\n}}\n\n",
        table_name = table.name,
    )
}
