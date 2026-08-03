//! Imports, context, and helpers shared by the generated routers.

mod helpers;

use crate::database::introspection::{Schema, Table};
use crate::generators::typescript::casing::{binding, Casing};

/// Renders the file header: imports, context type, base builder, and helpers.
pub(super) fn header(schema: &Schema, tables: &[&Table], casing: Casing) -> String {
    format!(
        "import {{ os, ORPCError }} from \"@orpc/server\";\n\
         import {{ z }} from \"zod\";\n\
         import type {{ Database }} from \"./client\";\n\
         import {{ {} }} from \"./client\";\n\
         import {{ {} }} from \"./zod\";\n\n\
         {}\n{}\n{}",
        repositories(tables, casing),
        schemas(schema, casing),
        helpers::context(),
        helpers::base(),
        helpers::helpers(),
    )
}

/// Renders the repository import list.
fn repositories(tables: &[&Table], casing: Casing) -> String {
    tables
        .iter()
        .map(|table| format!("{}Repository", binding(&table.name, casing)))
        .collect::<Vec<_>>()
        .join(", ")
}

/// Renders the Zod schema import list.
fn schemas(schema: &Schema, casing: Casing) -> String {
    schema
        .tables
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
        .join(", ")
}
