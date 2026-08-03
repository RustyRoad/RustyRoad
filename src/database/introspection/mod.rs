//! Rich database introspection for code generation.
//!
//! Reads the full shape of a schema — tables, columns with types and defaults,
//! primary keys, foreign keys, unique constraints, and indexes — into a
//! backend-neutral model that generators consume.

mod assemble;
pub mod cli;
pub mod model;
pub mod postgres;
mod queries;

pub use model::{Column, ForeignKey, Index, Schema, Table, Unique};
pub use postgres::read;
