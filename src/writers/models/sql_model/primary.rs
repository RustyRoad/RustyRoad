use super::{invalid, names};
use crate::database::DatabaseType;
use sqlparser::ast::{ColumnOption, CreateTable, Expr, TableConstraint};
use std::{collections::HashSet, io};

pub(super) fn columns(table: &CreateTable, database: &DatabaseType) -> io::Result<HashSet<String>> {
    let mut keys = HashSet::new();
    for column in &table.columns {
        if column
            .options
            .iter()
            .any(|option| matches!(option.option, ColumnOption::PrimaryKey(_)))
        {
            keys.insert(names::key(&column.name, database));
        }
    }
    for constraint in &table.constraints {
        if let TableConstraint::PrimaryKey(key) = constraint {
            for column in &key.columns {
                let Expr::Identifier(name) = &column.column.expr else {
                    return Err(invalid("primary keys must reference column names"));
                };
                keys.insert(names::key(name, database));
            }
        }
    }
    for key in &keys {
        if !table
            .columns
            .iter()
            .any(|column| names::key(&column.name, database) == *key)
        {
            return Err(invalid(format!("unknown primary-key column {key:?}")));
        }
    }
    Ok(keys)
}
