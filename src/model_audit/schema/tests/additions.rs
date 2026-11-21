//! Added-column drafts are generated without fabricating data backfills.
use super::{model, rows};
use crate::model_audit::schema::{matching, proposals};

#[test]
fn actual_columns_not_rust_type_names_determine_additive_gaps() {
    let desired = model("struct Rows { id: i32, note: Option<String> }");
    let catalog = rows("table");
    let mut audit = matching::audit(&desired, &catalog, None);
    proposals::build(&desired, &catalog, &mut audit, &[]);
    let sql = audit.proposal.unwrap();
    assert_eq!(
        sql.up,
        "ALTER TABLE \"public\".\"rows\"\n  ADD COLUMN \"note\" text;\n"
    );
    assert_eq!(
        sql.down,
        "ALTER TABLE \"public\".\"rows\"\n  DROP COLUMN \"note\";\n"
    );
}

#[test]
fn required_additions_warn_instead_of_inventing_backfills() {
    let desired = model("struct Rows { id: i32, note: String }");
    let catalog = rows("table");
    let mut audit = matching::audit(&desired, &catalog, None);
    proposals::build(&desired, &catalog, &mut audit, &[]);
    let sql = audit.proposal.unwrap();
    assert!(sql.up.contains("ADD COLUMN \"note\" text NOT NULL"));
    assert!(!sql.up.contains("DEFAULT"));
    assert!(sql
        .warnings
        .iter()
        .any(|warning| warning.contains("backfill")));
}
