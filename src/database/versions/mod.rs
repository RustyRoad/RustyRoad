//! Versioned schema support: serve multiple schema versions simultaneously.
//!
//! Modelled on pgroll. A migration is applied in two phases:
//!
//! - `start` applies the DDL and publishes a new schema `<schema>_<version>`
//!   containing a view per table. The previous version's schema stays in place, so
//!   clients on either version keep working during rollout.
//! - `complete` finalizes the migration and drops the previous version's schema.
//!
//! An incomplete migration can be rolled back instantly, because the previous
//! version was never removed. History is a linear parent chain with at most one
//! migration in progress, enforced by database constraints.

pub mod exec;
pub mod history;
pub mod introspect;
pub mod lifecycle;
pub mod model;
pub mod naming;
pub mod ops;
pub mod query;
pub mod quote;
pub mod record;
pub mod source;
pub mod stored_plan;
pub mod view;

#[cfg(test)]
mod tests;

pub use history::{Entry, HISTORY_TABLE};
pub use lifecycle::{complete, rollback, start, Started};
pub use model::{Column, Schema, Table};
pub use naming::{deletion_name, temporary_name, versioned_schema};
