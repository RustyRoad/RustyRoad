//! Column duplication: create a shadow physical column beside the original.
//!
//! An in-flight change writes to a shadow column rather than mutating the original,
//! so readers of the old schema version keep seeing the original values. The two are
//! kept in sync by triggers until the migration completes.

mod promote;

use super::super::naming::temporary_name;
use super::super::quote::quote_ident;

/// A column to duplicate, and the shape the copy should take.
pub struct Duplicate<'a> {
    pub table: &'a str,
    pub column: &'a str,
    /// Type of the shadow column; defaults to text when unknown.
    pub column_type: Option<&'a str>,
    pub nullable: bool,
    pub default: Option<&'a str>,
}

impl Duplicate<'_> {
    /// Returns the physical name of the shadow column.
    pub fn shadow_name(&self) -> String {
        temporary_name(self.column)
    }

    /// Returns the statement adding the shadow column.
    ///
    /// The shadow is always created nullable: existing rows have no value yet, and a
    /// NOT NULL constraint would fail before the backfill has run. The constraint is
    /// applied at completion instead.
    pub fn add_column(&self) -> String {
        let mut sql = format!(
            "ALTER TABLE {} ADD COLUMN IF NOT EXISTS {} {}",
            quote_ident(self.table),
            quote_ident(&self.shadow_name()),
            self.column_type.unwrap_or("text")
        );

        if let Some(default) = self.default {
            sql.push_str(&format!(" DEFAULT {default}"));
        }

        sql
    }
}
