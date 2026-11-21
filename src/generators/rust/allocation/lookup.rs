//! Read-only model and field access to the shared allocation plan.

use super::{namespace, Names};
use crate::database::introspection::Table;
use crate::generators::rust::naming::pascal;
use std::collections::BTreeMap;

impl Names {
    pub(in crate::generators::rust) fn row(&self, table: &Table) -> &str {
        &self.types[&(0, table.name.clone())]
    }

    pub(in crate::generators::rust) fn create(&self, table: &Table) -> &str {
        &self.types[&(2, table.name.clone())]
    }

    pub(in crate::generators::rust) fn patch(&self, table: &Table) -> &str {
        &self.types[&(3, table.name.clone())]
    }

    pub(in crate::generators::rust) fn enum_type(&self, name: &str) -> &str {
        &self.types[&(1, name.to_string())]
    }

    pub(in crate::generators::rust) fn field(&self, table: &Table, column: &str) -> &str {
        &self.fields[&table.name][column]
    }

    /// Allocate variants independently, retaining their original SQL/serde labels.
    pub(in crate::generators::rust) fn variants(values: &[String]) -> BTreeMap<String, String> {
        namespace::allocate(
            values.iter().map(|v| (v.clone(), pascal(v))).collect(),
            &["Self"],
            "Variant",
        )
    }
}
