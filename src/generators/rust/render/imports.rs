//! The `use super::...` lines each CRUD file opens with.
//!
//! The submodules reach the struct through `super`, and a table keyed by an enum column also
//! names that enum in its method signatures. Enum types live in the shared `enums` module —
//! one declaration per Postgres type — so the key's type is imported from there rather than
//! from the model's own root.

use crate::database::introspection::Schema;
use crate::generators::rust::casing;
use crate::generators::rust::model::Model;

/// Renders the imports for a file that names the key's type in a signature.
///
/// Keyed operations take the primary key as a parameter, so an enum-typed key has to be in
/// scope. Any other key type is either a primitive or a fully-qualified foreign type, and
/// needs no import.
pub fn with_key(model: &Model, schema: &Schema) -> String {
    let mut file = format!("use super::{};\n", model.name);

    // Matched against the schema's enums, so only a type that genuinely lives in the shared
    // module is imported. Guessing from the spelling imported `String` once.
    let declared = model
        .enums(schema)
        .iter()
        .map(|item| casing::to_pascal(&item.name))
        .find(|name| model.key().is_some_and(|key| key.mapping.names(name)));

    if let Some(name) = declared {
        file.push_str(&format!("use super::super::enums::{name};\n"));
    }

    file
}

/// Renders the import for a file that names only the struct.
pub fn struct_only(model: &Model) -> String {
    format!("use super::{};\n", model.name)
}
