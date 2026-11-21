//! Extracted fixtures retain their original assertions and inputs.
use super::audits;

/// Type-namespace declarations must not masquerade as preconfigured codecs.
#[test]
fn traits_and_extern_crate_aliases_shadow_known_codec_names() {
    for declaration in [
        "trait Uuid {}",
        "trait Uuid = Send;",
        "extern crate custom as Uuid;",
        "extern crate custom as sqlx;",
    ] {
        let ty = if declaration.contains("as sqlx") {
            "sqlx::types::Uuid"
        } else {
            "Uuid"
        };
        let source = format!("{declaration} struct Event {{ id: {ty} }}");
        let result = audits(&source);
        let event = result
            .iter()
            .find(|audit| audit.table.as_deref() == Some("event"))
            .unwrap();
        assert!(event.proposal.is_none(), "{declaration}");
        assert!(
            event
                .blockers
                .iter()
                .any(|reason| reason.contains("storage mapping")),
            "{declaration}: {:?}",
            event.blockers
        );
    }
}
