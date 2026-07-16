use super::super::{run_all_migrations, run_migration, MigrationDirection};
use crate::helpers::helpers::get_project_name_from_rustyroad_toml;
use clap::ArgMatches;

pub(super) async fn all() {
    super::print_config();
    get_project_name_from_rustyroad_toml()
        .unwrap_or_else(|error| panic!("This is not a Rusty Road project: {error}"));
    run_all_migrations(MigrationDirection::Up)
        .await
        .expect("Error running migrations");
}

pub(super) async fn one(matches: &ArgMatches) {
    super::print_config();
    let name = matches.get_one::<String>("name").unwrap().to_string();
    run_migration(name.clone(), MigrationDirection::Up)
        .await
        .expect("Error running migration");
    println!("'{name}' migration completed successfully!");
}
