//! Annotations for the routes that address one row by its key.

mod read;
mod write;

use super::{parameter, NOT_FOUND};

pub(in crate::generators::rust::actix) use read::find;
pub(in crate::generators::rust::actix) use write::{remove, update};
