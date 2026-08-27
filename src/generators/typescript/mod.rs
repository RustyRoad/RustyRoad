//! TypeScript code generation from an introspected database.
//!
//! Ports the mechanism Drizzle Kit uses for `drizzle-kit pull`: introspect the
//! live database, then emit a folder of TypeScript. RustyRoad emits:
//!
//! - `schema.ts`   — Drizzle table definitions, constraints, and indexes
//! - `relations.ts`— one/many relations derived from foreign keys
//! - `zod.ts`      — Zod schemas derived from those tables via `drizzle-zod`
//! - `client.ts`   — a typed repository per table
//! - `router.ts`   — oRPC procedures over those repositories
//! - `server.ts`   — Fastify adapter serving the router over RPC and REST
//! - `openapi.ts`  — script writing the OpenAPI document from the router
//! - `openapi/`    — a static OpenAPI document plus a Hey API config
//!
//! Everything traces back to one definition. A column change flows into the Zod
//! schemas, then into procedure validation, the OpenAPI document, and any client
//! generated from it. oRPC is the single API surface: each procedure declares its
//! method and path, so an RPC call and a REST request hit the same handler.

pub mod casing;
pub mod client;
mod columns;
mod constraints;
mod enums;
pub mod heyapi;
mod json_schema;
mod ordering;
pub mod orpc;
pub mod relations;
pub mod schema;
pub mod types;
pub mod writer;
pub mod zod;

#[cfg(test)]
mod tests;

pub use casing::Casing;
pub use writer::{write, Outputs};
