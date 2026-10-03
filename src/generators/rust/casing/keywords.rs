//! Keyword avoidance for generated field names.

/// Rust keywords that cannot be written as raw identifiers.
///
/// Every other keyword is usable as `r#keyword`, so only these force a renamed field.
/// `self`/`Self` are excluded because a field named for them is meaningless in a struct
/// definition, and `crate`/`super` cannot be raw at all.
const UNRAWABLE: [&str; 4] = ["crate", "self", "Self", "super"];

/// Rust keywords, reserved words included.
///
/// Reserved-but-unused words are listed because they are still rejected as bare
/// identifiers, and a column named `become` or `yield` is perfectly legal SQL.
const KEYWORDS: [&str; 51] = [
    "abstract", "as", "async", "await", "become", "box", "break", "const", "continue", "crate",
    "do", "dyn", "else", "enum", "extern", "false", "final", "fn", "for", "if", "impl", "in",
    "let", "loop", "macro", "match", "mod", "move", "mut", "override", "priv", "pub", "ref",
    "return", "self", "Self", "static", "struct", "super", "trait", "true", "try", "type",
    "typeof", "unsafe", "unsized", "use", "virtual", "where", "while", "yield",
];

/// Returns `name` in a form the compiler accepts as a field name.
pub(super) fn escape(name: &str) -> String {
    if UNRAWABLE.contains(&name) {
        // Cannot be raw, so the name itself has to change; the caller recovers the
        // column name through a rename attribute.
        return format!("{name}_");
    }
    if KEYWORDS.contains(&name) {
        return format!("r#{name}");
    }
    name.to_string()
}
