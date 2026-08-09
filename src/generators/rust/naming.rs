//! Safe Rust and SQL identifiers.

/// Converts a database identifier to snake_case suitable for a Rust field.
pub(super) fn snake(name: &str) -> String {
    let mut output = String::new();
    let mut separator = false;

    for character in name.chars() {
        if character.is_ascii_alphanumeric() {
            if character.is_ascii_uppercase() && !output.is_empty() && !separator {
                output.push('_');
            }
            if separator && !output.is_empty() {
                output.push('_');
            }
            output.push(character.to_ascii_lowercase());
            separator = false;
        } else {
            separator = true;
        }
    }

    if output.is_empty() {
        output.push_str("field");
    }
    if output.starts_with(|character: char| character.is_ascii_digit()) {
        output.insert_str(0, "field_");
    }
    if keyword(&output) {
        output.push('_');
    }
    output
}

/// Converts a database identifier to PascalCase suitable for a Rust type.
pub(super) fn pascal(name: &str) -> String {
    let snake = snake(name);
    let mut output = String::new();
    for part in snake
        .trim_end_matches('_')
        .split('_')
        .filter(|part| !part.is_empty())
    {
        let mut characters = part.chars();
        if let Some(first) = characters.next() {
            output.extend(first.to_uppercase());
            output.push_str(characters.as_str());
        }
    }
    if output.is_empty() {
        output.push_str("Record");
    }
    output
}

/// Quotes a Postgres identifier, including embedded quote characters.
pub(super) fn sql(name: &str) -> String {
    format!("\"{}\"", name.replace('"', "\"\""))
}

/// Produces an escaped Rust string literal.
pub(super) fn literal(value: &str) -> String {
    format!("{value:?}")
}

fn keyword(value: &str) -> bool {
    matches!(
        value,
        "as" | "async"
            | "await"
            | "break"
            | "const"
            | "continue"
            | "crate"
            | "dyn"
            | "else"
            | "enum"
            | "extern"
            | "false"
            | "fn"
            | "for"
            | "if"
            | "impl"
            | "in"
            | "let"
            | "loop"
            | "match"
            | "mod"
            | "move"
            | "mut"
            | "pub"
            | "ref"
            | "return"
            | "self"
            | "Self"
            | "static"
            | "struct"
            | "super"
            | "trait"
            | "true"
            | "type"
            | "unsafe"
            | "use"
            | "where"
            | "while"
            | "abstract"
            | "become"
            | "box"
            | "do"
            | "final"
            | "macro"
            | "override"
            | "priv"
            | "typeof"
            | "unsized"
            | "virtual"
            | "yield"
            | "try"
    )
}
