//! Distinguish historical table creation from pending duplicate column additions.
use super::sql;
use crate::{database::migrations::SqlOperation, model_audit::schema::types::Proposal};

/// Historical creation blocks duplicate creates/additions, not catalog-backed repairs.
/// For example, `CREATE TABLE rows(id bigint)` blocks `ADD id`, not `ALTER id`.
pub(super) fn conflicts(
    columns: &[String],
    schema: &str,
    proposal: &Proposal,
) -> Result<bool, String> {
    if proposal.operation == "create_table" {
        return Ok(true);
    }
    for operation in sql::operations(&proposal.up)? {
        if let SqlOperation::AlterTableAddColumn {
            table_name,
            columns: additions,
            ..
        } = operation
        {
            if sql::same_table(&table_name, schema, &proposal.table)
                && proposal.columns.iter().any(|name| {
                    additions
                        .iter()
                        .any(|column| sql::same_column(column, name))
                        && columns.iter().any(|column| sql::same_column(column, name))
                })
            {
                return Ok(true);
            }
        }
    }
    Ok(false)
}
