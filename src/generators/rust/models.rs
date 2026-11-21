//! Rust model rendering with focused header, row and field generators.
mod enums;
mod fields;
mod inputs;
mod preamble;
mod table;

use super::allocation::Names;
use crate::database::introspection::Schema;

/// Render generated rows and inputs with their schema-derived codecs.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out ./generated-db
/// ```
pub(super) fn render(schema: &Schema, names: &Names) -> String {
    let mut output = preamble::render();
    for item in &schema.enums {
        output.push_str(&enums::render(item, names));
    }
    for relation in &schema.tables {
        output.push_str(&table::render(relation, schema, names));
    }
    output
}
