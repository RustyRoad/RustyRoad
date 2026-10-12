use super::super::split::split_outside;
use super::super::SqlOperation;
use super::action::classify;
use super::action::qualified;
use super::assemble::assemble;
use regex::Regex;

pub(in crate::database::migrations::sql_migration_converter) fn split_alter(
    stmt: &str,
) -> Option<(String, Vec<String>)> {
    let re = Regex::new(&format!(
        r"(?is)^ALTER\s+TABLE\s+(?:IF\s+EXISTS\s+)?(?:ONLY\s+)?({})\s+(.*)$",
        qualified()
    ))
    .ok()?;
    let c = re.captures(stmt)?;
    Some((super::action::clean(&c[1]), split_outside(&c[2], ',')))
}

pub(in crate::database::migrations::sql_migration_converter) fn parse_alter_table(
    stmt: &str,
) -> Vec<SqlOperation> {
    let Some((table, actions)) = split_alter(stmt) else {
        return vec![SqlOperation::RawSql {
            sql: stmt.to_string(),
        }];
    };
    let classified = actions.iter().map(|action| classify(action)).collect();
    assemble(&table, stmt, classified)
}
