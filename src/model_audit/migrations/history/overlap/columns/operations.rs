//! Identify operations whose history impact is confined to explicit columns.
use sqlparser::ast::{AlterColumnOperation, AlterTableOperation, Expr, Ident};

/// Return columns for simple repairs; expressions and unknown operations stay raw.
pub(super) fn columns(operation: AlterTableOperation) -> Option<Vec<Ident>> {
    Some(match operation {
        AlterTableOperation::AddConstraint { constraint, .. } => {
            super::constraints::columns(&constraint)?
        }
        AlterTableOperation::AlterColumn {
            column_name,
            op:
                AlterColumnOperation::SetDefault {
                    value: Expr::Value(_),
                },
        } => vec![column_name],
        AlterTableOperation::AlterColumn { column_name, op }
            if matches!(
                op,
                AlterColumnOperation::SetNotNull
                    | AlterColumnOperation::DropNotNull
                    | AlterColumnOperation::DropDefault
                    | AlterColumnOperation::SetDataType { using: None, .. }
            ) =>
        {
            vec![column_name]
        }
        _ => return None,
    })
}
