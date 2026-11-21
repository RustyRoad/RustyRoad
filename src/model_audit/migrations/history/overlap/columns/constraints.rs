//! Inspect only CHECK expressions whose referenced columns are explicit.
use sqlparser::ast::{Expr, Ident, TableConstraint};

/// Return CHECK dependencies, e.g. `CHECK (old IN ('a', 'b'))` touches `old`.
/// Unknown constraints and expressions retain the table-wide history guard.
pub(super) fn columns(constraint: &TableConstraint) -> Option<Vec<Ident>> {
    let TableConstraint::Check(check) = constraint else {
        return None;
    };
    let mut columns = Vec::new();
    collect(&check.expr, &mut columns)?;
    Some(columns)
}

/// Walk a bounded expression grammar; functions and subqueries are not assumed pure.
/// For example, `other > 0` records `other`, but `custom(other)` is unresolved.
fn collect(expression: &Expr, columns: &mut Vec<Ident>) -> Option<()> {
    match expression {
        Expr::Identifier(column) => columns.push(column.clone()),
        Expr::Value(_) => (),
        Expr::Nested(expr)
        | Expr::UnaryOp { expr, .. }
        | Expr::IsNull(expr)
        | Expr::IsNotNull(expr) => collect(expr, columns)?,
        Expr::BinaryOp { left, right, .. } => {
            collect(left, columns)?;
            collect(right, columns)?;
        }
        Expr::InList { expr, list, .. } => {
            collect(expr, columns)?;
            for value in list {
                collect(value, columns)?;
            }
        }
        Expr::Between {
            expr, low, high, ..
        } => {
            collect(expr, columns)?;
            collect(low, columns)?;
            collect(high, columns)?;
        }
        _ => return None,
    }
    Some(())
}
