//! CREATE proposals preserve explicit ownership and key decisions.
use super::{empty, model};
use crate::model_audit::schema::{contracts, matching, proposals};

#[test]
fn explicit_new_table_has_reversible_quoted_sql_and_no_invented_defaults() {
    let model = model("pub struct NewCustomer { pub id: i32, pub note: Option<String> }");
    let catalog = empty();
    let map = matching::mappings(&["NewCustomer=customers".into()], &[model.clone()]).unwrap();
    let keys = contracts::keys(vec!["customers=id".into()]).unwrap();
    let mut audit = matching::audit(&model, &catalog, map.get(&model.id));
    proposals::build(&model, &catalog, &mut audit, &keys["customers"]);
    let sql = audit.proposal.unwrap();
    assert!(sql.up.contains("CREATE TABLE \"public\".\"customers\""));
    assert!(sql.up.contains("\"id\" integer NOT NULL"));
    assert!(sql.up.contains("PRIMARY KEY (\"id\")"));
    assert!(sql.up.contains("\"note\" text"));
    assert!(!sql.up.contains("DEFAULT"));
    assert_eq!(sql.down, "DROP TABLE \"public\".\"customers\";\n");
}

#[test]
fn absent_struct_does_not_invent_a_persistence_contract() {
    let model = model("struct Request { id: i32 }");
    let catalog = empty();
    let mut audit = matching::audit(&model, &catalog, None);
    proposals::build(&model, &catalog, &mut audit, &[]);
    assert!(audit.proposal.is_none());
    assert!(audit
        .blockers
        .iter()
        .any(|reason| reason.contains("explicit persistence contract")));
    assert_eq!(audit.mapping, "unresolved_persistence_contract");
}
