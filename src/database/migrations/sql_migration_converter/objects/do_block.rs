use super::super::split::{dollar_body, split_outside};
use super::super::{parse_statement, SqlOperation};
use regex::Regex;

fn raw(stmt: &str) -> Vec<SqlOperation> {
    vec![SqlOperation::RawSql {
        sql: stmt.to_string(),
    }]
}

pub(super) fn parse(stmt: &str) -> Option<Vec<SqlOperation>> {
    if !Regex::new(r"(?i)^DO\b").unwrap().is_match(stmt) {
        return None;
    }
    let Some(body) = dollar_body(stmt) else {
        return Some(raw(stmt));
    };
    let start = Regex::new(
        r"(?i)\b(ALTER\s+TABLE|CREATE\s+(?:UNIQUE\s+)?INDEX|CREATE\s+TABLE|DROP\s+TABLE|DROP\s+INDEX|UPDATE|INSERT\s+INTO|DELETE\s+FROM)\b",
    ).unwrap();
    let mut ops = Vec::new();
    for fragment in split_outside(&body, ';') {
        let Some(m) = start.find(&fragment) else {
            continue;
        };
        for op in parse_statement(&fragment[m.start()..]) {
            ops.push(guard(op));
        }
    }
    Some(if ops.is_empty() { raw(stmt) } else { ops })
}

fn guard(op: SqlOperation) -> SqlOperation {
    match op {
        SqlOperation::AlterTableAddColumn {
            table_name,
            columns,
            ..
        } => {
            let down_sql = columns
                .iter()
                .map(|c| format!("ALTER TABLE IF EXISTS {table_name} DROP COLUMN IF EXISTS {c};"))
                .collect::<Vec<_>>()
                .join("\n");
            SqlOperation::Cleanup {
                summary: format!("guarded ADD COLUMN {} on {table_name}", columns.join(", ")),
                down_sql,
            }
        }
        other => other,
    }
}
