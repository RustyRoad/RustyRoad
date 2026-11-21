//! Explicit imports and source contracts remain usable beside unknown globs.
use super::super::audits;

#[test]
fn absolute_imports_retain_identity_despite_wildcards() {
    let result = audits(
        r#"
        use custom::*;
        use ::std::string::String as Text;
        use ::std::option::Option as Maybe;
        use ::uuid::Uuid as Id;
        struct Row { id: Id, value: Maybe<Text> }
    "#,
    );
    let row = result.last().unwrap();
    assert!(row.blockers.is_empty(), "{:?}", row.blockers);
    assert_eq!(row.desired_columns[0].sql_type, "uuid");
    assert_eq!(row.desired_columns[1].sql_type, "text");
    assert!(row.desired_columns[1].nullable);
}

#[test]
fn explicit_source_contracts_are_not_erased_by_wildcards() {
    let result = audits(
        r#"
        use custom::*;
        #[derive(::sqlx::Type)] #[sqlx(type_name = "state")] enum State { A }
        struct Row { state: State }
    "#,
    );
    assert_eq!(
        result.last().unwrap().desired_columns[0].sql_type,
        "\"state\""
    );
}
