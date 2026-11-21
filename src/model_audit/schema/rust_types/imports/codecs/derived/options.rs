//! Parse named-codec options without accepting representation or transparent codecs.
use super::super::sql_name;

/// Require exactly one explicit SQL type name and known variant renaming rules.
pub(super) fn name(attrs: &[syn::Attribute]) -> Option<String> {
    let mut names = Vec::new();
    let mut renamed = false;
    for attr in attrs {
        if attr.path().is_ident("sqlx") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("rename_all") {
                    if renamed {
                        return Err(meta.error("Duplicate SQLx variant renaming rule"));
                    }
                    renamed = true;
                    let value = meta.value()?.parse::<syn::LitStr>()?;
                    return match value.value().as_str() {
                        "lowercase"
                        | "snake_case"
                        | "UPPERCASE"
                        | "SCREAMING_SNAKE_CASE"
                        | "kebab-case"
                        | "camelCase"
                        | "PascalCase" => Ok(()),
                        _ => Err(meta.error("Unknown SQLx variant renaming rule")),
                    };
                }
                if !meta.path.is_ident("type_name") {
                    return Err(meta.error("SQLx codec option requires review"));
                }
                names.push(meta.value()?.parse::<syn::LitStr>()?.value());
                Ok(())
            })
            .ok()?;
        }
    }
    if names.len() != 1 {
        return None;
    }
    sql_name(&names[0])
}
