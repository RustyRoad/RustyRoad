mod mysql;
mod postgres;
mod sqlite;

use super::invalid;
use crate::database::DatabaseType;
use sqlparser::ast::{ArrayElemTypeDef, DataType};
use std::io;

pub(super) fn map(data_type: &DataType, database: &DatabaseType) -> io::Result<String> {
    if let DataType::Array(element) = data_type {
        if !matches!(database, DatabaseType::Postgres) {
            return Err(invalid("SQL arrays require PostgreSQL"));
        }
        let inner = match element {
            ArrayElemTypeDef::SquareBracket(inner, _) => inner,
            _ => return Err(invalid("unsupported SQL array syntax")),
        };
        return Ok(format!("Vec<{}>", map(inner, database)?));
    }
    let spelling = data_type.to_string().to_ascii_lowercase();
    let modifiers = regex::Regex::new(r"\([^)]*\)").expect("static type modifier regex");
    let normalized = modifiers.replace_all(&spelling, "");
    let name = normalized.split_whitespace().collect::<Vec<_>>().join(" ");
    let mapped = match database {
        DatabaseType::Postgres => postgres::map(&name),
        DatabaseType::Mysql => mysql::map(&name),
        DatabaseType::Sqlite => sqlite::map(&name),
        DatabaseType::Mongo => None,
    };
    mapped
        .map(str::to_owned)
        .ok_or_else(|| invalid(format!("unsupported SQL type: {data_type}")))
}
