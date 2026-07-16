use rustyroad::Project;

#[test]
fn migration_validate_command_is_available() {
    let app = Project::cli();
    let migration_command = app.find_subcommand("migration").unwrap();
    let validate_command = migration_command.find_subcommand("validate").unwrap();

    assert_eq!(validate_command.get_name(), "validate");
    assert_eq!(validate_command.get_arguments().count(), 0);
}

#[test]
fn migration_validate_command_parses_without_database_arguments() {
    let matches = Project::cli()
        .try_get_matches_from(["rustyroad", "migration", "validate"])
        .unwrap();
    let (command, migration_matches) = matches.subcommand().unwrap();

    assert_eq!(command, "migration");
    assert_eq!(migration_matches.subcommand_name(), Some("validate"));
}
