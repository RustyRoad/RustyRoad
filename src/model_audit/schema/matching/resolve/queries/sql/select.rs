//! Extract a whole-row SELECT target after statement-level CTE rejection.
use super::{relation, whole};
use sqlparser::ast::{Query, SetExpr};

/// For example, `SELECT * FROM rows WHERE id = $1` targets `rows`.
pub(super) fn target(query: &Query) -> Option<String> {
    let SetExpr::Select(select) = &*query.body else {
        return None;
    };
    if !whole(&select.projection)
        || select.into.is_some()
        || select.having.is_some()
        || select.qualify.is_some()
        || !select.lateral_views.is_empty()
    {
        return None;
    }
    let [table] = select.from.as_slice() else {
        return None;
    };
    relation::table(table)
}
