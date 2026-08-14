//! `create.rs` generation: the insert.

mod statement;

use crate::generators::rust::model::Model;

/// Renders the create submodule.
pub fn render(model: &Model) -> String {
    let supplied = model.insertable();

    // A table whose every column is database-assigned has nothing to bind, so the insert
    // names no columns and takes its defaults.
    if supplied.is_empty() {
        return empty(model);
    }

    statement::render(model, &supplied)
}

/// Renders the insert for a table with no caller-supplied columns.
fn empty(model: &Model) -> String {
    format!(
        "use super::{name};\n\n\
         impl {name} {{\n\
         \t/// Inserts a row consisting entirely of database-assigned values.\n\
         \tpub async fn create() -> Result<Self, sqlx::Error> {{\n\
         \t\tlet sql = r#\"INSERT INTO {table} DEFAULT VALUES RETURNING *\"#;\n\n\
         \t\tcrate::db_pool::fetch_one(sqlx::query_as::<_, Self>(sql)).await\n\
         \t}}\n\
         }}\n",
        name = model.name,
        table = model.table
    )
}
