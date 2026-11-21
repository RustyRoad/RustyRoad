//! Read-only discovery and adoption planning for handwritten Rust models.
//!
//! Example: `rustyroad --format json model audit --offline`.

mod cli;
mod compare;
mod confirmation;
mod context;
mod coverage;
mod discover;
mod execute;
mod migrations;
mod output;
mod parse;
mod plan;
mod run;
mod schema;
mod select;
mod summary;
mod types;

#[cfg(test)]
mod tests;

pub(crate) use cli::command;
pub(crate) use run::run;
