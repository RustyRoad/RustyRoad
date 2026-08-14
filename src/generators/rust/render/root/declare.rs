//! The struct declaration, its submodules, and the enum types it imports.

use super::fields;
use crate::database::introspection::Schema;
use crate::generators::rust::casing;
use crate::generators::rust::model::Model;
use std::fmt::Write as _;

/// Derives every generated model carries.
///
/// `FromRow` is what makes `query_as` work, `ToSchema` puts the model in the OpenAPI document,
/// and `TS` exports the TypeScript interface, so a column reaches the API surface and the
/// frontend types without being described again.
const DERIVES: &str =
    "#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema, TS)]\n#[ts(export)]";

/// Declares the submodules the model actually has.
///
/// The CRUD modules are private, because they contribute `impl` blocks rather than names: the
/// methods they define are reached through the struct, which is already public. `routes` is the
/// exception — the parent's `configure` and utoipa's `paths(...)` both name its handlers.
pub(super) fn submodules(model: &Model) -> String {
    let mut file = if model.view {
        "mod read;\n".to_string()
    } else {
        "mod create;\nmod delete;\nmod read;\nmod update;\n".to_string()
    };

    if model.actix {
        file.push_str("pub mod routes;\n");
    }
    file.push('\n');

    file
}

/// Renders the `use` line for the enum types this model's columns reference.
///
/// From this file `super` is the models root itself, where the shared `enums` module lives —
/// one level shallower than the CRUD submodules, whose path is `super::super`.
pub(super) fn enum_imports(model: &Model, schema: &Schema) -> String {
    let names: Vec<String> = model
        .enums(schema)
        .iter()
        .map(|item| casing::to_pascal(&item.name))
        .collect();

    match names.len() {
        0 => String::new(),
        1 => format!("use super::enums::{};\n", names[0]),
        _ => format!("use super::enums::{{{}}};\n", names.join(", ")),
    }
}

/// Renders the struct itself.
pub(super) fn declaration(model: &Model) -> String {
    format!(
        "{DERIVES}\npub struct {} {{\n{}}}\n",
        model.name,
        fields::render(model)
    )
}

/// Renders the optional filters accepted by a view's `all_options` query.
pub(super) fn all_options(model: &Model) -> String {
    let mut fields = String::new();
    for field in &model.fields {
        let _ = writeln!(
            fields,
            "\tpub {}: Option<{}>,",
            field.ident,
            field.key_type()
        );
    }

    format!(
        "/// Optional equality filters for [`{name}::all_options`].\n\
         #[derive(Debug, Clone, Default)]\n\
         pub struct {options} {{\n{fields}}}\n",
        name = model.name,
        options = model.all_options_name(),
    )
}
