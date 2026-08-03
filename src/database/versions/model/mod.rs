//! Schema model used to build versioned views.
//!
//! Captures the subset needed to project a physical table into a versioned view:
//! the physical table name, and for each logical column the physical column
//! backing it plus whether it has been removed from this version.

mod column;
mod table;

pub use column::Column;
pub use table::{Schema, Table};
