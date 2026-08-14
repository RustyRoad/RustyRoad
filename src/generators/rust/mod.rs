//! Rust model generation from an introspected database.
//!
//! The companion to the TypeScript generator: the same introspected schema, emitted
//! as `sqlx` models instead of Drizzle tables. Generation follows the layout the
//! hand-written models use, so generated code is indistinguishable from code written
//! by hand — one folder per table, holding:
//!
//! - `mod.rs`    — the struct, its derives, `Default`, and `new`
//! - `create.rs` — the insert
//! - `read.rs`   — the listing and the keyed lookup
//! - `update.rs` — the update
//! - `delete.rs` — the delete
//!
//! Each CRUD file contributes an `impl` block to the struct the module root declares
//! and reaches it through `use super::<Name>`. Splitting by verb keeps every file
//! short enough to read whole, and means a developer looking for one statement knows
//! which file holds it before opening anything.

pub mod actix;
pub mod casing;
pub mod model;
pub mod render;
pub mod types;
pub mod writer;

#[cfg(test)]
mod tests;

pub use casing::{singularize, to_pascal, to_snake};
pub use model::Model;
pub use writer::{write, write_with, Layout, Options};
