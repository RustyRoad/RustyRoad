//! File ownership: derived files are rewritten, yours are preserved.

use super::support::schema;
use super::writer_support::{outcome_for, pull, scratch};
use crate::generators::typescript::writer::Outcome;

#[test]
fn a_first_pull_writes_everything() {
    let out = scratch("first");
    let report = pull(&out, &schema(), false);

    assert!(!report.outcomes.iter().any(Outcome::is_preserved));
    for name in ["schema.ts", "zod.ts", "router.ts", "api.ts", "server.ts"] {
        assert!(out.join(name).exists(), "missing {name}");
    }

    let _ = std::fs::remove_dir_all(&out);
}

#[test]
fn hand_written_procedures_survive_a_second_pull() {
    let out = scratch("preserve");
    pull(&out, &schema(), false);

    // Stand in for a developer composing a hand-written router into api.ts.
    let api = out.join("api.ts");
    let edited = "// my composition\nexport const router = { billing: billingRouter };\n";
    std::fs::write(&api, edited).unwrap();

    let report = pull(&out, &schema(), false);

    assert!(outcome_for(&report.outcomes, "api.ts").is_preserved());
    assert_eq!(std::fs::read_to_string(&api).unwrap(), edited);

    let _ = std::fs::remove_dir_all(&out);
}

#[test]
fn derived_files_are_always_rewritten() {
    let out = scratch("derived");
    pull(&out, &schema(), false);

    // Editing a derived file is always a mistake, so it must be reverted.
    let router = out.join("router.ts");
    std::fs::write(&router, "// stale\n").unwrap();

    let report = pull(&out, &schema(), false);

    assert!(!outcome_for(&report.outcomes, "router.ts").is_preserved());
    assert!(std::fs::read_to_string(&router).unwrap().contains("generated"));

    let _ = std::fs::remove_dir_all(&out);
}
