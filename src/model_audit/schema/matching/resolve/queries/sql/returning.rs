//! Extract whole-row RETURNING ownership without executing modifying statements.
use super::{relation, whole};
use sqlparser::ast::{FromTable, Statement, TableObject};

/// For example, `UPDATE sessions SET id = $1 RETURNING *` owns `sessions`.
pub(super) fn target(statement: &Statement) -> Option<String> {
    match statement {
        Statement::Insert(insert) => {
            if !whole(insert.returning.as_ref()?) {
                return None;
            }
            let TableObject::TableName(name) = &insert.table else {
                return None;
            };
            relation::name(name)
        }
        Statement::Update(update) if update.from.is_none() => {
            if !whole(update.returning.as_ref()?) {
                return None;
            }
            relation::table(&update.table)
        }
        Statement::Delete(delete) if delete.using.is_none() && delete.tables.is_empty() => {
            if !whole(delete.returning.as_ref()?) {
                return None;
            }
            let tables = match &delete.from {
                FromTable::WithFromKeyword(tables) | FromTable::WithoutKeyword(tables) => tables,
            };
            let [table] = tables.as_slice() else {
                return None;
            };
            relation::table(table)
        }
        _ => None,
    }
}
