//! Statement construction for installing and removing a trigger.

use super::naming::{function_name, NEEDS_BACKFILL_COLUMN};
use super::{function, Trigger};
use crate::database::versions::quote::quote_ident;

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
            format!(
                "DROP TRIGGER IF EXISTS {name} ON {}",
                quote_ident(self.table)
            ),
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
