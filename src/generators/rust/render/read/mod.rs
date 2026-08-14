//! `read.rs` generation: the queries.

mod find;
mod options;

use crate::database::introspection::Schema;
use crate::generators::rust::model::Model;

/// Renders the read submodule.
pub fn render(model: &Model, schema: &Schema) -> String {
    // The keyed lookup names the key's type, which may be a generated enum.
    let mut imports = super::imports::with_key(model, schema);
    if model.view {
        imports.push_str(&format!("use super::{};\n", model.all_options_name()));
    }
    let mut file = format!("{imports}\nimpl {} {{\n", model.name);

    file.push_str(&all(model));
    if model.view {
        file.push_str(&options::render(model));
    }

    // A keyed lookup needs one column to address the row by. A composite or absent key means
    // `find` cannot be generated, so only the listing is emitted.
    if let Some(key) = model.key() {
        file.push('\n');
        file.push_str(&find::render(model, key));
    }

    file.push_str("}\n");
    file
}

/// Renders the listing.
fn all(model: &Model) -> String {
    format!(
        "\t/// Returns every row.\n\
         \tpub async fn all() -> Result<Vec<Self>, sqlx::Error> {{\n\
         \t\tlet sql = r#\"\n\
         \t\t\tSELECT *\n\
         \t\t\tFROM {table}{order}\n\
         \t\t\"#;\n\n\
         \t\tcrate::db_pool::fetch_all(sqlx::query_as::<_, Self>(sql)).await\n\
         \t}}\n",
        table = model.table,
        order = super::order::clause(model, "\n\t\t\t")
    )
}

/// Returns the parameter type for a key, borrowing where that avoids a clone.
pub fn borrowed(rust: &str) -> String {
    match rust {
        "String" => "&str".to_string(),
        _ => rust.to_string(),
    }
}
