//! Fastify route generation with Zod validation.
//!
//! Emits one Fastify plugin per table using `fastify-type-provider-zod`, so the
//! Zod schemas derived from the Drizzle tables serve three jobs at once: request
//! validation, response serialization, and OpenAPI generation. Hey API then reads
//! that OpenAPI document, which means the client, the server, and the database all
//! trace back to one definition.

mod read;
mod routes;
mod wiring;
mod write;

use super::casing::Casing;
use crate::database::introspection::{Schema, Table};

/// Renders `routes.ts`.
pub fn render(schema: &Schema, casing: Casing) -> String {
    let tables: Vec<&Table> = schema
        .tables
        .iter()
        .filter(|table| table.has_simple_key())
        .collect();

    if tables.is_empty() {
        return String::from(
            "// No tables with a single-column primary key were found, \
             so no routes were generated.\n",
        );
    }

    let mut file = wiring::header(&tables, casing);
    for table in &tables {
        file.push_str(&routes::plugin(table, casing));
        file.push('\n');
    }
    file.push_str(&wiring::register(&tables, casing));

    file
}
