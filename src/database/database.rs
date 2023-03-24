// Add this import at the top of the `database.rs` file

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
}

#[derive(Debug, Clone, PartialEq, std::cmp::Eq)]
pub enum DatabaseType {
    Postgres,
    Mysql,
    Sqlite,
    Mongo,
}
