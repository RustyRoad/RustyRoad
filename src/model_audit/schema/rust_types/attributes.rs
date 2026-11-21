//! Separate descriptive metadata from attributes that need a storage decision.
use syn::parse::Parser;

/// Resolve one literal SQLx column rename independently of wire metadata.
/// Custom codecs, duplicate renames and mixed SQLx options remain blocked.
/// # Examples
/// ```sh
/// rustyroad model audit --source src/models --report-only
/// ```
pub(super) fn column_name(name: &str, attributes: &[String]) -> Result<String, String> {
    let error = || format!("{name}: field attributes require an explicit column/codec decision");
    let mut column = name.trim_start_matches("r#").to_owned();
    let mut renamed = false;
    for source in attributes {
        let attrs = syn::Attribute::parse_outer
            .parse_str(source)
            .map_err(|_| error())?;
        if attrs.is_empty() {
            return Err(error());
        }
        for attribute in attrs {
            if attribute.path().is_ident("sqlx") {
                if renamed {
                    return Err(error());
                }
                column = literal_rename(&attribute).ok_or_else(error)?;
                renamed = true;
            } else if !["doc", "schema", "schemars", "serde"]
                .iter()
                .any(|name| attribute.path().is_ident(name))
            {
                return Err(error());
            }
        }
    }
    Ok(column)
}

/// Extract one literal storage name, with no trailing nested options.
/// # Examples
/// ```sh
/// rustyroad model audit --source src/models --report-only
/// ```
fn literal_rename(attribute: &syn::Attribute) -> Option<String> {
    let option = attribute.parse_args::<syn::MetaNameValue>().ok()?;
    if !option.path.is_ident("rename") {
        return None;
    }
    let syn::Expr::Lit(value) = option.value else {
        return None;
    };
    let syn::Lit::Str(value) = value.lit else {
        return None;
    };
    Some(value.value())
}
