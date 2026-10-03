//! Enum variants whose Rust identifiers would collide.
//!
//! A Postgres enum may hold both `campaign_type` and `campaign-type` as labels — the
//! spotlessbinco `campaign_workflow_node_kind` type does. Both fold to `CampaignType`, so the
//! generated enum declared the same variant twice and the crate did not compile.

use super::variant_fixture::shared;

#[test]
fn colliding_variant_labels_get_distinct_identifiers() {
    let shared = shared();

    // Scoped to the enum body, since the file also holds imports.
    let body = shared
        .split("pub enum NodeKind {")
        .nth(1)
        .expect("the enum should be declared")
        .split('}')
        .next()
        .unwrap();

    let declared: Vec<&str> = body
        .lines()
        .map(str::trim)
        .filter(|line| line.ends_with(',') && !line.starts_with('#'))
        .collect();

    let mut sorted = declared.clone();
    sorted.sort_unstable();
    let before = sorted.len();
    sorted.dedup();

    assert_eq!(
        before,
        sorted.len(),
        "a variant identifier is declared twice: {declared:?}"
    );
    assert_eq!(before, 3, "expected one variant per label: {declared:?}");
}

#[test]
fn every_label_keeps_its_own_rename() {
    let shared = shared();

    // Whatever the identifiers end up being, each label must still round-trip: dropping one
    // would silently map two database values onto a single variant.
    for label in ["campaign_type", "campaign-type", "phone-call"] {
        assert!(
            shared.contains(&format!("#[sqlx(rename = \"{label}\")]")),
            "label {label} lost its rename:\n{shared}"
        );
    }
}

#[test]
fn a_variant_carries_exactly_one_rename_pair() {
    let shared = shared();

    // The first bug here emitted two sqlx renames before a single identifier, so the
    // attribute count has to match the variant count.
    assert_eq!(shared.matches("#[sqlx(rename =").count(), 3, "{shared}");
    assert_eq!(shared.matches("#[serde(rename =").count(), 3, "{shared}");
}
