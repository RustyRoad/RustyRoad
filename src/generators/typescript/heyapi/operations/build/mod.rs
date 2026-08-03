//! Building the operation list for a table.

mod read;
mod write;

use super::Operation;
use crate::database::introspection::Table;
use crate::generators::typescript::casing::to_pascal;

/// Builds every operation for a table, in route order.
///
/// Mirrors the five routes the Fastify generator emits, so the document cannot
/// describe an endpoint the server does not serve.
pub(in crate::generators::typescript::heyapi) fn operations(
    table: &Table,
    prefix: &str,
    key_type: &'static str,
) -> Vec<Operation> {
    let entity = to_pascal(&table.name);
    let collection = format!("{prefix}/{}", table.name);
    let single = format!("{collection}/{{id}}");

    vec![
        read::list(&entity, &collection),
        read::get(&entity, &single, key_type),
        write::create(&entity, &collection),
        write::update(&entity, &single, key_type),
        write::delete(&entity, &single, key_type),
    ]
}
