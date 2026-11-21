//! Reject aliases that rename or otherwise modify the physical row shape.
use sqlparser::ast::TableAlias;

/// A plain `AS s` is safe; `AS s(renamed_id)` changes the decoded columns.
pub(super) fn valid(alias: &Option<TableAlias>) -> bool {
    alias.as_ref().is_none_or(|alias| {
        alias.name.quote_style.is_none() && alias.columns.is_empty() && alias.at.is_none()
    })
}
