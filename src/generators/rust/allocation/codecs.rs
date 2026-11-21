//! Codec configuration shared by row, input and repository renderers.
use super::{functions, Names};
use crate::database::introspection::{PartialUniqueIndex, Schema};
use crate::generators::rust::types::{overrides, Override};
use std::{collections::BTreeMap, io};

/// Resolve explicitly configured codecs without changing SQL identities.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
impl Names {
    /// Validate codecs before constructing the common generation plan.
    /// # Examples
    /// ```sh
    /// rustyroad pull --language rust --out crates/db/src
    /// ```
    pub(in crate::generators::rust) fn with_codecs(
        schema: &Schema,
        entries: &BTreeMap<String, Override>,
    ) -> io::Result<Self> {
        let codecs = overrides::validate(entries)?;
        let mut names = Self::new(schema);
        names.codecs = codecs;
        Ok(names)
    }

    /// Allocates partial-index operations alongside the ordinary codec-aware namespace.
    /// # Examples
    /// ```sh
    /// rustyroad pull --language rust --out crates/db/src
    /// ```
    pub(in crate::generators::rust) fn with_partial_keys(
        schema: &Schema,
        entries: &BTreeMap<String, Override>,
        indexes: &[PartialUniqueIndex],
    ) -> io::Result<Self> {
        let mut names = Self::with_codecs(schema, entries)?;
        names.functions = functions::allocate_with_partials(schema, indexes);
        Ok(names)
    }

    /// Find an exact declared SQL type, preserving geometry modifiers and SRID.
    /// # Examples
    /// ```sh
    /// rustyroad pull --language rust --out crates/db/src
    /// ```
    pub(in crate::generators::rust) fn codec(&self, declared: &str) -> Option<&Override> {
        self.codecs.get(&overrides::normalize(declared))
    }

    /// Substitute an explicit OpenAPI type inside the actual generated wrappers.
    /// # Examples
    /// ```sh
    /// rustyroad pull --language rust --out crates/db/src
    /// ```
    pub(in crate::generators::rust) fn codec_schema(
        &self,
        declared: &str,
        field_type: &str,
    ) -> Option<String> {
        let element = crate::generators::rust::types::array_element(declared);
        let codec = self
            .codec(declared)
            .or_else(|| element.as_ref().and_then(|name| self.codec(name)))?;
        codec
            .schema
            .as_ref()
            .map(|schema| field_type.replace(&codec.rust, schema))
    }
}
