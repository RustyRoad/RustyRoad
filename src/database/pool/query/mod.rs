//! Helpers that run a prepared `sqlx` query on the shared pool.
//!
//! Each takes the query rather than SQL text, so callers keep `sqlx`'s binding and decoding
//! and only the executor comes from here. Split by what the query decodes into: mapped rows,
//! single-column scalars, or raw `PgRow`s.

mod raw;
mod scalar;
mod typed;

pub use raw::{execute, query_all, query_one, query_optional};
pub use scalar::{scalar_all, scalar_one, scalar_optional};
pub use typed::{fetch_all, fetch_one, fetch_optional};
