//! oRPC router generation.
//!
//! Unifies the API surface on oRPC, following spotlessbinco's convention: each
//! procedure declares `.route({ method, path })`, `.input()`, and `.output()` with
//! the Zod schemas derived from the Drizzle tables. One definition then serves
//! three callers:
//!
//! - a typed RPC client, with no HTTP shape to remember
//! - a REST request, routed by the declared method and path
//! - the OpenAPI document oRPC generates, which Hey API reads
//!
//! Replacing hand-written Fastify plugins with procedures removes the third place
//! a column had to be described.

mod preamble;
mod procedures;
mod router;
mod wiring;

use super::casing::{binding, Casing};
use crate::database::introspection::{Schema, Table};

pub use wiring::{openapi_script, server};

/// Renders `router.ts`.
pub fn render(schema: &Schema, casing: Casing, prefix: &str) -> String {
    let tables: Vec<&Table> = schema
        .tables
        .iter()
        .filter(|table| table.has_simple_key())
        .collect();

    if tables.is_empty() {
        return String::from(
            "// No tables with a single-column primary key were found, \
             so no procedures were generated.\n",
        );
    }

    let mut file = preamble::header(schema, &tables, casing);

    for table in &tables {
        file.push_str(&router::router(table, prefix, casing));
        file.push('\n');
    }

    file.push_str(&root(&tables, casing));
    file
}

/// Renders the root router and its exported type.
fn root(tables: &[&Table], casing: Casing) -> String {
    let members = tables
        .iter()
        .map(|table| {
            let name = binding(&table.name, casing);
            format!("\t{}: {name}Router,", table.name)
        })
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        "/** Root router combining every table's procedures. */\n\
         export const router = {{\n{members}\n}};\n\n\
         /** Router type, for building a typed client. */\n\
         export type AppRouter = typeof router;\n"
    )
}
