//! Read filter types share the row/input collision allocator.
use super::Names;
use crate::database::introspection::{ForeignKey, Table};

/// Access the generated read filter type's allocated name.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out ./generated-db
/// ```
impl Names {
    /// Resolve the filter name independently of row and patch names.
    /// # Examples
    /// ```sh
    /// rustyroad pull --language rust --out ./generated-db
    /// ```
    pub(in crate::generators::rust) fn filter(&self, table: &Table) -> &str {
        &self.types[&(4, table.name.clone())]
    }

    /// Resolve comparison inputs independently of schema rows and other inputs.
    /// # Examples
    /// ```sh
    /// rustyroad pull --language rust --out ./generated-db
    /// ```
    pub(in crate::generators::rust) fn comparison(&self, table: &Table) -> &str {
        &self.types[&(6, table.name.clone())]
    }

    /// Resolve the joined row type for one named foreign-key relationship.
    /// # Examples
    /// ```sh
    /// rustyroad pull --language rust --out ./generated-db
    /// ```
    pub(in crate::generators::rust) fn joined(&self, table: &Table, foreign: &ForeignKey) -> &str {
        &self.types[&(7, format!("{}\0{}", table.name, foreign.name))]
    }
}
