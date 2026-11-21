use super::command;
use clap::error::ErrorKind;

#[test]
fn preview_is_default_and_does_not_require_database_options() {
    let args = command().try_get_matches_from(["repair-down"]).unwrap();
    assert_eq!(
        args.get_one::<String>("path").map(String::as_str),
        Some("./config/database/migrations")
    );
    assert!(!args.get_flag("write"));
    assert!(!args.get_flag("dry-run"));
    assert!(args.get_one::<String>("reviewed-down-dir").is_none());
}

#[test]
fn explicit_preview_conflicts_with_writing() {
    let args = command()
        .try_get_matches_from(["repair-down", "--dry-run"])
        .unwrap();
    assert!(args.get_flag("dry-run"));
    assert!(!args.get_flag("write"));
    let error = command()
        .try_get_matches_from(["repair-down", "--dry-run", "--write"])
        .unwrap_err();
    assert_eq!(error.kind(), ErrorKind::ArgumentConflict);
}

#[test]
fn reviewed_write_accepts_an_explicit_migration_directory() {
    let args = command()
        .try_get_matches_from([
            "repair-down",
            "--path",
            "config/database/migrations/20260101000000-example",
            "--reviewed-down-dir",
            "reviewed-undo",
            "--write",
        ])
        .unwrap();
    assert!(args.get_flag("write"));
    assert_eq!(
        args.get_one::<String>("reviewed-down-dir")
            .map(String::as_str),
        Some("reviewed-undo")
    );
    assert_eq!(
        args.get_one::<String>("path").map(String::as_str),
        Some("config/database/migrations/20260101000000-example")
    );
}

#[test]
fn lifecycle_registers_file_repair_separately_from_ledger_repair() {
    let commands = super::super::lifecycle_commands();
    assert_eq!(
        commands
            .iter()
            .filter(|c| c.get_name() == "repair-down")
            .count(),
        1
    );
    assert_eq!(
        commands
            .iter()
            .filter(|c| c.get_name() == "repair-ledger")
            .count(),
        1
    );
    let args = super::super::migration_command()
        .subcommands(commands)
        .try_get_matches_from(["migration", "repair-down", "--dry-run"])
        .unwrap();
    let (name, repair) = args.subcommand().unwrap();
    assert_eq!(name, "repair-down");
    assert!(repair.get_flag("dry-run"));
}
