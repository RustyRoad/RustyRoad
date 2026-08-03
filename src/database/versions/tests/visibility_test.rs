//! Views must expose only client-facing columns and tables.

use crate::database::versions::ops::trigger::NEEDS_BACKFILL_COLUMN;

#[test]
fn backfill_marker_is_not_client_facing() {
    // Live failure: the marker column and RustyRoad's own history table appeared in
    // the published version's views, exposing implementation detail to clients.
    assert!(NEEDS_BACKFILL_COLUMN.starts_with("_rustyroad_"));
}

#[test]
fn internal_names_share_the_reserved_prefix() {
    use crate::database::versions::naming::{deletion_name, temporary_name};

    // Everything RustyRoad creates is recognisable by prefix, which is how
    // introspection filters it out of client-facing views.
    for name in [
        temporary_name("c"),
        deletion_name("c"),
        NEEDS_BACKFILL_COLUMN.to_string(),
    ] {
        assert!(name.starts_with("_rustyroad_"), "not reserved: {name}");
    }
}
