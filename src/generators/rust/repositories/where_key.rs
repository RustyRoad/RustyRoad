//! Dynamic bound predicates after a variable-length SET clause.

use super::super::naming::sql;
use super::selector;

/// Appends key binds; e.g. `render(&["token".into()])` follows all supplied Patch binds.
/// QueryBuilder numbers placeholders, so omitted fields cannot shift the wrong key.
pub(super) fn render(key: &[String]) -> String {
    key.iter().enumerate().map(|(index, column)| {
        let prefix = if index == 0 { " WHERE " } else { " AND " };
        let predicate = format!("{prefix}{} = ", sql(column));
        let argument = selector::key_argument(index + 1, key.len());
        format!("    query.push({predicate:?}).push_bind({argument});\n")
    }).collect()
}