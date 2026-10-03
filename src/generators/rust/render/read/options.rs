//! Filtered listing for a view.

use crate::generators::rust::model::Model;
use crate::generators::rust::render::{order, sql};
use std::fmt::Write as _;

/// Renders a listing whose optional filters are combined with `AND`.
pub(super) fn render(model: &Model) -> String {
    let options = format!("{}AllOptions", model.name);
    let present = model
        .fields
        .iter()
        .map(|field| format!("options.{}.is_some()", field.ident))
        .collect::<Vec<_>>()
        .join("\n\t\t\t|| ");
    let mut filters = String::new();

    for field in &model.fields {
        let _ = writeln!(
            filters,
            "\t\t\tif let Some(value) = options.{} {{\n\
             \t\t\t\tfilters.push(\"{} = \")\n\
             \t\t\t\t\t.push_bind_unseparated(value);\n\
             \t\t\t}}",
            field.ident,
            sql::quote(&field.column),
        );
    }

    let order = order::clause(model, " ");
    let ordering = if order.is_empty() {
        String::new()
    } else {
        format!("\n\t\tquery.push({order:?});")
    };

    format!(
        "\n\t/// Returns rows matching every supplied option.\n\
         \tpub async fn all_options(options: {options}) -> Result<Vec<Self>, sqlx::Error> {{\n\
         \t\tlet mut query = sqlx::QueryBuilder::<sqlx::Postgres>::new(\n\
         \t\t\t\"SELECT * FROM {table}\",\n\
         \t\t);\n\
         \t\tif {present} {{\n\
         \t\t\tquery.push(\" WHERE \");\n\
         \t\t\tlet mut filters = query.separated(\" AND \");\n\
         {filters}\
         \t\t}}{ordering}\n\n\
         \t\tcrate::db_pool::fetch_all(query.build_query_as::<Self>()).await\n\
         \t}}\n",
        table = model.table,
    )
}
