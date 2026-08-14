//! Database names to Rust identifiers.
//!
//! Rust casing is not a matter of taste the way it is in the TypeScript output:
//! `snake_case` fields and `PascalCase` types are enforced by the compiler's own lint
//! set, so generated code that ignores them warns on every build. The database name is
//! therefore always converted, and any difference between the converted name and the
//! column name is carried by a `rename` attribute so the mapping back to SQL stays
//! exact.

mod convert;
mod keywords;
mod plural;

pub use convert::{to_pascal, to_snake};
pub use plural::singularize;

/// A field name usable in a struct, plus the rename it needs to reach its column.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field {
    /// How the field is written in Rust, including any `r#` prefix.
    pub ident: String,
    /// The column name, present only when it differs from the bare field name.
    ///
    /// `None` means the field name already matches the column, so no attribute is
    /// needed and the generated struct stays readable.
    pub rename: Option<String>,
}

/// Builds the field for a column.
pub fn field(column: &str) -> Field {
    let snake = to_snake(column);
    let ident = keywords::escape(&snake);

    // Compared against the bare name because sqlx and serde both strip the `r#`
    // prefix when they derive a column name from the identifier.
    let bare = ident.trim_start_matches("r#");
    let rename = if bare == column {
        None
    } else {
        Some(column.to_string())
    };

    Field { ident, rename }
}
