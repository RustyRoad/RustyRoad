mod function;
mod trigger;
mod view;

use super::super::SqlOperation;

pub(super) use function::parse as parse_function;
pub(super) use trigger::parse as parse_trigger;
pub(super) use view::parse as parse_view;

pub(super) fn parse(stmt: &str) -> Option<Vec<SqlOperation>> {
    parse_function(stmt)
        .or_else(|| parse_view(stmt))
        .or_else(|| parse_trigger(stmt))
}
