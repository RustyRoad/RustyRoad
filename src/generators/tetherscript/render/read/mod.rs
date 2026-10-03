//! `read.tether` generation: the queries.

mod find;

use crate::generators::tetherscript::model::Model;

/// Renders the read submodule.
pub fn render(model: &Model) -> String {
    let mut file = format!(
        "// Queries for the `{}` table.\n\n{}",
        model.table,
        all(model)
    );

    // A keyed lookup needs one column to address the row by, so a composite or absent key
    // means only the listing can be generated.
    match model.key() {
        Some(key) => {
            file.push_str(&find::render(model, &key.column));
            file.push_str("export all\nexport find\n");
        }
        None => {
            file.push_str(&find::unsupported(model));
            file.push_str("export all\n");
        }
    }

    file
}

/// Renders the listing.
fn all(model: &Model) -> String {
    format!(
        "// Returns every row.\n\
         fn all() {{\n\
         \x20   let sql = \"SELECT * FROM {table}{order}\"\n\n\
         \x20   return db.query(sql, [])\n\
         }}\n\n",
        table = model.table,
        order = super::order::clause(model)
    )
}
