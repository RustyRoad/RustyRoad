use super::super::super::{run_all_migrations, MigrationDirection};
use dialoguer::Confirm;

pub(super) async fn run() {
    super::super::print_config();
    let confirmed = Confirm::new()
        .with_prompt(
            "Reset will rollback ALL migrations (down) in reverse order. This is destructive. Continue?",
        )
        .interact()
        .expect("Error confirming reset migrations");
    if !confirmed {
        println!("Migration reset canceled by user.");
        return;
    }
    run_all_migrations(MigrationDirection::Down)
        .await
        .expect("Error rolling back migrations");
    println!("All migrations rolled back successfully.");
}
