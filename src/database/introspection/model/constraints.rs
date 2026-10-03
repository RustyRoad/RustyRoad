//! Constraint parts of the introspected model.

/// A foreign key from one or more local columns to a referenced table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForeignKey {
    pub name: String,
    pub columns: Vec<String>,
    pub foreign_table: String,
    pub foreign_columns: Vec<String>,
    pub on_delete: Option<String>,
    pub on_update: Option<String>,
}

/// A unique constraint.
/// Introspection retains immediate constraints, e.g. a nondeferrable email key.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unique {
    pub name: String,
    pub columns: Vec<String>,
}

/// A non-constraint index.
/// Only valid, ready, immediate, unconditional plain-column indexes are represented.
/// For example, an index on `email INCLUDE (name)` has `columns == ["email"]`.
/// Partial/expression indexes are omitted by introspection, not treated as plain keys.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Index {
    pub name: String,
    pub columns: Vec<String>,
    pub unique: bool,
}