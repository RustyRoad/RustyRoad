//! What counts as a wired-up table in a hand-edited composition.

use super::support::{column, from_tables, users};
use super::wiring_test::{orders, unwired};
use super::writer_support::scratch;
use crate::database::introspection::Table;

#[test]
fn a_hand_edited_composition_still_counts_as_wired() {
    let out = scratch("wiring-edited");
    unwired(&out, &from_tables(vec![users(), orders()]));

    // A developer may restructure the file freely; what matters is the reference.
    std::fs::write(
        out.join("api.ts"),
        "export const router = {\n\tu: generated.users,\n\to: generated.orders,\n};\n",
    )
    .unwrap();

    assert!(unwired(&out, &from_tables(vec![users(), orders()])).is_empty());

    let _ = std::fs::remove_dir_all(&out);
}

#[test]
fn spreading_the_namespace_wires_every_table() {
    let out = scratch("wiring-spread");
    unwired(&out, &from_tables(vec![users()]));

    // Spreading covers tables added later, so nothing should be reported.
    std::fs::write(
        out.join("api.ts"),
        "export const router = { ...generated };\n",
    )
    .unwrap();

    assert!(unwired(&out, &from_tables(vec![users(), orders()])).is_empty());

    let _ = std::fs::remove_dir_all(&out);
}

#[test]
fn composite_key_tables_are_not_reported() {
    let out = scratch("wiring-composite");
    unwired(&out, &from_tables(vec![users()]));

    let memberships = Table {
        name: "memberships".to_string(),
        primary_key: vec!["user_id".to_string(), "group_id".to_string()],
        columns: vec![column("user_id", "integer"), column("group_id", "integer")],
        ..orders()
    };

    // No procedures are generated for a composite key, so none are missing.
    let reported = unwired(&out, &from_tables(vec![users(), memberships]));
    assert!(reported.is_empty(), "unexpected: {reported:?}");

    let _ = std::fs::remove_dir_all(&out);
}
