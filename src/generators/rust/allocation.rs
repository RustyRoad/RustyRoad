//! Schema-wide Rust names. SQL identifiers never enter this allocated namespace.

mod clocks;
mod codecs;
mod configured;
mod fields;
mod filters;
mod functions;
mod lookup;
mod namespace;
mod reserved;
mod scoped;
mod types;

use crate::database::introspection::Schema;
use std::collections::BTreeMap;

/// Shared by renderers to preserve allocated names and explicit codec mappings.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) struct Names {
    codecs: BTreeMap<String, super::types::Override>,
    column_codecs: BTreeMap<(String, String), super::types::Override>,
    types: BTreeMap<(u8, String), String>,
    fields: BTreeMap<String, BTreeMap<String, String>>,
    functions: BTreeMap<functions::Key, String>,
}

/// Allocate all generated names consistently from the same schema.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
impl Names {
    /// Create the default naming plan without project codec overrides.
    /// # Examples
    /// ```sh
    /// rustyroad pull --language rust --out crates/db/src
    /// ```
    pub(super) fn new(schema: &Schema) -> Self {
        Self {
            codecs: BTreeMap::new(),
            column_codecs: BTreeMap::new(),
            types: types::allocate(schema),
            fields: fields::allocate(schema),
            functions: functions::allocate(schema),
        }
    }
}

/// Read-only access to the allocated repository namespace for configured reports.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
impl Names {
    /// Rejects collisions with generated repository operations and configuration hooks.
    /// # Examples
    /// ```sh
    /// rustyroad pull --language rust --out crates/db/src
    /// ```
    pub(super) fn function_reserved(&self, name: &str) -> bool {
        name == "configure_generated"
            || name == super::repositories::clock::FUNCTION
            || name == super::repositories::read_snapshot::FUNCTION
            || self.functions.values().any(|value| value == name)
    }
}
