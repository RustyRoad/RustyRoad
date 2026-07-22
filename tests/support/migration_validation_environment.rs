use std::fs;
use std::path::Path;
use std::process::{Command, Output};

pub fn project(root: &Path, config: &str, database: &str) {
    fs::write(
        root.join(config),
        format!(
            "[database]\ndatabase_name = \"{database}\"\ndatabase_user = \"\"\n\
             database_password = \"\"\ndatabase_host = \"\"\ndatabase_port = \"0\"\n\
             database_type = \"sqlite\"\n"
        ),
    )
    .unwrap();
    let migration = root.join("config/database/migrations/20260101-create_users");
    fs::create_dir_all(&migration).unwrap();
    fs::write(
        migration.join("up.sql"),
        "CREATE TABLE users (id INTEGER PRIMARY KEY);",
    )
    .unwrap();
}

pub fn validate(root: &Path, environment: Option<&str>) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_rustyroad"));
    command
        .current_dir(root)
        .env_remove("ENV")
        .env_remove("ENVIRONMENT")
        .args(["migration", "validate"]);
    if let Some(environment) = environment {
        command.env("ENVIRONMENT", environment);
    }
    command.output().unwrap()
}
