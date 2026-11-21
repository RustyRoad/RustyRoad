//! Extracted fixtures retain their original assertions and inputs.
use super::audits;

/// Example: `type Key = uuid::Uuid; Key<i32>` cannot prove a UUID codec.
#[test]
fn rejects_arguments_suffixes_and_cycles_on_closed_aliases() {
    for (aliases, ty) in [
        ("type Key = uuid::Uuid;", "Key<i32>"),
        ("type Keys = Vec<uuid::Uuid>;", "Keys<i32>"),
        ("type Maybe = Option<uuid::Uuid>;", "Maybe<i32>"),
        ("type Stamp = chrono::DateTime<chrono::Utc>;", "Stamp<i32>"),
        ("type Namespace = uuid;", "Namespace::Uuid"),
        ("type Key = uuid::Uuid;", "Key::Uuid"),
        ("type String = String;", "String"),
        ("type Key = Other; type Other = Key;", "Key"),
    ] {
        let source = format!("{aliases} struct Row {{ id: i32, value: {ty} }}");
        let result = audits(&source);
        let row = result.last().unwrap();
        assert!(row.proposal.is_none(), "{source}");
        assert!(
            row.blockers
                .iter()
                .any(|reason| reason.contains("storage mapping")),
            "{source}: {:?}",
            row.blockers
        );
    }
}
