use super::names;
use crate::database::DatabaseType;
use sqlparser::ast::{ColumnDef, ColumnOption, CreateTable};
use sqlparser::tokenizer::Token;
use std::collections::HashSet;

mod implicit;

pub(super) fn required(
    column: &ColumnDef,
    table: &CreateTable,
    keys: &HashSet<String>,
    database: &DatabaseType,
) -> bool {
    if implicit::required(column, database)
        || column
            .options
            .iter()
            .any(|option| matches!(option.option, ColumnOption::NotNull))
    {
        return true;
    }
    let sql_type = column.data_type.to_string().to_ascii_uppercase();
    if matches!(database, DatabaseType::Postgres)
        && matches!(
            sql_type.as_str(),
            "SERIAL" | "BIGSERIAL" | "SMALLSERIAL" | "SERIAL2" | "SERIAL4" | "SERIAL8"
        )
    {
        return true;
    }
    if !keys.contains(&names::key(&column.name, database)) {
        return false;
    }
    if !matches!(database, DatabaseType::Sqlite) || table.strict || table.without_rowid {
        return true;
    }
    // Ordinary SQLite primary keys may contain NULL, except INTEGER rowid aliases.
    keys.len() == 1
        && sql_type == "INTEGER"
        && !column.options.iter().any(|option| {
            matches!(
                &option.option,
                ColumnOption::DialectSpecific(tokens) if tokens == &[Token::make_keyword("DESC")]
            )
        })
}
