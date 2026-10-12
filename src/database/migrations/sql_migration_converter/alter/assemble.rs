use super::super::SqlOperation;
use super::action::Action;

pub(super) fn assemble(table: &str, stmt: &str, actions: Vec<Action>) -> Vec<SqlOperation> {
    let (mut adds, mut drops, mut other) = (Vec::new(), Vec::new(), Vec::new());
    for action in actions {
        match action {
            Action::Add(name) => adds.push(name),
            Action::Drop(name) => drops.push(name),
            Action::Constraint(name) => other.push(SqlOperation::Cleanup {
                summary: format!("ALTER TABLE {table} ADD CONSTRAINT {name}"),
                down_sql: format!("ALTER TABLE {table} DROP CONSTRAINT IF EXISTS {name};"),
            }),
            Action::Raw(sql) => other.push(SqlOperation::RawSql { sql }),
        }
    }
    let mut ops = Vec::new();
    if !adds.is_empty() {
        ops.push(SqlOperation::AlterTableAddColumn {
            table_name: table.to_string(),
            columns: adds,
            full_sql: stmt.to_string(),
        });
    }
    ops.extend(other);
    if !drops.is_empty() {
        ops.push(SqlOperation::AlterTableDropColumn {
            table_name: table.to_string(),
            columns: drops,
            full_sql: stmt.to_string(),
        });
    }
    ops
}
