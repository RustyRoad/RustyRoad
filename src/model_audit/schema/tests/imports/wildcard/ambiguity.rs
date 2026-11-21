//! Unknown globs block ambiguous names only within their importing scope.
use super::super::audits;

#[test]
fn wildcard_imports_block_unproven_prelude_and_crate_names() {
    for import in [
        "use custom::*;",
        "use custom::{nested::*};",
        "#[cfg(feature = \"custom\")] use custom::*;",
    ] {
        for ty in ["String", "Option<i32>", "Vec<i32>", "uuid::Uuid"] {
            let result = audits(&format!("{import} struct Row {{ value: {ty} }}"));
            let row = result.last().unwrap();
            assert!(row.proposal.is_none(), "{import}: {ty}");
            assert!(row
                .blockers
                .iter()
                .any(|reason| reason.contains("storage mapping")));
        }
    }
}

#[test]
fn wildcards_are_scope_local_but_explicit_parent_paths_remain_guarded() {
    let result = audits(
        r#"
        use custom::*;
        mod inner { struct Row { value: String } }
    "#,
    );
    assert!(result.last().unwrap().blockers.is_empty());
    let result = audits(
        r#"
        use custom::*;
        mod inner { struct Row { value: super::String } }
    "#,
    );
    assert!(result.last().unwrap().proposal.is_none());
}
