//! Parse declarations rather than trusting substring counts for uniqueness.

use std::collections::BTreeSet;

pub(super) fn assert_unique(source: &str) {
    let file = syn::parse_file(source).expect("generated output must parse as Rust");
    let mut declarations = BTreeSet::new();
    for item in file.items {
        let ident = match item {
            syn::Item::Struct(item) => {
                let mut fields = BTreeSet::new();
                for field in item.fields {
                    if let Some(name) = field.ident {
                        assert!(
                            fields.insert(name.to_string()),
                            "duplicate field {name} in {}",
                            item.ident
                        );
                    }
                }
                item.ident
            }
            syn::Item::Enum(item) => {
                let mut variants = BTreeSet::new();
                for variant in item.variants {
                    assert!(
                        variants.insert(variant.ident.to_string()),
                        "duplicate enum variant"
                    );
                }
                item.ident
            }
            syn::Item::Fn(item) => item.sig.ident,
            _ => continue,
        };
        assert!(
            declarations.insert(ident.to_string()),
            "duplicate declaration {ident}"
        );
    }
}

/// Select only the named function's AST body, regardless of signature generics.
pub(super) fn function_source(source: &str, name: &str) -> String {
    syn::parse_file(source)
        .expect("generated output must parse as Rust")
        .items
        .into_iter()
        .find_map(|item| match item {
            syn::Item::Fn(function) if function.sig.ident == name => {
                let block = function.block;
                Some(quote::quote!(#block).to_string())
            }
            _ => None,
        })
        .expect("function exists")
}

#[cfg(test)]
mod tests {
    use super::function_source;

    #[test]
    fn finds_generic_functions_without_adjacent_or_documented_decoys() {
        let source = r#"
            pub async fn lookup_extra() { "prefix decoy"; }
            /// Example: pub async fn lookup() { "documentation decoy"; }
            pub async fn lookup<'e, T>(value: &'e T) where T: Sync {
                "selected body";
            }
            pub async fn following() { "adjacent decoy"; }
        "#;
        let body = function_source(source, "lookup");
        assert!(body.contains("selected body"));
        assert!(!body.contains("decoy"));
    }

    #[test]
    fn finds_nongeneric_body_without_following_declarations() {
        let body = function_source("pub async fn plain() { 1; } fn other() { 2; }", "plain");
        assert!(body.contains('1'));
        assert!(!body.contains("other"));
    }

    #[test]
    #[should_panic(expected = "function exists")]
    fn missing_function_fails_clearly() {
        function_source("pub async fn different() {}", "missing");
    }
}
