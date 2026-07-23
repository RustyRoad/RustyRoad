use rustyroad::Project;

#[path = "support/migration_breaking_change.rs"]
mod support;
use support::{assert_blocked, project, run};

#[test]
fn allow_breaking_flag_reaches_run_subcommand() {
    let matches = Project::cli()
        .try_get_matches_from([
            "rustyroad",
            "migration",
            "run",
            "change_customer_id",
            "--allow-breaking",
        ])
        .unwrap();
    let (_, migration) = matches.subcommand().unwrap();
    let (_, run) = migration.subcommand().unwrap();
    assert!(run.get_flag("allow-breaking"));
}

#[test]
fn noninteractive_run_blocks_before_database_connection() {
    let root = project();
    let output = run(&root, &["migration", "run", "change_customer_id"]);
    assert_blocked(&output);
}

#[test]
fn noninteractive_all_blocks_before_database_connection() {
    let root = project();
    let output = run(&root, &["migration", "all"]);
    assert_blocked(&output);
}
