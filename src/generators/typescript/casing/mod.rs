//! Identifier casing and escaping, following Drizzle Kit's rules.

mod convert;
mod rules;

pub use convert::to_camel;
pub use rules::{binding, db_name_argument, escape, identifier, to_pascal};

/// How database names are turned into TypeScript identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Casing {
    /// Keep the database name as written.
    Preserve,
    /// Convert `snake_case` to `camelCase`.
    Camel,
}

impl Casing {
    /// Parses a CLI value, defaulting to camel as Drizzle does.
    pub fn parse(value: Option<&str>) -> Self {
        match value {
            Some("preserve") => Self::Preserve,
            _ => Self::Camel,
        }
    }

    /// Applies the casing rule without escaping.
    fn apply(self, name: &str) -> String {
        match self {
            Self::Preserve => name.to_string(),
            Self::Camel => to_camel(name),
        }
    }
}
