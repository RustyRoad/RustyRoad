use crate::{database::Database, writers::write_to_file, Project};
use bcrypt::{hash as new_hash, DEFAULT_COST};

async fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    new_hash(password, DEFAULT_COST)
}


pub async fn load_sql_for_new_project(
    project: &Project,
    database: Database,
) -> Result<Vec<String>, std::io::Error> {
    let mut statements = Vec::new();
    // Hash the admin password
    let hashed_admin_password = match hash_password("admin").await {
        Ok(hashed_password) => hashed_password,
        Err(err) => {
            eprintln!("Failed to hash admin password: {:?}", err);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to hash admin password"));
        }
    };

    match database.database_type {
        crate::database::DatabaseType::Sqlite => {
            // create the users table
            statements.push(
                "CREATE TABLE IF NOT EXISTS Users (
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
                "CREATE TABLE IF NOT EXISTS Roles (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    name TEXT NOT NULL UNIQUE
                );"
                .to_string(),
            );

            // create the permissions table
            statements.push(
                "CREATE TABLE IF NOT EXISTS Permissions (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    name TEXT NOT NULL UNIQUE,
                    role_id INTEGER NOT NULL,
                    FOREIGN KEY (role_id) REFERENCES Roles(id)
                );"
                .to_string(),
            );

            // create the sessions table
            statements.push(
                "CREATE TABLE IF NOT EXISTS Sessions (
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
            statements.push(format!(
                "INSERT OR IGNORE INTO Roles (name) VALUES ('{}');",
                "admin"
            ));

            // create default permissions
            statements.push(format!(
                "INSERT OR IGNORE INTO Permissions (name, role_id) VALUES ('{}', 1);",
                "create_user"
            ));

            // create default permissions
            statements.push(format!(
                "INSERT OR IGNORE INTO Permissions (name, role_id) VALUES ('{}', 1);",
                "read_user"
            ));

            // add admin user
            statements.push(format!(
                    "INSERT OR IGNORE INTO Users (password, username, role_id) VALUES ('{hashed_admin_password}', 'admin', 1);"
                ));


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
                format!(
                    "INSERT INTO Users (password, username, role_id) VALUES ('{hashed_admin_password}', 'admin', 1);"
                )
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
            // create the roles table
            statements.push(
                "
CREATE TABLE Roles (
    id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE
);"
                .to_string(),
            );

            // create the users table
            statements.push(
                "
CREATE TABLE Users (
    id INT AUTO_INCREMENT PRIMARY KEY,
    password VARCHAR(255) NOT NULL,
    username VARCHAR(255) NOT NULL UNIQUE,
    email VARCHAR(255) UNIQUE,
    role_id INT,
    FOREIGN KEY (role_id) REFERENCES Roles(id)
);"
                .to_string(),
            );

            // create the permissions table
            statements.push(
                "
CREATE TABLE Permissions (
    id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE,
    role_id INT NOT NULL,
    FOREIGN KEY (role_id) REFERENCES Roles(id)
);"
                .to_string(),
            );

            // create the sessions table
            statements.push(
                "
CREATE TABLE Sessions (
    id INT AUTO_INCREMENT PRIMARY KEY,
    user_id INT NOT NULL,
    session_token VARCHAR(255) NOT NULL UNIQUE,
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
                format!(
                    "INSERT INTO Users (password, username, role_id) VALUES ('{hashed_admin_password}', 'admin', 1);"
                )
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
