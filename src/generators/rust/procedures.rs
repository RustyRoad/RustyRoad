//! Actix CRUD procedure rendering with identity-aware column codecs.
mod handlers;
mod preamble;
mod routes;
mod scaffolds;
use super::allocation::Names;
use crate::database::introspection::Schema;
pub(super) use scaffolds::{composition, module};

/// Prefix shared with the TypeScript API target and generated OpenAPI document.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) const ROUTE_PREFIX: &str = "/api";

/// Render handlers using the same catalog identity and codecs as row models.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn render(schema: &Schema, names: &Names) -> String {
    let tables = schema
        .tables
        .iter()
        .filter(|table| table.has_simple_key())
        .collect::<Vec<_>>();
    let mut output = preamble::render();
    for table in &tables {
        output.push_str(&handlers::render(table, schema, names));
    }
    output.push_str(&routes::render(&tables, names));
    output
}
