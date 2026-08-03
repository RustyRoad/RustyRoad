//! Backfill triggers keeping old and new columns in sync.
//!
//! While a migration is in flight, both schema versions are served from the same
//! physical table. A write through either version must be visible through the other,
//! so a trigger rewrites each row into the column the writer did not populate.

mod function;
mod naming;

use super::super::quote::quote_ident;

pub use naming::{function_name, Direction, NEEDS_BACKFILL_COLUMN};

/// A trigger keeping one column in sync with its counterpart.
pub struct Trigger<'a> {
    pub table: &'a str,
    /// Column this trigger populates.
    pub physical_column: &'a str,
    pub direction: Direction,
    /// Expression producing the value, referencing `NEW`.
    pub expression: &'a str,
    /// Schema name of the version being served.
    pub latest_schema: &'a str,
}

impl Trigger<'_> {
    /// Returns every statement needed to install the trigger.
    ///
    /// The marker column is added first: the batched backfill uses it to find rows it
    /// has not processed, and the trigger clears it for rows written meanwhile.
    pub fn install(&self) -> Vec<String> {
        let table = quote_ident(self.table);
        let name = quote_ident(&function_name(self.table, self.physical_column));

        vec![
            format!(
                "ALTER TABLE {table} ADD COLUMN IF NOT EXISTS {} BOOLEAN DEFAULT true",
                quote_ident(NEEDS_BACKFILL_COLUMN)
            ),
            function::function(self),
            format!(
                "CREATE OR REPLACE TRIGGER {name}\n  \
                 BEFORE INSERT OR UPDATE ON {table}\n  \
                 FOR EACH ROW EXECUTE PROCEDURE {name}()"
            ),
        ]
    }

    /// Returns the statements removing the trigger and its function.
    pub fn remove(&self) -> Vec<String> {
        let name = quote_ident(&function_name(self.table, self.physical_column));
        vec![
            format!("DROP TRIGGER IF EXISTS {name} ON {}", quote_ident(self.table)),
            format!("DROP FUNCTION IF EXISTS {name}()"),
        ]
    }
}

/// Returns the statement removing the backfill marker column.
///
/// Run at completion, once no trigger depends on it.
pub fn drop_marker(table: &str) -> String {
    format!(
        "ALTER TABLE {} DROP COLUMN IF EXISTS {}",
        quote_ident(table),
        quote_ident(NEEDS_BACKFILL_COLUMN)
    )
}
