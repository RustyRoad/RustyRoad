use mysql_async::{Conn, Opts};
use sqlx::Connection;
use std::error::Error;
use std::fs;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio_postgres::{Client, Config, Connection as Postgres_Connection, Error as PG_Connection_Error, NoTls};
use tokio_postgres::tls::NoTlsStream;
use toml::Value;

#[derive(Debug)]
pub struct Database {
    pub name: String,
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: String,
    pub database_type: DatabaseType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DatabaseType {
    Postgres,
    Mysql,
    Sqlite,
    Mongo,
}


#[derive(Debug, Clone)]
pub enum DatabaseConnection {
    Pg(PgConnection),
    MySql(MySqlConnection),
    Sqlite(SqliteConnection),
}

#[derive(Debug, Clone)]
pub struct PgConnection(pub Arc<Client>);
#[derive(Debug, Clone)]
pub struct MySqlConnection(pub Arc<Conn>);
#[derive(Debug, Clone)]
pub struct SqliteConnection(pub Arc<sqlx::SqliteConnection>);

impl Database {
    pub fn new(
        name: String,
        username: String,
        password: String,
        host: String,
        port: String,
        database_type: &str,
    ) -> Database {
        Database {
            name,
            username,
            password,
            host,
            port,
            database_type: match database_type {
                "postgres" => DatabaseType::Postgres,
                "mysql" => DatabaseType::Mysql,
                "sqlite" => DatabaseType::Sqlite,
                "mongo" => DatabaseType::Mongo,
                _ => DatabaseType::Postgres,
            },
        }
    }

    pub fn create_database_connection_string(&self) -> String {
        match &self.database_type {
            DatabaseType::Mysql => {
                let port = self.port.parse::<u16>().unwrap_or_else(|_| 3306);
                format!(
                    "mysql://{}:{}@{}:{}/{}",
                    self.username, self.password, self.host, port, self.name
                )
            }
            DatabaseType::Sqlite => {
                format!("sqlite://{}.db", self.name)
            }
            DatabaseType::Mongo => {
                todo!("MongoDB is not yet supported");
            }
            DatabaseType::Postgres => {
                let port = self.port.parse::<u16>().unwrap_or_else(|_| 5432);
                format!(
                    "postgres://{}:{}@{}:{}/{}",
                    self.username, self.password, self.host, port, self.name
                )
            }
        }
    }

    async fn connect_raw(s: &str) -> Result<(Client, Postgres_Connection<TcpStream, NoTlsStream>), PG_Connection_Error> {
        let socket = TcpStream::connect("127.0.0.1:5433").await.unwrap();
        let config = s.parse::<Config>().unwrap();
        config.connect_raw(socket, NoTls).await
    }

    async fn connect(s: &str) -> Client {
        let (client, connection) = Self::connect_raw(s).await.unwrap();
        tokio::spawn(connection);
        client
    }

    pub async fn create_database_connection(&self) -> Result<DatabaseConnection, Box<dyn Error + Send>> {
        match &self.database_type {
            DatabaseType::Mysql => {
                let connection_url = self.create_database_connection_string();
                let connection_options = Opts::from_url(&connection_url).expect("Database URL is invalid");
                let connection = Conn::new(connection_options).await.expect("Failed to connect to MySQL");
                Ok(DatabaseConnection::MySql(MySqlConnection(Arc::new(connection))))
            }
            DatabaseType::Sqlite => {
                let connection_url = self.create_database_connection_string();
                let connection = sqlx::SqliteConnection::connect(&connection_url).await.expect("Failed to connect to SQLite");
                Ok(DatabaseConnection::Sqlite(SqliteConnection(Arc::new(connection))))
            }
            DatabaseType::Postgres => {
                let connection_url = self.create_database_connection_string();
                let client = Self::connect(&connection_url);
                Ok(
                    DatabaseConnection::Pg(
                        PgConnection(Arc::new(client.await))
                    )
                )
            }
            DatabaseType::Mongo => {
                todo!("MongoDB is not yet supported");
            }
        }
    }

    pub async fn get_database_from_rustyroad_toml() -> Result<Database, std::io::Error> {
        let database: Database = match fs::read_to_string("rustyroad.toml") {
            Ok(file) => {
                let toml: Value = toml::from_str(&file).unwrap();

                let database_table = toml["database"].as_table().unwrap();
                Database::new(
                    database_table["database_name"]
                        .as_str()
                        .unwrap()
                        .to_string(),
                    database_table["database_user"]
                        .as_str()
                        .unwrap()
                        .to_string(),
                    database_table["database_password"]
                        .as_str()
                        .unwrap()
                        .to_string(),
                    database_table["database_host"]
                        .as_str()
                        .unwrap()
                        .to_string(),
                    database_table["database_port"]
                        .as_str()
                        .unwrap()
                        .to_string(),
                    &database_table["database_type"]
                        .as_str()
                        .unwrap(),

                )
            }
            Err(_) => {
                eprintln!("Error: Could not find rustyroad.toml");
                std::process::exit(1);
            }
        };

        Ok(database)
    }
}

impl DatabaseConnection {
    pub fn get_database_type(&self) -> DatabaseType {
        match self {
            DatabaseConnection::Pg(_) => DatabaseType::Postgres,
            DatabaseConnection::MySql(_) => DatabaseType::Mysql,
            DatabaseConnection::Sqlite(_) => DatabaseType::Sqlite,
        }
    }
}
