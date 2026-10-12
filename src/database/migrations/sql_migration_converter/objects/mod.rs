//! Parsers for reversible non-table SQL objects.
mod create;
mod do_block;
use super::SqlOperation;

pub(super) fn parse_object(stmt: &str) -> Option<Vec<SqlOperation>> {
    create::parse(stmt).or_else(|| do_block::parse(stmt))
}
