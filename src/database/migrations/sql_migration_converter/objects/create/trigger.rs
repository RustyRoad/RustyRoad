use super::super::super::alter::{clean, qualified, IDENT};
use super::super::super::SqlOperation;
use regex::Regex;

pub(in crate::database::migrations::sql_migration_converter::objects) fn parse(
    stmt: &str,
) -> Option<Vec<SqlOperation>> {
    let pattern = Regex::new(&format!(
        r"(?is)^CREATE\s+(?:OR\s+REPLACE\s+)?TRIGGER\s+({IDENT})\s+.*?\bON\s+({})",
        qualified()
    ))
    .unwrap();
    let captures = pattern.captures(stmt)?;
    let trigger = &captures[1];
    let table = clean(&captures[2]);
    Some(vec![SqlOperation::Cleanup {
        summary: format!("CREATE TRIGGER {trigger} ON {table}"),
        down_sql: format!("DROP TRIGGER IF EXISTS {trigger} ON {table};"),
    }])
}
