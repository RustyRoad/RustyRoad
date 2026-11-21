//! Clock-aware insertion types share the schema-wide collision allocator.
use super::Names;
use crate::database::introspection::Table;

/// Resolve names for generated inputs that can omit required timestamps.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out ./generated-db
/// ```
impl Names {
    /// Return the allocated clock-aware insertion input name.
    /// # Examples
    /// ```sh
    /// rustyroad pull --language rust --out ./generated-db
    /// ```
    pub(in crate::generators::rust) fn clock_input(&self, table: &Table) -> &str {
        &self.types[&(5, table.name.clone())]
    }
}
