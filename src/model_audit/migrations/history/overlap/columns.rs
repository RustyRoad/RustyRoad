//! Scope history conflicts only when every operation is demonstrably column-local.

mod constraints;
mod operations;
use super::sql;
use crate::model_audit::schema::types::Proposal;
use sqlparser::{ast::Statement, dialect::PostgreSqlDialect, parser::Parser};

/// Compare simple repairs, e.g. `ALTER COLUMN old DROP NOT NULL` permits adding new.
/// Unknown syntax, USING expressions and whole-table proposals retain raw checks.
pub(super) fn conflicts(
    source: &str,
    schema: &str,
    proposal: &Proposal,
) -> Option<Result<bool, String>> {
    if proposal.operation == "create_table" || proposal.columns.is_empty() {
        return None;
    }
    let statements = Parser::parse_sql(&PostgreSqlDialect {}, source).ok()?;
    if statements.is_empty() {
        return None;
    }
    let mut overlap = false;
    for statement in statements {
        let Statement::AlterTable(table) = statement else {
            return None;
        };
        if table.operations.is_empty() {
            return None;
        }
        for operation in table.operations {
            let columns = operations::columns(operation)?;
            overlap |= sql::same_table(&table.name.to_string(), schema, &proposal.table)
                && columns.iter().any(|column| {
                    proposal
                        .columns
                        .iter()
                        .any(|name| sql::same_column(&column.to_string(), name))
                });
        }
    }
    Some(Ok(overlap))
}

#[cfg(test)]
mod tests;
