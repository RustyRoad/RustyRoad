//! Rust code generation from an introspected Postgres database.
//!
//! This is the Actix/SQLx counterpart to the TypeScript Drizzle/oRPC target. It
//! emits strongly typed row and input models, SQLx repositories, and Actix CRUD
//! procedures from the same introspected schema.

mod models;
mod naming;
mod procedures;
mod repositories;
mod types;
mod writer;

#[cfg(test)]
mod tests;

pub use writer::{write, Outputs};
