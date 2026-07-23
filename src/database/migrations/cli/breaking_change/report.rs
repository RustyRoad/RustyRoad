use crate::database::migrations::BreakingChangeFinding;

pub(super) fn warnings(findings: &[BreakingChangeFinding]) {
    if findings.is_empty() {
        return;
    }
    eprintln!("\nBREAKING MIGRATION WARNING");
    eprintln!("RustyRoad detected schema operations that can destroy data or break foreign keys:");
    for finding in findings {
        eprintln!("  - {finding}");
    }
    eprintln!(
        "\nChanging a foreign-key column type can leave referencing and referenced columns incompatible."
    );
    eprintln!("Review both sides of every foreign key and preserve all values before applying.");
    eprintln!("Run `rustyroad migration validate` against a disposable database first.");
}
