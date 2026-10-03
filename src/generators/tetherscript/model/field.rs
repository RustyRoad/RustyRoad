//! One column, resolved into the field that represents it.

use crate::generators::tetherscript::types::Kind;

/// A column resolved into the field that represents it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field {
    /// The column name, used both as the SQL identifier and the map key.
    ///
    /// Unlike the Rust generator there is no renaming: a map key is a string, so the
    /// column name is always usable as written and a second spelling would only give
    /// the row two names for one value.
    pub column: String,
    pub kind: Kind,
    pub primary_key: bool,
    /// True when the database supplies the value, via a sequence or identity.
    pub generated: bool,
    pub nullable: bool,
}

impl Field {
    /// Returns `true` when a caller supplies this field on insert.
    pub fn insertable(&self) -> bool {
        !self.generated && !self.timestamp()
    }

    /// Returns `true` when an update writes this field.
    pub fn updatable(&self) -> bool {
        !self.primary_key && !self.generated && !self.timestamp()
    }

    /// Returns `true` when the field is required, so validation must reject an empty.
    pub fn required(&self) -> bool {
        !self.nullable && self.insertable()
    }

    /// Returns `true` when the column is an audit timestamp the database maintains.
    fn timestamp(&self) -> bool {
        matches!(self.column.as_str(), "created_at" | "updated_at")
    }
}
