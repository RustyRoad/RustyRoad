use super::invalid;
use crate::{database::DatabaseType, generators::rust::casing};
use sqlparser::ast::Ident;
use std::io;

pub(super) fn column(name: &Ident, database: &DatabaseType) -> String {
    if matches!(database, DatabaseType::Postgres) && name.quote_style.is_none() {
        name.value.to_lowercase()
    } else {
        name.value.clone()
    }
}

pub(super) fn key(name: &Ident, database: &DatabaseType) -> String {
    match database {
        DatabaseType::Mysql | DatabaseType::Sqlite => name.value.to_ascii_lowercase(),
        _ => column(name, database),
    }
}

pub(super) fn field(name: &str) -> io::Result<String> {
    if name.is_empty() {
        return Err(invalid("column names must not be empty"));
    }
    let field = casing::field(name).ident;
    syn::parse_str::<syn::Ident>(&field)
        .map_err(|_| invalid(format!("column {name:?} has no usable Rust field name")))?;
    Ok(field)
}

pub(super) fn model(name: &str) -> io::Result<String> {
    let model = casing::to_pascal(name);
    syn::parse_str::<syn::Ident>(&model)
        .map_err(|_| invalid(format!("table {name:?} has no usable Rust model name")))?;
    Ok(model)
}
