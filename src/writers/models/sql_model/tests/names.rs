use super::*;

#[test]
fn quoted_names_are_escaped_and_mapped_to_sql_names() {
    let model = model(
        r#"CREATE TABLE "odd table" ("type" TEXT, "self" TEXT, "display-name" TEXT, "quote""name" TEXT)"#,
        DatabaseType::Postgres,
    );
    let Fields::Named(fields) = model.fields else {
        panic!("expected named fields")
    };
    let actual = fields
        .named
        .iter()
        .map(|field| field.ident.as_ref().unwrap().to_string())
        .collect::<Vec<_>>();
    assert_eq!(actual, ["r#type", "self_", "display_name", "quotename"]);
    for (index, expected) in [(1, "self"), (2, "display-name"), (3, "quote\"name")] {
        let mut renames = Vec::new();
        for attribute in &fields.named[index].attrs {
            if attribute.path().is_ident("sqlx") || attribute.path().is_ident("serde") {
                attribute
                    .parse_nested_meta(|meta| {
                        assert!(meta.path.is_ident("rename"));
                        renames.push(meta.value()?.parse::<syn::LitStr>()?.value());
                        Ok(())
                    })
                    .unwrap();
            }
        }
        assert_eq!(renames, [expected, expected]);
    }
}

#[test]
fn numeric_names_use_the_shared_identifier_prefix() {
    let actual = fields(r#"CREATE TABLE "123" ("123" TEXT)"#, DatabaseType::Postgres);
    assert_eq!(actual, [("f_123".into(), "Option<String>".into())]);
}
