//! Zod schema generation via `drizzle-zod`.
//!
//! Rather than hand-writing validators, the schemas are derived from the Drizzle
//! table definitions with `drizzle-zod`. That keeps them in lockstep with the
//! database: a column change flows into validation and into the OpenAPI document
//! without a second source of truth.
//!
//! `fastify-type-provider-zod` then uses these directly for request validation,
//! response serialization, and OpenAPI generation, which is what Hey API reads.

use super::casing::{binding, identifier, to_pascal, Casing};
use super::json_schema;
use crate::database::introspection::{Schema, Table};

/// Renders `zod.ts`.
pub fn render(schema: &Schema, casing: Casing) -> String {
    let mut file = header(schema, casing);

    for table in &schema.tables {
        file.push_str(&schemas(table, casing));
        file.push('\n');
    }

    file
}

/// Renders imports and the shared error schema.
fn header(schema: &Schema, casing: Casing) -> String {
    let tables = schema
        .tables
        .iter()
        .map(|table| binding(&table.name, casing))
        .collect::<Vec<_>>()
        .join(", ");

    format!(
        "import {{ z }} from \"zod\";\n\
         import {{\n\
         \tcreateInsertSchema,\n\
         \tcreateSelectSchema,\n\
         \tcreateUpdateSchema,\n\
         }} from \"drizzle-zod\";\n\
         import {{ {tables} }} from \"./schema\";\n\n\
         /** Body returned when a row is missing. */\n\
         export const errorSchema = z.object({{ error: z.string() }});\n\n"
    )
}

/// Renders the select, insert, and update schemas plus inferred types.
///
/// `createUpdateSchema` already makes every field optional, which is exactly the
/// shape a PATCH body needs.
fn schemas(table: &Table, casing: Casing) -> String {
    let name = binding(&table.name, casing);
    let type_name = to_pascal(&table.name);
    let select = refinements(table, casing, Variant::Select);
    let insert = refinements(table, casing, Variant::Insert);
    let update = refinements(table, casing, Variant::Update);

    format!(
        "export const {name}SelectSchema = createSelectSchema({name}{select});\n\
         export const {name}InsertSchema = createInsertSchema({name}{insert});\n\
         export const {name}UpdateSchema = createUpdateSchema({name}{update});\n\n\
         export type {type_name} = z.infer<typeof {name}SelectSchema>;\n\
         export type New{type_name} = z.infer<typeof {name}InsertSchema>;\n\
         export type Patch{type_name} = z.infer<typeof {name}UpdateSchema>;\n"
    )
}

#[derive(Clone, Copy)]
enum Variant {
    Select,
    Insert,
    Update,
}

/// Builds `drizzle-zod` refinements for database-annotated JSON columns.
fn refinements(table: &Table, casing: Casing, variant: Variant) -> String {
    let entries = table
        .columns
        .iter()
        .filter_map(|column| {
            let schema = column.json_schema.as_ref()?;
            let mut expression = json_schema::zod(schema);
            if column.nullable {
                expression.push_str(".nullable()");
            }
            let optional = match variant {
                Variant::Select => false,
                Variant::Insert => {
                    column.nullable || column.default.is_some() || column.auto_increment
                }
                Variant::Update => true,
            };
            if optional {
                expression.push_str(".optional()");
            }
            Some(format!(
                "{}: {expression}",
                identifier(&column.name, casing)
            ))
        })
        .collect::<Vec<_>>();

    if entries.is_empty() {
        String::new()
    } else {
        format!(", {{ {} }}", entries.join(", "))
    }
}
