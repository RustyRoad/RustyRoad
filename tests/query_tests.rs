use rustyroad::Project;

#[test]
fn test_cli_has_query_command() {
    // Test that the CLI includes the query command
    let app = Project::cli();
    let db_command = app.find_subcommand("db").unwrap();
    let query_command = db_command.find_subcommand("query").unwrap();
    
    // Verify the query command exists and has the right structure
    assert_eq!(query_command.get_name(), "query");
    
    // Check that it has the QUERY argument
    let args: Vec<_> = query_command.get_arguments().collect();
    assert_eq!(args.len(), 1);
    assert_eq!(args[0].get_id(), "QUERY");
}