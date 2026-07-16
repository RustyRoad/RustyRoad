use super::super::super::detect_rogue_migrations;
use super::operation;

pub(super) fn run() {
    let detected = detect_rogue_migrations();
    if detected.is_empty() {
        println!("No rogue migrations detected.");
        return;
    }
    println!(
        "Found {} rogue migration(s) that would be converted:\n",
        detected.len()
    );
    for (index, migration) in detected.iter().enumerate() {
        println!(
            "  {}. {} (from {:?})",
            index + 1,
            migration.name,
            migration.source_path
        );
        println!("     Operations:");
        migration.operations.iter().for_each(operation::print);
        println!();
    }
    println!("\nRun without --dry-run to convert these migrations.");
}
