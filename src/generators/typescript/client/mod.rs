//! Typed TypeScript client generation.
//!
//! Emits a CRUD repository per table over the generated Drizzle schema.
//!
//! Row types are inferred from the Drizzle tables rather than from the Zod
//! schemas. The two disagree on a few column types, so the repository — which
//! returns whatever the driver produced — must be typed by Drizzle. The Zod
//! schemas remain the contract at the HTTP boundary, where they validate and
//! serialize.

mod read;
mod repository;
mod rows;
mod write;

use super::casing::{binding, Casing};
use crate::database::introspection::Schema;

pub(super) use rows::{key_is_numeric, quoted_property};

/// Renders `client.ts`.
pub fn render(schema: &Schema, casing: Casing) -> String {
    let mut file = header(schema, casing);

    for table in &schema.tables {
        file.push_str(&rows::row_types(table, casing));
    }
    file.push('\n');

    for table in schema.tables.iter().filter(|t| !t.primary_key.is_empty()) {
        file.push_str(&repository::repository(table, casing));
        file.push('\n');
    }

    file
}

/// Renders imports, the database handle, and the client factory.
fn header(schema: &Schema, casing: Casing) -> String {
    let tables = schema
        .tables
        .iter()
        .map(|table| binding(&table.name, casing))
        .collect::<Vec<_>>()
        .join(", ");

    format!(
        "import {{ drizzle }} from \"drizzle-orm/node-postgres\";\n\
         import {{ eq }} from \"drizzle-orm\";\n\
         import type {{ InferSelectModel, InferInsertModel }} from \"drizzle-orm\";\n\
         import {{ {tables} }} from \"./schema\";\n\n\
         /** Database handle shared by every repository below. */\n\
         export type Database = ReturnType<typeof drizzle>;\n\n\
         /** Opens a client against `connectionString`. */\n\
         export function createClient(connectionString: string): Database {{\n\
         \treturn drizzle(connectionString);\n\
         }}\n\n"
    )
}
