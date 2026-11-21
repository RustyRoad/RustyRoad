//! Parse whole-row PostgreSQL contracts without executing SQL.
mod relation;
mod returning;
mod select;
#[cfg(test)]
mod tests;
use sqlparser::{
    ast::{SelectItem, Statement},
    dialect::PostgreSqlDialect,
    parser::Parser,
};

/// Only an unmodified wildcard is whole-row evidence, e.g. `RETURNING *`.
fn whole(items: &[SelectItem]) -> bool {
    matches!(items, [SelectItem::Wildcard(options)] if options.to_string().is_empty())
}

/// Extract one physical target, e.g. `SELECT * FROM sessions WHERE id = $1`.
pub(super) fn target(sql: &str) -> Option<String> {
    let statements = Parser::parse_sql(&PostgreSqlDialect {}, sql).ok()?;
    let [statement] = statements.as_slice() else {
        return None;
    };
    match statement {
        Statement::Query(query) if query.with.is_none() => select::target(query),
        _ => returning::target(statement),
    }
}
