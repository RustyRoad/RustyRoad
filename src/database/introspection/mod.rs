//! Rich database introspection for code generation.
//!
//! Reads the full shape of a schema — tables, columns with types and defaults,
//! primary keys, foreign keys, unique constraints, and indexes — into a
//! backend-neutral model that generators consume.

mod assemble;
pub mod cli;
pub mod model;
mod partial;
pub mod postgres;
pub(crate) mod queries;

pub use model::{Column, Enum, ForeignKey, Index, Schema, Table, Unique};
pub use partial::{read_partial_unique_indexes, PartialUniqueIndex};
pub use postgres::{read, read_for_rust};
