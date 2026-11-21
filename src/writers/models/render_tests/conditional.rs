use super::{generated, tokens};
use syn::{Expr, FnArg, ImplItem, Stmt};

#[test]
fn copies_only_existence_guards_to_conditional_fields() {
    let (model, implementation) = generated(
        "Record",
        "struct records {
            #[cfg(feature = \"extra\")] #[serde(default)] extra: i64,
            #[cfg_attr(feature = \"models\", cfg(feature = \"extra\"), serde(default))]
            conditional: bool,
            #[serde(rename = \"base\")] base: String,
        }",
    );
    let ImplItem::Fn(constructor) = &implementation.items[0] else {
        panic!("expected constructor");
    };
    let Stmt::Expr(Expr::Struct(value), _) = &constructor.block.stmts[0] else {
        panic!("expected struct initializer");
    };
    let expected = [
        "# [cfg (feature = \"extra\")]",
        "# [cfg_attr (feature = \"models\" , cfg (feature = \"extra\"))]",
        "",
    ];
    for ((parameter, initializer), expected) in constructor
        .sig
        .inputs
        .iter()
        .zip(&value.fields)
        .zip(expected)
    {
        let FnArg::Typed(parameter) = parameter else {
            panic!("expected typed parameter");
        };
        let attributes = &parameter.attrs;
        assert_eq!(quote::quote!(#(#attributes)*).to_string(), expected);
        let attributes = &initializer.attrs;
        assert_eq!(quote::quote!(#(#attributes)*).to_string(), expected);
    }
    assert_eq!(constructor.sig.inputs.len(), 3);
    assert_eq!(value.fields.len(), 3);
    assert!(tokens(&model).contains("serde"));
}
