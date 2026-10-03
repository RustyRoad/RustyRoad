//! Rendering one file of the `products` fixture model.

use super::support::{products, resolve, schema};
use crate::generators::tetherscript::render;

/// Renders one file of the `products` model.
pub(super) fn file(name: &str) -> String {
    let schema = schema();
    let model = resolve(&products(), &schema);

    render::files(&model)
        .into_iter()
        .find(|file| file.name == name)
        .unwrap_or_else(|| panic!("{name} should be emitted"))
        .contents
}
