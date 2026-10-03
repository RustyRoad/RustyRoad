use rustyroad::Project;

#[test]
fn test_cli_has_db_enums_command() {
    // Test that the CLI includes the db enums command
    let app = Project::cli();
    let db = app
        .find_subcommand("db")
        .expect("db subcommand should exist");
    let enums = db
        .find_subcommand("enums")
        .expect("db enums subcommand should exist");

    assert_eq!(enums.get_name(), "enums");
    assert!(enums.get_about().is_some());
    assert!(enums.get_long_about().is_some());
}

#[test]
fn test_cli_db_requires_a_subcommand() {
    // `db` without a subcommand must show help rather than doing nothing
    let app = Project::cli();
    let db = app
        .find_subcommand("db")
        .expect("db subcommand should exist");

    assert!(db.is_subcommand_required_set());
}

#[test]
fn test_db_enums_accepts_global_format_flag() {
    // The global --format flag parses alongside `db enums`
    let matches = Project::cli()
        .try_get_matches_from(["rustyroad", "--format", "json", "db", "enums"])
        .expect("--format json db enums should parse");

    assert_eq!(
        matches.get_one::<String>("format").map(String::as_str),
        Some("json")
    );
    assert_eq!(matches.subcommand().map(|(name, _)| name), Some("db"));
}
