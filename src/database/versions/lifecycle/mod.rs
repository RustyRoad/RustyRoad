//! Two-phase migration lifecycle.
//!
//! `start` applies the migration's DDL and publishes a new versioned schema while
//! leaving the previous version intact, so clients on either version keep working.
//! `complete` finalizes and drops the previous version's schema. `rollback` undoes
//! a started-but-incomplete migration, which is instant because the previous
//! version was never removed.

mod finish;
mod publish;
mod start;

use crate::database::migrations::CustomMigrationError;
use std::io;

pub use finish::{complete, rollback};
pub use publish::supports_versions;
pub use start::{start, Started};

/// Builds an error for an invalid lifecycle transition.
fn conflict(message: String) -> CustomMigrationError {
    CustomMigrationError::IoError(io::Error::new(io::ErrorKind::InvalidInput, message))
}
