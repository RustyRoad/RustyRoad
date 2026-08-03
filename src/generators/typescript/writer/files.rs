//! The server-side files `pull` emits.

use super::outputs::Outputs;
use crate::database::introspection::Schema;
use crate::generators::typescript::casing::Casing;
use crate::generators::typescript::{client, fastify, relations, schema, zod};

/// Returns each server-side file with whether it is enabled.
pub(super) fn server(
    model: &Schema,
    casing: Casing,
    outputs: Outputs,
) -> [(bool, &'static str, String); 5] {
    [
        (outputs.schema, "schema.ts", schema::render(model, casing)),
        (
            outputs.relations,
            "relations.ts",
            relations::render(model, casing),
        ),
        (outputs.client, "client.ts", client::render(model, casing)),
        (outputs.api, "zod.ts", zod::render(model, casing)),
        (outputs.api, "routes.ts", fastify::render(model, casing)),
    ]
}
