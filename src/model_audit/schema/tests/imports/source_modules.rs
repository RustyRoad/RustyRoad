//! Module ownership fixtures use source files only, never a Cargo subprocess.
mod absolute;
mod contracts;
mod fixtures;
mod globs;
mod ownership;
mod rejected;
mod self_aliases;

use fixtures::{storage, CODEC};
