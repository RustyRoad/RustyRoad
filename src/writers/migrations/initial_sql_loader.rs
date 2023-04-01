use crate::{Project, writers::write_to_file};

pub fn load_sql_for_new_project(project: &Project) -> Result<(), std::io::Error> {
    let mut template = String::new();

    template.push_str(&format!(
        "CREATE DATABASE {} DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;",
        project.name
    ));

    // create the users table
    template.push_str(&format!(
        "
        CREATE TABLE Users (
	      id INTEGER PRIMARY KEY AUTOINCREMENT,
	      password TEXT NOT NULL,
	      username TEXT NOT NULL UNIQUE,
	      role_id INTEGER,
	      FOREIGN KEY (role_id) REFERENCES Roles(id)
        );"
    ));

    // create the roles table
    template.push_str(&format!(
        "
        CREATE TABLE Roles (
	      id INTEGER PRIMARY KEY AUTOINCREMENT,
	      name TEXT NOT NULL UNIQUE
        );"
    ));

    // create the permissions table
    template.push_str(&format!(
        "
        CREATE TABLE Permissions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
	          name TEXT NOT NULL UNIQUE,
	          role_id INTEGER NOT NULL,
	          FOREIGN KEY (role_id) REFERENCES Roles(id)
        )"
    ));

    // create the sessions table
    template.push_str(&format!(
        "
        CREATE TABLE Sessions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
	          user_id INTEGER NOT NULL,
	          session_token TEXT NOT NULL UNIQUE,
	          session_data TEXT,
	          expiration_date DATETIME,
	          FOREIGN KEY (user_id) REFERENCES Users(id)
        )"
    ));

    // add admin role
    template.push_str(&format!(
        "
        INSERT INTO Roles (name) VALUES ('{}');",
        "admin"
    ));

    // create default permissions
    template.push_str(&format!(
        "
        INSERT INTO Permissions (name, role_id) VALUES ('{}', 1);",
        "create_user"
    ));
    // create default permissions
    template.push_str(&format!(
        "
        INSERT INTO Permissions (name, role_id) VALUES ('{}', 1);",
        "read_user"
    ));

    // add admin user
    template.push_str(&format!(
        "
        INSERT INTO Users (password, username, role_id) VALUES ('admin', 'admin', 1);"
    ));

    // write the template to the file
    write_to_file(&format!("{}/initial.sql", project.config_initializers_db), template.as_bytes())
}