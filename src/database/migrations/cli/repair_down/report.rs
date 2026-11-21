//! Report planned and written files distinctly, including partial publication failures.

use crate::database::migrations::repair_down::{Entry, Status};

pub(super) fn print(entries: Vec<Entry>, write: bool) {
    let blocked = entries
        .iter()
        .any(|entry| matches!(entry.status, Status::Blocked));
    for entry in entries {
        println!(
            "{:?}: {} — {}",
            entry.status,
            entry.directory.display(),
            entry.detail
        );
        if let Some(sql) = entry.sql {
            println!("{sql}");
        }
    }
    if blocked {
        eprintln!("Repair blocked; inspect every status. Ready entries were not written.");
        std::process::exit(2);
    }
    if !write {
        println!("Preview only: no files changed. Review undo SQL before using --write.");
    }
}
