//! The `use super::...` lines each generated handler file opens with.

use crate::database::introspection::Schema;
use crate::generators::rust::casing;
use crate::generators::rust::model::Model;

/// Renders the imports the handlers need.
///
/// A table keyed by an enum column names that enum in every keyed signature, and enum types
/// live in the shared `enums` module — so the import is derived from what the model actually
/// references rather than assumed to be the struct alone.
pub(super) fn render(model: &Model, schema: &Schema) -> String {
    let mut file = format!(
        "use super::{};\nuse actix_web::{{web, HttpResponse, Responder}};\n",
        model.name
    );

    if let Some(name) = key_enum(model, schema) {
        file.push_str(&format!("use super::super::enums::{name};\n"));
    }
    file.push('\n');

    file
}

/// Returns the shared enum type the primary key resolves to, if it is one.
///
/// Matched against the schema's enums rather than guessed from the type's spelling: `String`
/// looks exactly like a generated type, and importing it produced 141 unresolved imports on the
/// live schema when the model layer made the same mistake.
fn key_enum(model: &Model, schema: &Schema) -> Option<String> {
    let key = model.key()?;

    model
        .enums(schema)
        .iter()
        .map(|item| casing::to_pascal(&item.name))
        .find(|name| key.mapping.names(name))
}
