use super::super::super::detect_and_convert_rogue_migrations;

pub(super) fn run(remove_source: bool) {
    let count = detect_and_convert_rogue_migrations(true, remove_source);
    if count == 0 {
        println!("No rogue migrations found to convert.");
    }
}
