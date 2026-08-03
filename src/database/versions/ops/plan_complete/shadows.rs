//! Shadow column promotion and discard at completion.

use super::super::duplicate::Duplicate;
use super::super::Plan;

impl Plan {
    /// Replaces each original column with its shadow.
    pub(super) fn promote_shadows(&self) -> Vec<String> {
        self.promotions
            .iter()
            .flat_map(|(table, column)| duplicate(table, column).promote())
            .collect()
    }

    /// Removes every shadow column, used on rollback.
    pub(super) fn discard_shadows(&self) -> Vec<String> {
        self.promotions
            .iter()
            .map(|(table, column)| duplicate(table, column).discard())
            .collect()
    }
}

/// Builds a duplicator for an existing column.
///
/// Nullability and type are irrelevant here: promotion and discard act on names.
fn duplicate<'a>(table: &'a str, column: &'a str) -> Duplicate<'a> {
    Duplicate {
        table,
        column,
        column_type: None,
        nullable: true,
        default: None,
    }
}
