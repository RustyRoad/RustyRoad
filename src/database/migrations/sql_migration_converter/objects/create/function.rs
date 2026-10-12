use super::super::super::alter::{clean, qualified};
use super::super::super::SqlOperation;
use regex::Regex;

fn raw(stmt: &str) -> Vec<SqlOperation> {
    vec![SqlOperation::RawSql {
        sql: stmt.to_string(),
    }]
}

pub(in crate::database::migrations::sql_migration_converter::objects) fn parse(
    stmt: &str,
) -> Option<Vec<SqlOperation>> {
    let pattern = Regex::new(&format!(
        r"(?is)^CREATE\s+(?:OR\s+REPLACE\s+)?FUNCTION\s+({})\s*\((.*?)\)\s*(?:RETURNS|AS|LANGUAGE|$)",
        qualified()
    ))
    .unwrap();
    let captures = pattern.captures(stmt)?;
    let name = clean(&captures[1]);
    if !captures[2].trim().is_empty() {
        return Some(raw(stmt));
    }
    Some(vec![SqlOperation::Cleanup {
        summary: format!("CREATE FUNCTION {name}()"),
        down_sql: format!("DROP FUNCTION IF EXISTS {name}();"),
    }])
}
