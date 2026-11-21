use super::render;
use crate::database::DatabaseType;
use quote::ToTokens;
use syn::{Fields, Item};

mod failures;
mod implicit;
mod key_names;
mod mysql;
mod names;
mod postgres;
mod postgres_names;
mod serial;
mod sqlite;
mod sqlite_keys;

fn model(sql: &str, database: DatabaseType) -> syn::ItemStruct {
    let source = render(sql, &database).unwrap();
    // Exercise the actual writer's rendering boundary, not only the SQL parser.
    let source = crate::writers::models::render::render_base_model("Example", &source).unwrap();
    let file = syn::parse_file(&source).unwrap();
    assert_eq!(file.items.len(), 2, "expected a model and its constructor");
    let Item::Struct(model) = file.items.into_iter().next().unwrap() else {
        panic!("expected a struct");
    };
    model
}

fn fields(sql: &str, database: DatabaseType) -> Vec<(String, String)> {
    let Fields::Named(fields) = model(sql, database).fields else {
        panic!("expected named fields");
    };
    fields
        .named
        .into_iter()
        .map(|field| {
            (
                field.ident.unwrap().to_string(),
                field.ty.to_token_stream().to_string().replace(' ', ""),
            )
        })
        .collect()
}

fn assert_invalid(sql: &str, database: DatabaseType) {
    assert_eq!(
        render(sql, &database).unwrap_err().kind(),
        std::io::ErrorKind::InvalidData
    );
}
