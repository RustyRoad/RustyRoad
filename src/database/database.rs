// Database options

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
                _ => DatabaseType::Postgres,
            },
        }
    }
}
#[derive(Debug)]
pub enum DatabaseType {
    Postgres,
    Mysql,
    Sqlite,
    Mongo,
}
