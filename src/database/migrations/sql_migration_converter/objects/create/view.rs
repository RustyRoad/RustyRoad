use super::super::super::alter::{clean, qualified};
use super::super::super::SqlOperation;
use regex::Regex;

pub(in crate::database::migrations::sql_migration_converter::objects) fn parse(
    stmt: &str,
) -> Option<Vec<SqlOperation>> {
    let pattern = Regex::new(&format!(
        r"(?is)^CREATE\s+(?:OR\s+REPLACE\s+)?(?:TEMP(?:ORARY)?\s+)?(MATERIALIZED\s+)?VIEW\s+(?:IF\s+NOT\s+EXISTS\s+)?({})",
        qualified()
    ))
    .unwrap();
    let captures = pattern.captures(stmt)?;
    let kind = if captures.get(1).is_some() {
        "MATERIALIZED VIEW"
    } else {
        "VIEW"
    };
    let name = clean(&captures[2]);
    Some(vec![SqlOperation::Cleanup {
        summary: format!("CREATE {kind} {name}"),
        down_sql: format!("DROP {kind} IF EXISTS {name};"),
    }])
}
