use super::super::super::{run_migration, MigrationDirection};
use clap::ArgMatches;
use dialoguer::Confirm;

pub(super) async fn run(matches: &ArgMatches) {
    super::super::print_config();
    let name = matches.get_one::<String>("name").unwrap().to_string();
    let confirmed = Confirm::new()
        .with_prompt(format!(
            "Redo will rollback (down) then re-apply (up) the '{name}' migration. Continue?"
        ))
        .interact()
        .expect("Error confirming redo migration");
    if !confirmed {
        println!("'{name}' migration redo canceled by user.");
        return;
    }
    println!("Rolling back '{name}'...");
    run_migration(name.clone(), MigrationDirection::Down)
        .await
        .expect("Error rolling back migration");
    println!("Re-applying '{name}'...");
    run_migration(name.clone(), MigrationDirection::Up)
        .await
        .expect("Error running migration");
    println!("'{name}' migration redo completed successfully!");
}
