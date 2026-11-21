use super::{generated, render_base_model, tokens};
use std::io::ErrorKind;

#[test]
fn preserves_nested_conditional_guards_without_impl_incompatible_attributes() {
    let (model, implementation) = generated(
        "Record",
        "#[cfg_attr(feature = \"models\", derive(Debug),
            cfg_attr(feature = \"extra\", cfg(unix), serde(default)))]
        #[cfg_attr(feature = \"extra\", derive(PartialEq))]
        struct records { id: i64 }",
    );
    assert_eq!(implementation.attrs.len(), 1);
    assert_eq!(
        tokens(&implementation.attrs[0]),
        "# [cfg_attr (feature = \"models\" , cfg_attr (feature = \"extra\" , cfg (unix)))]"
    );
    let model = tokens(&model);
    assert!(model.contains("derive (Debug)"));
    assert!(model.contains("serde (default)"));
    assert!(model.contains("derive (PartialEq)"));
}

#[test]
fn rejects_malformed_conditional_guards() {
    for attribute in [
        "#[cfg_attr()]",
        "#[cfg_attr(unix)]",
        "#[cfg_attr = \"unix\"]",
        "#[cfg_attr(unix, cfg_attr(windows))]",
    ] {
        for source in [
            format!("{attribute} struct records {{}}"),
            format!("struct records {{ {attribute} id: i64 }}"),
        ] {
            assert_eq!(
                render_base_model("Record", &source).unwrap_err().kind(),
                ErrorKind::InvalidData,
                "source: {source}"
            );
        }
    }
}
