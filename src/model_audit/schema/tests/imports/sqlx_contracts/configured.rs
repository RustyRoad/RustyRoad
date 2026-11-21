//! Extracted fixtures retain their original assertions and inputs.

#[test]
fn configured_mapping_overrides_source_contract() {
    let source = r#"#[derive(sqlx::Type)] #[sqlx(type_name = "state")]
        enum State { A } struct Row { id: i32, state: State }"#;
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("models.rs");
    std::fs::write(&path, source).unwrap();
    let model = crate::model_audit::parse::parse(&path, directory.path(), source)
        .unwrap()
        .pop()
        .unwrap();
    let resolver = crate::model_audit::schema::rust_types::Resolver::for_model(&model);
    let mappings = std::collections::BTreeMap::from([("State".into(), "text".into())]);
    let column = crate::model_audit::schema::rust_types::model_column(
        "state",
        &model.fields["state"],
        &mappings,
        &resolver,
    )
    .unwrap();
    assert_eq!(column.sql_type, "text");
}
