//! Decoding query result values for display.
//!
//! Shared by the `query` and `db schema` commands, which both need to render
//! arbitrary rows without knowing their types ahead of time.

mod arrays;
mod ladder;
pub mod postgres;
mod scalars;

pub use postgres::{pg_display, pg_row};
