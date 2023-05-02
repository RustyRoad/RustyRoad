// Add this import at the top of the `database.rs` file

use std::fs;
use toml::Value;

#[derive(Debug, Clone, PartialEq, std::cmp::Eq)]
pub struct Database {
    pub name: String,
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: String,
    pub database_type: DatabaseType,
}
/// # Name: Database
/// ## Type: Struct
/// ## Description
/// This struct is used to configure the database.
/// This is used when creating a new project and the u
/// Example:
/// ```rust
/// use crate::database::Database;
/// let database = Database::new(
///    "database_name".to_string(),
///   "username".to_string(),
///  "password".to_string(),
/// "host".to_string(),
/// "port".to_string(),
/// "database_type".to_string(),
/// );
/// ```
///
impl Database {
    pub fn new(
        name: String,
        username: String,
        password: String,
        host: String,
        port: String,
        database_type: String,
    ) -> Database {
        Database {
            name,
            username,
            password,
            host,
            port,
            database_type: match database_type.as_str() {
                "postgres" => DatabaseType::Postgres,
                "mysql" => DatabaseType::Mysql,
                "sqlite" => DatabaseType::Sqlite,
                "mongo" => DatabaseType::Mongo,
                // this is defaulting, need to address the code running this line
                _ => DatabaseType::Postgres,
            },
        }
    }

    /// Reads the `rustyroad.toml` configuration file and extracts the database configuration.
    /// Returns a `Database` struct containing the database configuration.
    ///
    /// # Returns
    ///
    /// * `Ok(Database)` - If the `rustyroad.toml` file is found and successfully parsed, returns a `Database` struct.
    /// * `Err(std::io::Error)` - If there is an error reading the `rustyroad.toml` file, returns an I/O error.
    ///
    /// # Panics
    ///
    /// * If the `rustyroad.toml` file is not found or cannot be parsed, the function will print an error message
    ///   and exit the process with status code 1.
    pub fn get_database_from_rustyroad_toml() -> Result<Database, std::io::Error> {
        let database = match fs::read_to_string("rustyroad.toml") {
            Ok(file) => {
                let toml: Value = toml::from_str(&file).unwrap();

                // Access the [database] table from the TOML document
                let database_table = toml["database"].as_table().unwrap();

                // Access the keys within the [database] table
                let name = database_table["database_name"].as_str().unwrap().to_string();
                let username = database_table["database_user"].as_str().unwrap().to_string();
                let password = database_table["database_password"].as_str().unwrap().to_string();
                let host = database_table["database_host"].as_str().unwrap().to_string();
                let port = database_table["database_port"].as_str().unwrap().to_string();
                let database_type = database_table["database_type"].as_str().unwrap().to_string();

                Database::new(name, username, password, host, port, database_type)
            }
            Err(_) => {
                println!("No rustyroad.toml file found in the workspace root.");
                println!("Please run `rustyroad new` to create a new project.");
                std::process::exit(1);
            }
        };
        Ok(database)
    }


}

#[derive(Debug, Clone, PartialEq, std::cmp::Eq)]
pub enum DatabaseType {
    Postgres,
    Mysql,
    Sqlite,
    Mongo,
}
