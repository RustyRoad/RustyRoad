use super::{invalid, names, nullability, primary, types};
use crate::database::DatabaseType;
use sqlparser::ast::CreateTable;
use std::{collections::HashSet, io};

pub(super) fn render(table: &CreateTable, database: &DatabaseType) -> io::Result<String> {
    let keys = primary::columns(table, database)?;
    let mut seen = HashSet::new();
    let mut fields = String::new();
    for column in &table.columns {
        let name = names::column(&column.name, database);
        let field = names::field(&name)?;
        if !seen.insert(field.clone()) {
            return Err(invalid(format!("duplicate Rust field for column {name:?}")));
        }
        let mut rust_type = types::map(&column.data_type, database)?;
        if !nullability::required(column, table, &keys, database) {
            rust_type = format!("Option<{rust_type}>");
        }
        if field.trim_start_matches("r#") != name {
            fields.push_str(&format!("    #[sqlx(rename = {name:?})]\n"));
            fields.push_str(&format!("    #[serde(rename = {name:?})]\n"));
        }
        fields.push_str(&format!("    pub {field}: {rust_type},\n"));
    }
    Ok(fields)
}
