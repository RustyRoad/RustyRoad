//! Accept only transparent serde representation and non-storage wrapper metadata.
use syn::parse::Parser;

/// Rejects conditional, custom-storage, or mixed serde wrapper contracts.
/// # Examples
/// ```sh
/// rustyroad model audit --source src/models --report-only
/// ```
pub(super) fn transparent(sources: &[String]) -> bool {
    let mut found = false;
    for source in sources {
        let Ok(attributes) = syn::Attribute::parse_outer.parse_str(source) else {
            return false;
        };
        for attribute in attributes {
            if attribute.path().is_ident("serde") {
                if found {
                    return false;
                }
                let mut options = 0;
                if attribute
                    .parse_nested_meta(|meta| {
                        if !meta.path.is_ident("transparent") || !meta.input.is_empty() {
                            return Err(meta.error("expected only transparent"));
                        }
                        options += 1;
                        Ok(())
                    })
                    .is_err()
                    || options != 1
                {
                    return false;
                }
                found = true;
            } else if attribute.path().is_ident("derive") {
                if attribute
                    .parse_nested_meta(|meta| {
                        let name = meta.path.segments.last().map(|part| part.ident.to_string());
                        if !matches!(
                            name.as_deref(),
                            Some(
                                "Debug"
                                    | "Clone"
                                    | "Copy"
                                    | "Default"
                                    | "PartialEq"
                                    | "Eq"
                                    | "Serialize"
                                    | "Deserialize"
                                    | "ToSchema"
                            )
                        ) {
                            return Err(meta.error("unsupported wrapper derive"));
                        }
                        Ok(())
                    })
                    .is_err()
                {
                    return false;
                }
            } else if !["doc", "allow", "deny", "warn", "forbid"]
                .iter()
                .any(|name| attribute.path().is_ident(name))
            {
                return false;
            }
        }
    }
    found
}
