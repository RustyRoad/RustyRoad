use super::invalid;
use crate::database::DatabaseType;
use sqlparser::{
    ast::{CreateTable, Statement},
    dialect::*,
    parser::Parser,
};
use std::io;

pub(super) fn table(sql: &str, database: &DatabaseType) -> io::Result<CreateTable> {
    let dialect: Box<dyn Dialect> = match database {
        DatabaseType::Postgres => Box::new(PostgreSqlDialect {}),
        DatabaseType::Mysql => Box::new(MySqlDialect {}),
        DatabaseType::Sqlite => Box::new(SQLiteDialect {}),
        DatabaseType::Mongo => {
            return Err(invalid("SQL model generation does not support MongoDB"))
        }
    };
    let statements = Parser::parse_sql(dialect.as_ref(), sql)
        .map_err(|error| invalid(format!("invalid model SQL: {error}")))?;
    let mut table = None;
    for statement in statements {
        match statement {
            Statement::CreateTable(value) if table.is_none() => table = Some(value),
            Statement::CreateIndex(_) | Statement::Comment { .. } => {}
            _ => {
                return Err(invalid(
                    "expected one CREATE TABLE, optionally with indexes/comments",
                ))
            }
        }
    }
    let table = table.ok_or_else(|| invalid("model SQL has no CREATE TABLE"))?;
    if table.columns.is_empty()
        || table.query.is_some()
        || table.like.is_some()
        || table.clone.is_some()
        || table.inherits.is_some()
        || table.partition_of.is_some()
    {
        return Err(invalid(
            "model requires an explicit, self-contained column list",
        ));
    }
    Ok(table)
}
