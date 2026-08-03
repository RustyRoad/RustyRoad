//! The server-side files `pull` emits.

use super::outputs::Outputs;
use crate::database::introspection::Schema;
use crate::generators::typescript::casing::Casing;
use crate::generators::typescript::{client, orpc, relations, schema, zod};

/// Path prefix the generated procedures declare.
///
/// Procedure routes, the server mount, and the OpenAPI document all derive from
/// this, so they cannot disagree.
pub const ROUTE_PREFIX: &str = "/api";

/// Returns each server-side file with whether it is enabled.
pub(super) fn server(
    model: &Schema,
    casing: Casing,
    outputs: Outputs,
) -> [(bool, &'static str, String); 7] {
    [
        (outputs.schema, "schema.ts", schema::render(model, casing)),
        (
            outputs.relations,
            "relations.ts",
            relations::render(model, casing),
        ),
        (outputs.client, "client.ts", client::render(model, casing)),
        (outputs.api, "zod.ts", zod::render(model, casing)),
        (
            outputs.api,
            "router.ts",
            orpc::render(model, casing, ROUTE_PREFIX),
        ),
        (outputs.api, "server.ts", orpc::server()),
        (outputs.api, "openapi.ts", orpc::openapi_script()),
    ]
}
