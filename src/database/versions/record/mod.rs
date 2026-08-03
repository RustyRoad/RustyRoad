//! History mutations: starting, completing, discarding, and baselining.

mod insert;
mod mutate;
mod statement;

pub use insert::{insert_done, start};
pub use mutate::{complete, discard};
