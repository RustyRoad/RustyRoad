use super::render_base_model;
use quote::ToTokens;
use syn::{Item, ItemImpl, ItemStruct};

fn tokens(value: &impl ToTokens) -> String {
    value.to_token_stream().to_string()
}

#[path = "render_tests/attributes.rs"]
mod attributes;
#[path = "render_tests/cfg_attr.rs"]
mod cfg_attr;
#[path = "render_tests/conditional.rs"]
mod conditional;
#[path = "render_tests/constructor.rs"]
mod constructor;
#[path = "render_tests/derives.rs"]
mod derives;
#[path = "render_tests/empty.rs"]
mod empty;
#[path = "render_tests/invalid.rs"]
mod invalid;

fn generated(name: &str, source: &str) -> (ItemStruct, ItemImpl) {
    let output = render_base_model(name, source).unwrap();
    let mut items = syn::parse_file(&output).unwrap().items.into_iter();
    let Item::Struct(model) = items.next().unwrap() else {
        panic!("expected one model struct");
    };
    let Item::Impl(implementation) = items.next().unwrap() else {
        panic!("expected a constructor impl");
    };
    assert!(items.next().is_none());
    (model, implementation)
}
