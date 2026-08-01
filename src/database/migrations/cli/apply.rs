use super::super::{run_all_migrations, run_migration, MigrationDirection};
use crate::helpers::helpers::get_project_name_from_rustyroad_toml;
use clap::ArgMatches;

pub(super) async fn all(matches: &ArgMatches) {
    super::print_config();
    get_project_name_from_rustyroad_toml()
        .unwrap_or_else(|error| panic!("This is not a Rusty Road project: {error}"));
    enforce(super::breaking_change::approve_all(matches).await);
    run_all_migrations(MigrationDirection::Up)
        .await
        .expect("Error running migrations");
}

pub(super) async fn one(matches: &ArgMatches) {
    super::print_config();
    let name = matches.get_one::<String>("name").unwrap().to_string();
    enforce(super::breaking_change::approve_named(&name, matches).await);
    run_migration(name.clone(), MigrationDirection::Up)
        .await
        .expect("Error running migration");
    println!("'{name}' migration completed successfully!");
}

fn enforce(result: std::io::Result<()>) {
    if let Err(error) = result {
        eprintln!("{error}");
        std::process::exit(2);
    }
}
