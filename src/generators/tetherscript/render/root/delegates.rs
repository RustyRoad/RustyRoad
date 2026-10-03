//! The functions forwarding to each CRUD module, and the export list.

use crate::generators::tetherscript::model::Model;

/// Renders the forwarding functions.
///
/// A caller imports the model and reaches every operation through it, rather than
/// having to know which file holds which statement. Both mutating paths validate
/// first, so a bad row cannot reach SQL by calling the submodule's entry point.
pub(super) fn render(model: &Model) -> String {
    let mut file = String::from(
        "// Inserts a row, after validating it.\n\
         fn create(row) {\n\
         \x20   validate(row)?\n\
         \x20   return create_mod.create(row)\n\
         }\n\n\
         // Returns every row.\n\
         fn all() {\n\
         \x20   return read_mod.all()\n\
         }\n\n",
    );

    if model.key().is_some() {
        file.push_str(&keyed());
    }

    file
}

/// Renders the operations that address a single row.
fn keyed() -> String {
    "// Returns the row with `key`, or nil when it does not exist.\n\
     fn find(key) {\n\
     \x20   return read_mod.find(key)\n\
     }\n\n\
     // Updates the row with `key`, after validating the new values.\n\
     fn update(key, row) {\n\
     \x20   validate(row)?\n\
     \x20   return update_mod.update(key, row)\n\
     }\n\n\
     // Deletes the row with `key`, reporting whether one was removed.\n\
     fn delete(key) {\n\
     \x20   return delete_mod.delete(key)\n\
     }\n\n"
        .to_string()
}

/// Renders the export list.
///
/// A name is reachable only when exported, so an omission here makes a generated
/// function invisible rather than producing an error at the call site.
pub(super) fn exports(model: &Model) -> String {
    let mut names = vec![
        "TABLE", "MODEL", "columns", "new", "validate", "create", "all",
    ];

    if model.key().is_some() {
        names.extend(["find", "update", "delete"]);
    }

    names
        .iter()
        .map(|name| format!("export {name}\n"))
        .collect()
}
