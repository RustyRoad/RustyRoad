//! Turning a plural table name into a singular type name.

/// Irregular plural endings, checked before the general `s` rule.
const IRREGULAR: [(&str, &str); 6] = [
    ("ies", "y"),
    ("sses", "ss"),
    ("ches", "ch"),
    ("shes", "sh"),
    ("xes", "x"),
    ("zes", "z"),
];

/// Endings that look plural but are not, so the trailing `s` must survive.
///
/// Without these, `status` becomes `statu` and `analysis` becomes `analysi`.
const NOT_PLURAL: [&str; 3] = ["ss", "us", "is"];

/// Converts a plural table name to its singular form.
///
/// Tables are conventionally plural and Rust types are singular, so `products` becomes
/// `Product`. Only the regular English endings are handled, which is the same trade-off
/// Rails makes without a full inflector: a name that does not match passes through
/// unchanged, giving a slightly odd type name rather than a wrong one, and never
/// affects the SQL, which always uses the real table name.
pub fn singularize(name: &str) -> String {
    for (suffix, replacement) in IRREGULAR {
        if let Some(stem) = name.strip_suffix(suffix) {
            if !stem.is_empty() {
                return format!("{stem}{replacement}");
            }
        }
    }

    if NOT_PLURAL.iter().any(|ending| name.ends_with(ending)) || name == "s" {
        return name.to_string();
    }

    match name.strip_suffix('s') {
        Some(stem) if !stem.is_empty() => stem.to_string(),
        _ => name.to_string(),
    }
}
