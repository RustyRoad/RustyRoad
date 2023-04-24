use std::env;

use sqlx::{Sqlite, SqlitePool};

use crate::{database::Database, writers::write_to_file, Project};

pub async fn load_sql_for_new_project(
    project: &Project,
    database: Database,
) -> Result<Vec<String>, std::io::Error> {
    let mut statements = Vec::new();

    match database.database_type {
        crate::database::DatabaseType::Sqlite => {
            std::process::Command::new("export DATABASE_URL={}")
                .arg(&project.config_dev_db)
                .spawn()
                .expect("Failed to set DATABASE_URL");
            // create the users table
            statements.push(
                "CREATE TABLE Users (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    password TEXT NOT NULL,
                    username TEXT NOT NULL UNIQUE,
                    role_id INTEGER,
                    FOREIGN KEY (role_id) REFERENCES Roles(id)
                );"
                .to_string(),
            );

            // create the roles table
            statements.push(
                "CREATE TABLE Roles (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    name TEXT NOT NULL UNIQUE
                );"
                .to_string(),
            );

            // create the permissions table
            statements.push(
                "CREATE TABLE Permissions (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    name TEXT NOT NULL UNIQUE,
                    role_id INTEGER NOT NULL,
                    FOREIGN KEY (role_id) REFERENCES Roles(id)
                );"
                .to_string(),
            );

            // create the sessions table
            statements.push(
                "CREATE TABLE Sessions (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    user_id INTEGER NOT NULL,
                    session_token TEXT NOT NULL UNIQUE,
                    session_data TEXT,
                    expiration_date DATETIME,
                    FOREIGN KEY (user_id) REFERENCES Users(id)
                );"
                .to_string(),
            );

            // add admin role
            statements.push(format!("INSERT INTO Roles (name) VALUES ('{}');", "admin"));

            // create default permissions
            statements.push(format!(
                "INSERT INTO Permissions (name, role_id) VALUES ('{}', 1);",
                "create_user"
            ));

            // create default permissions
            statements.push(format!(
                "INSERT INTO Permissions (name, role_id) VALUES ('{}', 1);",
                "read_user"
            ));

            // add admin user
            statements.push(
                "INSERT INTO Users (password, username, role_id) VALUES ('admin', 'admin', 1);"
                    .to_string(),
            );

            // Execute the migration
            if let Err(e) = execute_sqlite_migration(&project, statements.clone()).await {
                panic!("Failed to execute migration: {:?}", e);
            }

            // create the down migration
            let mut down_statements = Vec::new();
            down_statements.push(format!("DROP DATABASE {};", project.name));

            // write the down migration to the file
            for (idx, statement) in down_statements.iter().enumerate() {
                write_to_file(
                    &format!("{}", &project.initial_migration_down),
                    statement.as_bytes(),
                )
                .unwrap_or_else(|why| {
                    panic!(
                        "Failed to write to file {}_{}: {:?}",
                        &project.initial_migration_down,
                        idx,
                        why.kind()
                    )
                });
            }
        }
        crate::database::DatabaseType::Postgres => {
            // create the roles table
            statements.push(
                "
CREATE TABLE Roles (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
);"
                .to_string(),
            );

            // create the users table
            statements.push(
                "
CREATE TABLE Users (
    id SERIAL PRIMARY KEY,
    password TEXT NOT NULL,
    username TEXT NOT NULL UNIQUE,
    role_id INTEGER,
    FOREIGN KEY (role_id) REFERENCES Roles(id)
);"
                .to_string(),
            );

            // create the permissions table
            statements.push(
                "
CREATE TABLE Permissions (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    role_id INTEGER NOT NULL,
    FOREIGN KEY (role_id) REFERENCES Roles(id)
);"
                .to_string(),
            );

            // create the sessions table
            statements.push(
                "
CREATE TABLE Sessions (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    session_token TEXT NOT NULL UNIQUE,
    session_data TEXT,
    expiration_date TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES Users(id)
);"
                .to_string(),
            );

            // add admin role
            statements.push(format!("INSERT INTO Roles (name) VALUES ('{}');", "admin"));

            // create default permissions
            statements.push(format!(
                "INSERT INTO Permissions (name, role_id) VALUES ('{}', 1);",
                "create_user"
            ));

            // create default permissions
            statements.push(format!(
                "INSERT INTO Permissions (name, role_id) VALUES ('{}', 1);",
                "read_user"
            ));

            // add admin user
            statements.push(
                "INSERT INTO Users (password, username, role_id) VALUES ('admin', 'admin', 1);"
                    .to_string(),
            );

            // write the template to the file
            for (idx, statement) in statements.iter().enumerate() {
                write_to_file(
                    &format!("{}", &project.initial_migration_up),
                    statement.as_bytes(),
                )
                .unwrap_or_else(|why| {
                    panic!(
                        "Failed to write to file {}_{}: {:?}",
                        &project.initial_migration_up,
                        idx,
                        why.kind()
                    )
                });
            }

            // create the down migration
            let mut down_statements = Vec::new();
            down_statements.push(format!("DROP SCHEMA public CASCADE;"));
            down_statements.push(format!("CREATE SCHEMA public;"));

            // write the down migration to the file
            for (idx, statement) in down_statements.iter().enumerate() {
                write_to_file(
                    &format!("{}", &project.initial_migration_down),
                    statement.as_bytes(),
                )
                .unwrap_or_else(|why| {
                    panic!(
                        "Failed to write to file {}_{}: {:?}",
                        &project.initial_migration_down,
                        idx,
                        why.kind()
                    )
                });
            }
        }
        crate::database::DatabaseType::Mysql => {
            // create the users table
            statements.push(
                "
    CREATE TABLE Users (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        password TEXT NOT NULL,
        username TEXT NOT NULL UNIQUE,
        role_id INTEGER,
        FOREIGN KEY (role_id) REFERENCES Roles(id)
    );"
                .to_string(),
            );

            // create the roles table
            statements.push(
                "
    CREATE TABLE Roles (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL UNIQUE
    );"
                .to_string(),
            );

            // create the permissions table
            statements.push(
                "
    CREATE TABLE Permissions (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL UNIQUE,
        role_id INTEGER NOT NULL,
        FOREIGN KEY (role_id) REFERENCES Roles(id)
    );"
                .to_string(),
            );

            // create the sessions table
            statements.push(
                "
    CREATE TABLE Sessions (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        user_id INTEGER NOT NULL,
        session_token TEXT NOT NULL UNIQUE,
        session_data TEXT,
        expiration_date DATETIME,
        FOREIGN KEY (user_id) REFERENCES Users(id)
    );"
                .to_string(),
            );

            // add admin role
            statements.push(format!("INSERT INTO Roles (name) VALUES ('{}');", "admin"));

            // create default permissions
            statements.push(format!(
                "INSERT INTO Permissions (name, role_id) VALUES ('{}', 1);",
                "create_user"
            ));

            // create default permissions
            statements.push(format!(
                "INSERT INTO Permissions (name, role_id) VALUES ('{}', 1);",
                "read_user"
            ));

            // add admin user
            statements.push(
                "INSERT INTO Users (password, username, role_id) VALUES ('admin', 'admin', 1);"
                    .to_string(),
            );

            // write the template to the file
            for (idx, statement) in statements.iter().enumerate() {
                write_to_file(
                    &format!("{}", &project.initial_migration_up),
                    statement.as_bytes(),
                )
                .unwrap_or_else(|why| {
                    panic!(
                        "Failed to write to file {}_{}: {:?}",
                        &project.initial_migration_up,
                        idx,
                        why.kind()
                    )
                });
            }

            // create the down migration
            let mut down_statements = Vec::new();
            down_statements.push(format!("DROP DATABASE {};", project.name));

            // write the down migration to the file
            for (idx, statement) in down_statements.iter().enumerate() {
                write_to_file(
                    &format!("{}", &project.initial_migration_down),
                    statement.as_bytes(),
                )
                .unwrap_or_else(|why| {
                    panic!(
                        "Failed to write to file {}_{}: {:?}",
                        &project.initial_migration_down,
                        idx,
                        why.kind()
                    )
                });
            }
        }
        crate::database::DatabaseType::Mongo => todo!(),
    }

    Ok(statements)
}

pub async fn execute_sqlite_migration(
    project: &Project,
    statements: Vec<String>,
) -> Result<(), sqlx::Error> {
    // Set the DATABASE_URL environment variable
    env::set_var("DATABASE_URL", &project.config_dev_db);

    // Create a connection pool to the SQLite database
    let pool = SqlitePool::connect(&project.config_dev_db).await?;

    // Execute each SQL statement
    for statement in statements {
        sqlx::query::<Sqlite>(&statement).execute(&pool).await?;
    }

    Ok(())
}
