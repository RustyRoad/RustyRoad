//! Validated per-column codecs with no array-element or same-type propagation.
use super::Names;
use crate::database::introspection::{Column, Schema, Table};
use crate::generators::rust::types::{column_validation, ColumnOverride, Override};
use std::io;

/// Attach scoped contracts and retain catalog identities through cloned columns.
/// # Examples
/// ```sh
/// rustyroad pull --schema public --language rust --out crates/db/src
/// ```
impl Names {
    /// Validate all scoped targets before any renderer or output writer runs.
    /// # Examples
    /// ```sh
    /// rustyroad pull --schema public --language rust --out crates/db/src
    /// ```
    pub(in crate::generators::rust) fn with_columns(
        mut self,
        schema: &Schema,
        namespace: &str,
        entries: &[ColumnOverride],
    ) -> io::Result<Self> {
        self.column_codecs = column_validation::validate(schema, namespace, entries)?;
        Ok(self)
    }

    /// Resolve only the supplied owning relation and original catalog column name.
    /// # Examples
    /// ```sh
    /// rustyroad pull --schema public --language rust --out crates/db/src
    /// ```
    pub(in crate::generators::rust) fn column_codec(
        &self,
        table: &Table,
        column: &Column,
    ) -> Option<&Override> {
        self.column_codecs
            .get(&(table.name.clone(), column.name.clone()))
    }

    /// Apply the scoped OpenAPI contract without leaking a global schema override.
    /// Actual Option/nested Option wrappers remain attached to the field type.
    /// # Examples
    /// ```sh
    /// rustyroad pull --schema public --language rust --out crates/db/src
    /// ```
    pub(in crate::generators::rust) fn column_schema(
        &self,
        table: &Table,
        column: &Column,
        field_type: &str,
    ) -> Option<String> {
        match self.column_codec(table, column) {
            Some(codec) => codec
                .schema
                .as_ref()
                .map(|schema| field_type.replace(&codec.rust, schema)),
            None => self.codec_schema(&column.sql_type, field_type),
        }
    }
}
