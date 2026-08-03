//! Catalog queries backing Postgres introspection.

mod columns;
mod constraints;

pub use columns::{COLUMNS, PRIMARY_KEYS};
pub use constraints::{FOREIGN_KEYS, INDEXES, UNIQUES};
