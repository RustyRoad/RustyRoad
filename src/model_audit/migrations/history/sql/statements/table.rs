//! Preserve PostgreSQL identifier identity when inspecting table migration history.
use crate::database::migrations::SqlOperation;
use sqlparser::{
    ast::{AlterTableOperation, Statement},
    dialect::PostgreSqlDialect,
    parser::Parser,
};

#[cfg(test)]
mod tests;

/// Parse table DDL, retaining unsupported syntax as conservatively inspected raw SQL.
/// For example, `ADD COLUMN "Note" text` must not become unquoted `note`.
pub(super) fn parse(sql: &str) -> Vec<SqlOperation> {
    let raw = || SqlOperation::RawSql {
        sql: sql.to_owned(),
    };
    let Ok(statements) = Parser::parse_sql(&PostgreSqlDialect {}, sql) else {
        return vec![raw()];
    };
    statements
        .into_iter()
        .map(|statement| match statement {
            Statement::CreateTable(table) => SqlOperation::CreateTable {
                table_name: table.name.to_string(),
                columns: table
                    .columns
                    .iter()
                    .map(|column| column.name.to_string())
                    .collect(),
                full_sql: sql.to_owned(),
            },
            Statement::AlterTable(table) => {
                let columns: Option<Vec<_>> = table
                    .operations
                    .iter()
                    .map(|operation| match operation {
                        AlterTableOperation::AddColumn { column_def, .. } => {
                            Some(column_def.name.to_string())
                        }
                        _ => None,
                    })
                    .collect();
                match columns {
                    Some(columns) if !columns.is_empty() => SqlOperation::AlterTableAddColumn {
                        table_name: table.name.to_string(),
                        columns,
                        full_sql: sql.to_owned(),
                    },
                    _ => raw(),
                }
            }
            _ => raw(),
        })
        .collect()
}
