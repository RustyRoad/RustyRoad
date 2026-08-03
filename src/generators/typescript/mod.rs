//! TypeScript code generation from an introspected database.
//!
//! Ports the mechanism Drizzle Kit uses for `drizzle-kit pull`: introspect the
//! live database, then emit a folder of TypeScript. RustyRoad emits:
//!
//! - `schema.ts`   — Drizzle table definitions, constraints, and indexes
//! - `relations.ts`— one/many relations derived from foreign keys
//! - `zod.ts`      — Zod schemas derived from those tables via `drizzle-zod`
//! - `client.ts`   — a typed repository per table, typed from the Zod schemas
//! - `routes.ts`   — Fastify plugins using `fastify-type-provider-zod`
//!
//! Zod is the single source of validation: the same schemas validate requests,
//! serialize responses, and produce the OpenAPI document that client generators
//! such as Hey API consume. Generation reads only the introspection model.

pub mod casing;
pub mod client;
mod columns;
mod constraints;
pub mod fastify;
mod ordering;
pub mod relations;
pub mod schema;
pub mod types;
pub mod writer;
pub mod zod;

#[cfg(test)]
mod tests;

pub use casing::Casing;
pub use writer::{write, Outputs};
