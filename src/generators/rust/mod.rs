//! Rust API generation from an introspected database.
//!
//! The one Rust generator: the same introspected schema as the TypeScript generator,
//! emitted as a flat `src/db` folder of `sqlx` models (each deriving `ToSchema`), typed
//! repositories, and Actix procedures.

mod api;
pub mod casing;
mod models;
mod naming;
mod procedures;
mod repositories;
pub mod types;

#[cfg(test)]
mod tests;

pub use api::write as write_api;
pub use casing::{singularize, to_pascal, to_snake};
