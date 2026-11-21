//! Compare parsed canonical migration operations with one proposed table change.

mod columns;
mod creation;
use super::{raw, sql};
use crate::{database::migrations::SqlOperation, model_audit::schema::types::Proposal};

#[cfg(test)]
mod tests;

/// Recognize existing table/column work, e.g. `ADD COLUMN name` blocks its duplicate.
pub(super) fn conflicts(
    operation: &SqlOperation,
    schema: &str,
    proposal: &Proposal,
) -> Result<bool, String> {
    let target = |name: &str| sql::same_table(name, schema, &proposal.table);
    Ok(match operation {
        SqlOperation::CreateTable {
            table_name,
            columns,
            ..
        } => target(table_name) && creation::conflicts(columns, schema, proposal)?,
        SqlOperation::AlterTableAddColumn {
            table_name,
            columns,
            ..
        } => {
            target(table_name)
                && (proposal.operation == "create_table"
                    || columns.iter().any(|column| {
                        proposal
                            .columns
                            .iter()
                            .any(|name| sql::same_column(column, name))
                    }))
        }
        SqlOperation::RawSql { sql } => columns::conflicts(sql, schema, proposal)
            .unwrap_or_else(|| raw::overlaps(sql, schema, &proposal.table))?,
        SqlOperation::AlterTableDropColumn { table_name, .. }
        | SqlOperation::DropTable { table_name, .. } => target(table_name),
        _ => false,
    })
}
