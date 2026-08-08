//! Backfill triggers keeping old and new columns in sync.
//!
//! While a migration is in flight, both schema versions are served from the same
//! physical table. A write through either version must be visible through the other,
//! so a trigger rewrites each row into the column the writer did not populate.

mod function;
mod install;
mod naming;
mod rewrite;

pub use install::drop_marker;
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
    /// Logical-to-physical column rewrite applied to `expression`.
    ///
    /// `NEW` exposes physical columns only, so an expression referencing a logical
    /// name served by a view must be rewritten before it reaches PL/pgSQL.
    pub rewrite: Option<(&'a str, &'a str)>,
}

impl Trigger<'_> {
    /// Returns the expression with logical names resolved to physical ones.
    pub(super) fn resolved_expression(&self) -> String {
        match self.rewrite {
            Some((logical, physical)) => rewrite::to_physical(self.expression, logical, physical),
            None => self.expression.to_string(),
        }
    }
}
