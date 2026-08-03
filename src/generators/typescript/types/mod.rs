//! Postgres type to Drizzle column builder mapping.

mod lookup;
mod options;
mod parse;

/// A resolved column builder: the import name and the call it generates.
pub struct Builder {
    /// Symbol to import from `drizzle-orm/pg-core`.
    pub import: &'static str,
    /// Extra options rendered inside the builder call, if any.
    pub options: Option<String>,
}

impl Builder {
    /// Builds a mapping with no extra options.
    fn plain(import: &'static str) -> Self {
        Self {
            import,
            options: None,
        }
    }

    /// Builds a mapping carrying builder options.
    fn with_options(import: &'static str, options: String) -> Self {
        Self {
            import,
            options: Some(options),
        }
    }
}

/// Maps a Postgres type to its Drizzle builder.
///
/// `sql_type` comes from `format_type`, so modifiers are present and are carried
/// into the builder call: `character varying(255)` becomes `varchar({ length: 255 })`.
pub fn map(sql_type: &str, auto_increment: bool) -> Builder {
    let lowered = sql_type.to_lowercase();
    let base = parse::strip_modifier(&lowered);

    if auto_increment {
        return lookup::serial(&base);
    }
    if let Some(builder) = lookup::scalar(&base) {
        return builder;
    }

    lookup::parameterized(&base, &lowered)
}
