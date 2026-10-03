//! `update.rs` generation: the update.

mod statement;

use crate::database::introspection::Schema;
use crate::generators::rust::model::Model;

/// Renders the update submodule.
pub fn render(model: &Model, schema: &Schema) -> String {
    let Some(key) = model.key() else {
        return unsupported(
            model,
            "a single-column primary key is needed to address the row to update",
        );
    };

    let writable = model.updatable();
    if writable.is_empty() {
        return unsupported(model, "every column is either the key or database-assigned");
    }

    statement::render(model, key, &writable, schema)
}

/// Renders a placeholder explaining why no update was generated.
///
/// An empty file would look like an oversight; stating the reason in the generated output
/// puts the explanation where the developer is already looking.
fn unsupported(model: &Model, reason: &str) -> String {
    format!(
        "//! No update was generated for `{}`: {reason}.\n\
         //!\n\
         //! Add one here if the table needs it; this file is regenerated, so prefer\n\
         //! a sibling module for code you intend to keep.\n",
        model.table
    )
}
