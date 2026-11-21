use super::{generated, tokens};
use syn::{Fields, ImplItem, Visibility};

#[test]
fn renders_one_struct_and_complete_typed_constructor() {
    let (model, implementation) = generated(
        "User",
        "#[derive(Debug, sqlx::FromRow)] struct users { id: i64, name: Option<String> }",
    );
    assert_eq!(model.ident, "User");
    assert!(matches!(model.vis, Visibility::Public(_)));
    let Fields::Named(fields) = &model.fields else {
        panic!("expected named fields");
    };
    assert!(fields
        .named
        .iter()
        .all(|field| matches!(field.vis, Visibility::Public(_))));
    assert_eq!(tokens(&fields.named[0].ty), "i64");
    assert_eq!(tokens(&fields.named[1].ty), "Option < String >");
    assert_eq!(tokens(&implementation.self_ty), "User");
    let ImplItem::Fn(constructor) = &implementation.items[0] else {
        panic!("expected constructor");
    };
    assert!(matches!(constructor.vis, Visibility::Public(_)));
    assert_eq!(constructor.sig.ident, "new");
    assert_eq!(
        tokens(&constructor.sig.inputs),
        "id : i64 , name : Option < String >"
    );
    assert_eq!(tokens(&constructor.sig.output), "-> Self");
    assert_eq!(tokens(&constructor.block), "{ Self { id , name } }");
}
