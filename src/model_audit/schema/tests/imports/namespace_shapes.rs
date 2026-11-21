//! Alias expansion must preserve invalid namespace shapes instead of erasing them.
use super::audits;

/// Example: `db<i32>::Uuid` cannot become the concrete `uuid::Uuid` codec.
#[test]
fn parameterized_alias_namespaces_remain_unresolved() {
    for ty in [
        "db<i32>::Uuid",
        "db::types<i32>::Uuid",
        "<Other as Trait>::Key",
        "Option<db<i32>::Uuid>",
        "Vec<db::types<i32>::Uuid>",
    ] {
        let source = format!(
            "use uuid::Uuid as Key; mod nested {{
                use sqlx as db; use super::Key;
                struct Row {{ id: i32, value: {ty} }}
            }}"
        );
        syn::parse_file(&source).unwrap_or_else(|error| panic!("{ty}: {error}"));
        let result = audits(&source);
        let row = &result[0];
        assert!(row.proposal.is_none(), "{ty}");
        assert!(
            row.blockers
                .iter()
                .any(|reason| reason.contains("storage mapping")),
            "{ty}: {:?}",
            row.blockers
        );
    }
}

mod concrete;
