//! TetherScript model generation from an introspected database.
//!
//! The third target alongside TypeScript and Rust, emitting models that reach SQL
//! through TetherScript's `db` capability. The layout matches the Rust generator —
//! one folder per table, a module root plus a file per CRUD verb — but the contents
//! differ in one structural way that is worth stating plainly:
//!
//! TetherScript is dynamically typed and has no structs. A row is a map keyed by
//! column name, so there is no field type to declare and nothing a compiler will
//! check. What the Rust model gets from its struct definition, the TetherScript model
//! has to earn at runtime: `mod.tether` carries the column list, the seeded defaults,
//! and a `validate` that checks required columns and value kinds before any write.
//!
//! Queries bind parameters as a list rather than splicing them into SQL, so a hostile
//! value cannot change the shape of a statement.

pub mod model;
pub mod render;
pub mod types;
pub mod writer;

#[cfg(test)]
mod tests;

pub use model::Model;
pub use types::Kind;
pub use writer::{write, Layout};
