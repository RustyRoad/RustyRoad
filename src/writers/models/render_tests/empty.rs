use super::{generated, tokens};
use syn::ImplItem;

#[test]
fn renders_empty_named_model() {
    let (model, implementation) = generated("Empty", "struct records {}");
    assert!(model.fields.is_empty());
    let ImplItem::Fn(constructor) = &implementation.items[0] else {
        panic!("expected constructor");
    };
    assert!(constructor.sig.inputs.is_empty());
    assert_eq!(tokens(&constructor.sig.output), "-> Self");
    assert_eq!(tokens(&constructor.block), "{ Self { } }");
}
