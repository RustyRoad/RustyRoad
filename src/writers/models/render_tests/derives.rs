use super::{generated, tokens};
use syn::{punctuated::Punctuated, Path, Token};

fn derives(source: &str) -> Vec<String> {
    let (model, _) = generated("User", source);
    model
        .attrs
        .iter()
        .filter(|attr| attr.path().is_ident("derive"))
        .flat_map(|attr| {
            attr.parse_args_with(Punctuated::<Path, Token![,]>::parse_terminated)
                .unwrap()
        })
        .map(|path| tokens(&path))
        .collect()
}

#[test]
fn preserves_sqlx_derives_without_adding_diesel_attributes() {
    assert_eq!(
        derives("#[derive(Debug, sqlx::FromRow)] struct users {}"),
        [
            "Debug",
            "sqlx :: FromRow",
            "serde :: Serialize",
            "serde :: Deserialize",
            "Clone"
        ]
    );
    let (model, _) = generated("User", "struct users {}");
    assert!(model
        .attrs
        .iter()
        .all(|attr| attr.path().is_ident("derive")));
}

#[test]
fn does_not_duplicate_existing_required_derives() {
    let existing = "#[derive(Debug, Clone, serde::Serialize, Deserialize)] struct users {}";
    assert_eq!(
        derives(existing),
        ["Debug", "Clone", "serde :: Serialize", "Deserialize"]
    );
}
