//! Queries over migration history.

mod entries;
mod pointers;

pub use entries::{entry, latest_baseline, parent_of};
pub use pointers::{active, current_version, latest};
