//! Naming for backfill triggers and their marker column.

/// Column marking rows the backfill has not yet processed.
pub const NEEDS_BACKFILL_COLUMN: &str = "_rustyroad_needs_backfill";

/// Prefix for generated trigger functions and triggers.
const TRIGGER_PREFIX: &str = "_rustyroad_trigger";

/// Direction a trigger rewrites values in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    /// Populates the new column from the old, for readers of the new version.
    Up,
    /// Populates the old column from the new, for readers of the old version.
    Down,
}

impl Direction {
    /// Returns the direction's SQL-visible label.
    pub fn label(self) -> &'static str {
        match self {
            Self::Up => "up",
            Self::Down => "down",
        }
    }
}

/// Returns the trigger function name for a table and physical column.
pub fn function_name(table: &str, physical_column: &str) -> String {
    format!("{TRIGGER_PREFIX}_{table}_{physical_column}")
}
