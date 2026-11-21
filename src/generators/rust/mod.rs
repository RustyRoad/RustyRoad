//! Rust API generation from an introspected database.
//!
//! The one Rust generator: the same introspected schema as the TypeScript generator,
//! emitted as a flat `src/db` folder of `sqlx` models (each deriving `ToSchema`), typed
//! repositories, and Actix procedures.

pub mod aggregates;
mod allocation;
mod api;
pub mod balances;
pub mod casing;
pub mod group_metrics;
pub mod json_inserts;
mod models;
mod naming;
mod procedures;
pub mod projections;
pub mod related_writes;
pub mod relation_inserts;
mod repositories;
pub mod spatial_reports;
pub mod types;
pub mod url_stats;

#[cfg(test)]
mod tests;

pub use api::{
    write as write_api, write_with_codecs as write_api_with_codecs,
    write_with_reports as write_api_with_reports,
};
pub use api::{
    write_with_scoped_generation as write_api_with_scoped_generation,
    write_with_scoped_reports as write_api_with_scoped_reports,
};
pub use casing::{singularize, to_pascal, to_snake};
