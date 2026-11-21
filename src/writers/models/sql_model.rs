//! Offline SQL-to-model conversion for the legacy model writer.
use crate::database::DatabaseType;
use std::io;

mod fields;
mod names;
mod nullability;
mod parse;
mod primary;
#[cfg(test)]
mod tests;
mod types;

pub(super) fn render(sql: &str, database: &DatabaseType) -> io::Result<String> {
    let table = parse::table(sql, database)?;
    let fields = fields::render(&table, database)?;
    let name = table
        .name
        .0
        .last()
        .and_then(|part| part.as_ident())
        .ok_or_else(|| invalid("expected a table identifier"))?;
    let name = names::model(&name.value)?;
    let source = format!("#[derive(Debug, sqlx::FromRow)]\npub struct {name} {{\n{fields}}}\n");
    syn::parse_file(&source).map_err(|error| invalid(error.to_string()))?;
    Ok(source)
}

fn invalid(message: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message.into())
}
