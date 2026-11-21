//! Ambiguous import bindings must never invent PostgreSQL codecs.

use super::audits;

#[test]
fn local_shadowing_does_not_impersonate_a_known_codec() {
    let audits = audits("struct Uuid; struct Event { id: Uuid }");
    let event = audits
        .iter()
        .find(|audit| audit.table.as_deref() == Some("event"))
        .unwrap();
    assert!(event.proposal.is_none());
    assert!(event
        .blockers
        .iter()
        .any(|error| error.contains("storage mapping")));
}

#[test]
fn unknown_and_conditional_imports_require_a_reviewed_mapping() {
    for source in [
        "use custom::Uuid; struct Event { id: Uuid }",
        "#[cfg(feature = \"uuid\")] use uuid::Uuid; struct Event { id: Uuid }",
    ] {
        assert!(audits(source)[0].proposal.is_none());
    }
}

mod declarations;
