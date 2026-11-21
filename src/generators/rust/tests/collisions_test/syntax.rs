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

pub(super) fn function_source<'a>(source: &'a str, name: &str) -> &'a str {
    source
        .split(&format!("pub async fn {name}("))
        .nth(1)
        .expect("function exists")
        .split("\n\n")
        .next()
        .unwrap()
}
