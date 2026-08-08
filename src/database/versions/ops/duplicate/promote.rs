//! Promoting a shadow column to replace its original.

use super::Duplicate;
use crate::database::versions::quote::quote_ident;

impl Duplicate<'_> {
    /// Returns the statements promoting the shadow column to the real one.
    ///
    /// Runs at completion: the original is dropped and the shadow takes its name, so
    /// the physical table matches what the new version's views already expose.
    pub fn promote(&self) -> Vec<String> {
        let table = quote_ident(self.table);
        let mut statements = vec![
            format!(
                "ALTER TABLE {table} DROP COLUMN IF EXISTS {}",
                quote_ident(self.column)
            ),
            format!(
                "ALTER TABLE {table} RENAME COLUMN {} TO {}",
                quote_ident(&self.shadow_name()),
                quote_ident(self.column)
            ),
        ];

        // Safe only now that the backfill has given every row a value.
        if !self.nullable {
            statements.push(format!(
                "ALTER TABLE {table} ALTER COLUMN {} SET NOT NULL",
                quote_ident(self.column)
            ));
        }

        statements
    }

    /// Returns the statement removing the shadow column, used on rollback.
    pub fn discard(&self) -> String {
        format!(
            "ALTER TABLE {} DROP COLUMN IF EXISTS {}",
            quote_ident(self.table),
            quote_ident(&self.shadow_name())
        )
    }
}
