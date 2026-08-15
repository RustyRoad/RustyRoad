//! The server-side files `pull` emits.

use super::outputs::Outputs;
use crate::database::introspection::Schema;
use crate::generators::ownership::Ownership;
use crate::generators::typescript::casing::Casing;
use crate::generators::typescript::{client, orpc, relations, schema, zod};

/// Path prefix the generated procedures declare.
///
/// Procedure routes, the server mount, and the OpenAPI document all derive from
/// this, so they cannot disagree.
pub const ROUTE_PREFIX: &str = "/api";

/// One file to emit: whether it is enabled, its name, ownership, and contents.
pub(super) struct Emit {
    pub enabled: bool,
    pub name: &'static str,
    pub ownership: Ownership,
    pub contents: String,
}

/// Returns every server-side file.
///
/// `api.ts` is the only scaffold: it is the composition point where generated
/// procedures meet hand-written ones, so it must survive regeneration.
pub(super) fn server(model: &Schema, casing: Casing, outputs: Outputs) -> Vec<Emit> {
    vec![
        generated(outputs.schema, "schema.ts", schema::render(model, casing)),
        generated(
            outputs.relations,
            "relations.ts",
            relations::render(model, casing),
        ),
        generated(outputs.client, "client.ts", client::render(model, casing)),
        generated(outputs.zod, "zod.ts", zod::render(model, casing)),
        generated(
            outputs.api,
            "router.ts",
            orpc::render(model, casing, ROUTE_PREFIX),
        ),
        Emit {
            enabled: outputs.api,
            name: "api.ts",
            ownership: Ownership::Scaffold,
            contents: orpc::composition(model, casing),
        },
        generated(outputs.api, "server.ts", orpc::server()),
        generated(outputs.api, "openapi.ts", orpc::openapi_script()),
    ]
}

/// Builds an entry for a file derived from the database.
fn generated(enabled: bool, name: &'static str, contents: String) -> Emit {
    Emit {
        enabled,
        name,
        ownership: Ownership::Generated,
        contents,
    }
}
