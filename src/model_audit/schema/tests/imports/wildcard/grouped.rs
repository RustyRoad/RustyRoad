//! Every grouped wildcard must be proven, independent of import order.
use super::super::audits;

#[test]
fn value_helpers_cannot_hide_unsafe_grouped_globs() {
    for imports in [
        "use self::{values::*, types::*};",
        "use self::{types::*, values::*};",
        "use self::{values::*, external::*};",
        "use self::{external::*, values::*};",
    ] {
        let result = audits(&format!(
            "mod values {{ pub fn value() {{}} }}
             mod types {{ pub struct String; }}
             {imports}
             struct Row {{ value: String }}"
        ));
        let row = result.last().unwrap();
        assert!(row.proposal.is_none(), "{imports}");
        assert!(!row.blockers.is_empty(), "{imports}");
        assert!(
            row.desired_columns
                .iter()
                .all(|column| column.name != "value"),
            "{imports}: {:?}",
            row.desired_columns
        );
    }
}
