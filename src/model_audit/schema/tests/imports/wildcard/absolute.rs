//! Absolute paths distinguish external identities from unproven bare names.
use super::super::audits;

/// `::String` denotes a crate-root name, not `std::string::String`.
#[test]
fn absolute_bare_names_cannot_borrow_prelude_codecs() {
    for ty in ["::String", "::Option<i32>", "::Vec<i32>", "::i32"] {
        let result = audits(&format!("struct Row {{ value: {ty} }}"));
        assert!(result.last().unwrap().proposal.is_none(), "{ty}");
    }
}

/// Fully qualified standard-library identities still establish their codecs.
#[test]
fn absolute_standard_library_paths_retain_known_codecs() {
    let result = audits(
        r#"
        use unknown::*;
        struct Row { value: ::std::option::Option<::std::string::String> }
    "#,
    );
    let row = result.last().unwrap();
    assert!(row.blockers.is_empty(), "{:?}", row.blockers);
    assert_eq!(row.desired_columns[0].sql_type, "text");
    assert!(row.desired_columns[0].nullable);
}
