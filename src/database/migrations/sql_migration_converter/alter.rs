//! `ALTER TABLE` identifier parsing and reversal.
mod action;
mod assemble;
mod parse;
pub(super) use action::{clean, qualified, IDENT};
pub(super) use parse::{parse_alter_table, split_alter};
