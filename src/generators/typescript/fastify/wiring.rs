//! Import header and plugin registration for `routes.ts`.

use super::super::casing::{binding, Casing};
use crate::database::introspection::Table;

/// Renders imports shared by every plugin.
pub(super) fn header(tables: &[&Table], casing: Casing) -> String {
    let repositories = tables
        .iter()
        .map(|table| format!("{}Repository", binding(&table.name, casing)))
        .collect::<Vec<_>>()
        .join(", ");

    let schemas = tables
        .iter()
        .flat_map(|table| {
            let name = binding(&table.name, casing);
            [
                format!("{name}SelectSchema"),
                format!("{name}InsertSchema"),
                format!("{name}UpdateSchema"),
            ]
        })
        .collect::<Vec<_>>()
        .join(", ");

    format!(
        "import {{ z }} from \"zod\";\n\
         import type {{ FastifyPluginAsyncZod }} from \"fastify-type-provider-zod\";\n\
         import type {{ Database }} from \"./client\";\n\
         import {{ {repositories} }} from \"./client\";\n\
         import {{ errorSchema, {schemas} }} from \"./zod\";\n\n\
         /** Options every generated plugin accepts. */\n\
         export interface RouteOptions {{\n\tdb: Database;\n}}\n\n"
    )
}

/// Renders a plugin registering every table's routes under its own prefix.
pub(super) fn register(tables: &[&Table], casing: Casing) -> String {
    let registrations = tables
        .iter()
        .map(|table| {
            format!(
                "\tawait app.register({}Routes, {{ ...options, prefix: \"/{}\" }});",
                binding(&table.name, casing),
                table.name
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        "/** Registers every generated route plugin. */\n\
         export const apiRoutes: FastifyPluginAsyncZod<RouteOptions> = async (\n\
         \tapp,\n\
         \toptions,\n\
         ) => {{\n{registrations}\n}};\n"
    )
}
