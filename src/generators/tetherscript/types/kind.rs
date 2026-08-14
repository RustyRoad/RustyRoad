//! The kind a column's value takes at runtime.

/// The kind a column's value takes at runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Int,
    Float,
    Bool,
    Str,
    /// A JSON document, which arrives as a map or list.
    Json,
    /// A Postgres array, which arrives as a list.
    List,
}

impl Kind {
    /// Returns the name the generated model checks against.
    ///
    /// These match the names `type_of` reports, so a generated check reads the same way a
    /// hand-written one would — and a mismatch here would reject every valid row.
    pub fn name(self) -> &'static str {
        match self {
            Self::Int => "int",
            Self::Float => "float",
            Self::Bool => "bool",
            Self::Str => "str",
            Self::Json => "map",
            Self::List => "list",
        }
    }

    /// Returns the literal a defaulted field is seeded with.
    pub fn zero(self) -> &'static str {
        match self {
            Self::Int => "0",
            Self::Float => "0.0",
            Self::Bool => "false",
            Self::Str => "\"\"",
            Self::Json => "map()",
            Self::List => "[]",
        }
    }
}
