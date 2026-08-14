//! One column, resolved into the field that represents it.

use crate::generators::rust::types::Mapping;

/// A column resolved into the field that represents it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field {
    /// The column name as it exists in the database.
    pub column: String,
    /// The field name in Rust, including any `r#` prefix.
    pub ident: String,
    /// The `serde`/`sqlx` rename needed to reach the column, when it differs.
    pub rename: Option<String>,
    /// The resolved Rust type.
    pub mapping: Mapping,
    /// True when the column is part of the primary key.
    pub primary_key: bool,
    /// True when the database supplies the value, via a sequence or identity.
    pub generated: bool,
    pub nullable: bool,
}

impl Field {
    /// Returns the non-null type used when this field addresses a row.
    ///
    /// PostgreSQL reports projected view columns as nullable even when an `id` expression is
    /// never null. A lookup value itself cannot be absent, so keyed method and route parameters
    /// use the inner type while the row field preserves the catalog's `Option<_>` shape.
    pub fn key_type(&self) -> &str {
        self.mapping
            .rust
            .strip_prefix("Option<")
            .and_then(|rest| rest.strip_suffix('>'))
            .unwrap_or(&self.mapping.rust)
    }

    /// Returns `true` when a caller supplies this field on insert.
    ///
    /// A sequence-backed key and the audit timestamps are set by the database, so
    /// binding them from a struct would either fail or overwrite the server clock.
    pub fn insertable(&self) -> bool {
        !self.generated && !self.timestamp()
    }

    /// Returns `true` when this field is updated by an `UPDATE`.
    ///
    /// The key identifies the row rather than being written, and the audit timestamps
    /// are handled by the statement: `updated_at` is set to the server clock, and
    /// binding it as well would assign the column twice in one `SET`, which Postgres
    /// rejects.
    pub fn updatable(&self) -> bool {
        !self.primary_key && !self.generated && !self.timestamp()
    }

    /// Returns `true` when the column is an audit timestamp the database maintains.
    fn timestamp(&self) -> bool {
        matches!(self.column.as_str(), "created_at" | "updated_at")
    }
}
