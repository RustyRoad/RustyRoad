mod redo;
mod reset;

use super::super::{run_migration, MigrationDirection};
use clap::ArgMatches;
use dialoguer::Confirm;

pub(super) async fn one(matches: &ArgMatches) {
    super::print_config();
    let name = matches.get_one::<String>("name").unwrap().to_string();
    let confirmed = Confirm::new()
        .with_prompt(format!(
            "Are you sure you want to rollback the '{name}' migration?"
        ))
        .interact()
        .expect("Error confirming rollback migration");
    if !confirmed {
        println!("'{name}' migration rollback canceled by user.");
        return;
    }
    println!("Rolling back the '{name}' migration...");
    run_migration(name.clone(), MigrationDirection::Down)
        .await
        .expect("Error rolling back migration");
    println!("'{name}' migration rollback completed successfully!");
}

pub(super) async fn redo(matches: &ArgMatches) {
    redo::run(matches).await;
}

pub(super) async fn reset() {
    reset::run().await;
}
