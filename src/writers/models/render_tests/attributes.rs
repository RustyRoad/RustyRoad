use super::{generated, tokens};
use syn::ImplItem;

#[test]
fn preserves_attributes_generics_and_raw_identifiers() {
    let (model, implementation) = generated(
        "Envelope",
        "#[cfg(feature = \"models\")] #[derive(Debug, sqlx::FromRow)]
         struct records<T> where T: Clone { #[serde(rename = \"kind\")] r#type: T }",
    );
    let attributes = &model.attrs;
    assert!(quote::quote!(#(#attributes)*).to_string().contains("cfg"));
    assert_eq!(implementation.attrs.len(), 1);
    assert_eq!(
        tokens(&implementation.attrs[0]),
        "# [cfg (feature = \"models\")]"
    );
    let field = model.fields.iter().next().unwrap();
    assert_eq!(field.ident.as_ref().unwrap(), "r#type");
    assert!(field
        .attrs
        .iter()
        .any(|attribute| attribute.path().is_ident("serde")));
    assert_eq!(tokens(&model.generics.params), "T");
    assert_eq!(tokens(&model.generics.where_clause), "where T : Clone");
    assert_eq!(tokens(&implementation.generics.params), "T");
    assert_eq!(
        tokens(&implementation.generics.where_clause),
        "where T : Clone"
    );
    assert_eq!(tokens(&implementation.self_ty), "Envelope < T >");
    let ImplItem::Fn(constructor) = &implementation.items[0] else {
        panic!("expected constructor");
    };
    assert_eq!(tokens(&constructor.sig.inputs), "r#type : T");
    assert_eq!(tokens(&constructor.block), "{ Self { r#type } }");
}
