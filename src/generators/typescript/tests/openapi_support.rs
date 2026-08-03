//! Shared helper for reading the generated OpenAPI document.

use super::support::schema;
use crate::generators::typescript::heyapi::render;
use crate::generators::typescript::Casing;
use serde_json::Value;

/// Returns one generated file's contents by name.
pub(super) fn file(name: &str) -> String {
    render(&schema(), Casing::Camel, "/api")
        .into_iter()
        .find(|file| file.name == name)
        .unwrap_or_else(|| panic!("{name} rendered"))
        .contents
}

/// Parses the rendered OpenAPI document.
pub(super) fn document() -> Value {
    serde_json::from_str(&file("openapi.json")).expect("document should parse as JSON")
}
