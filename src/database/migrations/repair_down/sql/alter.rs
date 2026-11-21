//! Reverse unconditional column additions, never infer missing prior definitions.

use sqlparser::ast::{AlterTable, AlterTableOperation};

pub(super) fn table(table: &AlterTable) -> Result<Vec<String>, String> {
    if table.if_exists {
        return Err(super::conditional());
    }
    if table.location.is_some() || table.on_cluster.is_some() || table.table_type.is_some() {
        return Err("Nonstandard ALTER TABLE options require reviewed undo".into());
    }
    super::names::check(&table.name, 2)?;
    if table.operations.is_empty() {
        return Err("ALTER TABLE has no reversible operations".into());
    }
    let mut undo = Vec::new();
    for operation in table.operations.iter().rev() {
        match operation {
            AlterTableOperation::AddColumn {
                if_not_exists: true,
                ..
            } => {
                return Err(super::conditional());
            }
            AlterTableOperation::AddColumn {
                if_not_exists: false,
                column_def,
                column_position: None,
                ..
            } => {
                super::names::identifier(&column_def.name)?;
                let only = if table.only { "ONLY " } else { "" };
                undo.push(format!(
                    "ALTER TABLE {only}{} DROP COLUMN {} RESTRICT;",
                    table.name, column_def.name,
                ));
            }
            _ => return Err("Only unconditional ADD COLUMN is inferred for ALTER TABLE".into()),
        }
    }
    Ok(undo)
}
