//! `Default` and `new`.

mod zero;

use crate::generators::rust::model::Model;

/// Renders `Default`, so a caller can build a row and set only what it cares about.
pub(super) fn default_impl(model: &Model) -> String {
    let body = model
        .fields
        .iter()
        .map(|field| format!("\t\t\t{}: {},", field.ident, zero::value(field)))
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        "impl Default for {} {{\n\
         \tfn default() -> Self {{\n\
         \t\tSelf {{\n{body}\n\t\t}}\n\
         \t}}\n\
         }}\n",
        model.name
    )
}

/// Renders `new`, taking every field a caller supplies.
///
/// The generated key and the audit timestamps are omitted: the database sets them, so
/// accepting them would invite a caller to pass a value that the insert discards.
pub(super) fn new(model: &Model) -> String {
    let parameters = model
        .insertable()
        .iter()
        .map(|field| format!("\t\t{}: {},", field.ident, field.mapping.rust))
        .collect::<Vec<_>>()
        .join("\n");

    let assignments = model
        .fields
        .iter()
        .map(|field| {
            if field.insertable() {
                format!("\t\t\t{},", field.ident)
            } else {
                format!("\t\t\t{}: {},", field.ident, zero::value(field))
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        "impl {} {{\n\
         \t/// Builds a row from the values a caller supplies.\n\
         \t///\n\
         \t/// The primary key and the audit timestamps are omitted: the database\n\
         \t/// assigns them, so a value passed here would be discarded on insert.\n\
         \tpub fn new(\n{parameters}\n\t) -> Self {{\n\
         \t\tSelf {{\n{assignments}\n\t\t}}\n\
         \t}}\n\
         }}\n",
        model.name
    )
}
