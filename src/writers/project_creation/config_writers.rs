use crate::database::{Database, DatabaseType};
use crate::Project;
use std::fs;
use std::io::Error;

/// This function writes the dependencies for the project to the Cargo.toml file.
///
/// # Arguments
///
/// * `project` - A Project struct containing information about the project.
/// * `database_data` - A Database struct containing information about the database to be used in the project.
///
/// # Returns
///
/// * `Ok(())` if the content was successfully written to the file, or an Error if something went wrong.
pub fn write_to_cargo_toml(project: &Project, database_data: &Database) -> Result<(), Error> {
    let dependencies = match database_data.database_type {
        DatabaseType::Postgres => {
            r#"[dependencies.sqlx]
features = ["postgres", "macros", "chrono", "json", "uuid", "offline"]
version = "0.6.2""#
        }
        DatabaseType::Mysql => {
            r#"[dependencies.sqlx]
features = ["mysql", "macros", "chrono", "json", "uuid", "offline"]
version = "0.6.2""#
        }
        DatabaseType::Sqlite => {
            r#"[dependencies.rusqlite]
version = "0.28.0"
features = ["bundled"]"#
        }
        DatabaseType::Mongo => {
            r#"[dependencies.mongodb]
version =  "2.4.0"
default-features = false
features = ["sync", "bson", "tls"]"#
        }
    };

    let config = format!(
        r#"[package]
name = "{}"
version = "0.1.0"
authors = ["RustyRoad"]
edition = "2021"

[dependencies]
actix-cors = "0.6.4"
actix-web = "4.0.0-beta.8"
actix-files = "0.6.2"
actix-session = {{version = "0.7.2", features = ["cookie-session"]}}
actix-identity = "0.5.2"
tokio = {{ version = "1", features = ["macros", "rt-multi-thread"] }}
serde = {{ version = "1.0", features = ["derive"] }}
serde_json = "1.0.82"
random-string = "1.0.0"
env_logger = "0.10.0"
local-ip-address = "0.5.0"
futures = "0.3.23"
tera = "1.17.1"
reqwest = "0.11"
rustyroad = "0.1.7"
rand = "0.8.5"
chrono = "0.4.24"
base64 = "0.21.0"
dotenv = "0.15.0"
bcrypt = "0.14.0"
color-eyre = "0.6.2"
{}
"#,
        &project.name, dependencies
    );

    fs::write(&project.cargo_toml, config.as_bytes())
}
