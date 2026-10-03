//! The declarations and re-exports the parent `mod.rs` carries.

use crate::generators::layout::Module;
use crate::generators::rust::casing;

/// Renders the parent `mod.rs` declaring and re-exporting every model.
pub(super) fn render(modules: &[Module]) -> String {
    let mut file = String::from(
        "//! Database models.\n\
         //!\n\
         //! Written once by `rustyroad pull`, then yours: add hand-written models here\n\
         //! alongside the generated ones. A model added to the database later is\n\
         //! reported by `pull` rather than inserted, so your edits survive.\n\n",
    );

    for module in modules {
        file.push_str(&format!("pub mod {};\n", module.name));
    }
    file.push('\n');

    for module in modules {
        file.push_str(&exports(module));
    }

    file
}

/// Renders one module's re-exports.
///
/// A glob rather than the struct alone, because a model may also declare enum types for its
/// columns. Naming only the struct leaves those reachable at `models::product::ProductStatus`
/// but not `models::ProductStatus`, which is where a caller holding a `Product` looks for
/// them — and the field's type is part of the struct's own public API.
fn exports(module: &Module) -> String {
    if module.has_extra_types {
        return format!("pub use {}::*;\n", module.name);
    }
    format!(
        "pub use {}::{};\n",
        module.name,
        casing::to_pascal(&module.name)
    )
}
