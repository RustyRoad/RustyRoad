//! Handlers that address one row by its key.

mod read;
mod write;

pub(in crate::generators::rust::actix) use read::find;
pub(in crate::generators::rust::actix) use write::{remove, update};
