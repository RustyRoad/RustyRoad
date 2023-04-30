//! # Rusty Road
//! Rusty Road is a framework written in Rust that is based on Ruby on Rails. It is designed to provide the familiar conventions and ease of use of Ruby on Rails, while also taking advantage of the performance and efficiency of Rust.
//! Below you will find a struct that represents a project.  It is used to create a new project.
//! ## Description
//! Rusty Road is a CLI tool that is used to create and manage your rust web apps.
//! You can use this package as a part of your project and this documentation will help you understand how to use it, however, it is not intended to be used as a standalone package.
//! ## Example
//! ```
//! use rustyroad::Project;
//!
//! fn main() {
//!    Project::initial_prompt().expect("Failed to create project");
//! }
//! ```
//!
//! ### Code Explanation
//! The code above is the main function for the RustyRoad project.  It is the entry point for the program.
//! The project is created by calling the `initial_prompt` function on the `Project` struct.
//! The initial prompt function will ask the user a series of questions and then create a new project based on the answers.
//! From there, the user can use the project to create a new web app.
//! Notice that other functions are called on the `Project` struct.  These functions are used to create a new web app.
//! These are the functions that ship with the cli tool and are not publicly available.

#![deny(warnings)]
#![allow(dead_code)]

use clap::{arg, Arg, Command, Parser};
use serde::Deserialize;
use sqlx::mysql::MySqlConnectOptions;
use sqlx::postgres::PgConnectOptions;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::ConnectOptions;

use std::fs::create_dir;
use std::io::Error;
use std::{env, fs};
use std::{fs::OpenOptions, io::Write};
pub mod database;
pub mod generators;
use database::{create_migration, Database, DatabaseType};
pub mod writers;

use crate::generators::create_directory;
use crate::writers::{
    migrations::initial_sql_loader, project_creation::create_file::create_files,
};
/**
 * # Struct RustyRoad
 * ## Description
 * This struct is used to configure the project.
 * This is specfically used to read the rustyroad.toml file and
 * and decode the toml into a struct.
 */
#[derive(Debug, Deserialize)]
pub struct RustyRoad {
    name: String,
}
use crate::generators::create_file;
use crate::writers::templates::{write_to_base_html, write_to_header, navbar::write_to_navbar};
use crate::writers::{
    add_new_route_to_main_rs, create_database_if_not_exists, write_to_file, write_to_main_rs,
    write_to_route_name_html, write_to_route_name_rs, write_to_routes_mod, project_creation::*
};

// timestamp

// get time

/// # Name: Project
/// ## Type: Struct
/// ## Description
/// This struct is used to configure the project.
/// This is specfically used to read the rustyroad.toml file and
/// and decode the toml into a struct.
/// ## Example
/// ```
/// use rustyroad::Project;
///
/// fn main() {
///   Project::run();
/// }
///
/// ```
/// ### Code Explanation
/// The code above is the main function for the RustyRoad project.  It is the entry point for the program.
/// The project is created by calling the `initial_prompt` function on the `Project` struct.
/// The initial prompt function will ask the user a series of questions and then create a new project based on the answers.
/// From there, the user can use the project to create a new web app.
/// Notice that other functions are called on the `Project` struct.  These functions are used to create a new web app.
/// These are the functions that ship with the cli tool and are not publicly available.
#[derive(Parser, Debug, Clone)]
pub struct Project {
    pub name: String,
    pub rocket_toml: String,
    pub rustyroad_toml: String,
    pub src_dir: String,
    pub main_rs: String,
    pub cargo_toml: String,
    pub package_json: String,
    pub readme: String,
    pub gitignore: String,
    pub templates: String,
    pub static_dir: String,
    pub template_components: String,
    pub template_sections: String,
    pub template_layouts: String,
    pub template_pages: String,
    pub static_css: String,
    pub static_js: String,
    pub index_js: String,
    pub static_images: String,
    pub config: String,
    pub config_env: String,
    pub config_dev_env: String,
    pub config_prod_env: String,
    pub config_test_env: String,
    pub config_default_env: String,
    pub db: String,
    pub config_dev_db: String,
    pub config_prod_db: String,
    pub config_test_db: String,
    pub routes: String,
    pub routes_module: String,
    pub controllers: String,
    pub models: String,
    pub models_module: String,
    pub migrations: String,
    pub seeders: String,
    pub tests: String,
    pub config_initializers: String,
    pub config_initializers_assets: String,
    pub config_initializers_db: String,
    pub config_initializers_default: String,
    pub config_initializers_middleware: String,
    pub config_initializers_routes: String,
    pub index_html: String,
    pub base_html: String,
    pub tailwind_css: String,
    pub tailwind_config: String,
    pub postcss_config: String,
    pub not_found_html: String,
    pub server_error_html: String,
    pub favicon_ico: String,
    pub robots_txt: String,
    pub login_page_html: String,
    pub signup_page_html: String,
    pub reset_password_page_html: String,
    pub forgot_password_page_html: String,
    pub dashboard_page_html: String,
    pub user_controller_directory: String,
    pub user_controller: String,
    pub user_controller_module: String,
    pub user_model_directory: String,
    pub user_model: String,
    pub initial_migration_directory: String,
    pub initial_migration_up: String,
    pub initial_migration_down: String,
    pub user_test: String,
    pub user_route: String,
    pub index_route: String,
    pub login_route: String,
    pub signup_route: String,
    pub reset_password_route: String,
    pub forgot_password_route: String,
    pub dashboard_route: String,
    pub navbar_component: String,
    pub header_section: String,
}

/// # RustyRoad Project Builder
/// Description: This is the main file for the RustyRoad project.
/// It is the entry point for the program.
///
/// ## Usage
///
/// ```rust
/// use rustyroad::Project;
///
/// let project = Project::new("MyProject".to_string());
/// ```
///
///
impl Project {
    /// # Create a new project
    /// This function creates a new project.
    /// It is called from within the CreateProject function.
    /// Takes a name and a path as arguments
    /// The {name} is the name of the project.
    /// The {path} is the path to the directory where the project will be created.
    /// If a path is not provided, the project will be created in the current directory.

    // Write to rustyroad_toml
    pub fn write_to_rustyroad_toml(&self, database_data: &Database) -> Result<(), Error> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.rustyroad_toml)?;

        let config = format!(
            "name = \"{}\"
            database_name = \"{}\"
            database_user = \"{}\"
            database_password = \"{}\"
            database_host = \"{}\"
            database_port = \"{}\"
            database_type = \"{:?}\"",
            self.name,
            database_data.clone().name,
            database_data.username,
            database_data.password,
            database_data.host,
            database_data.port,
            database_data.database_type
        );
        file.write_all(config.as_bytes())?;
        Ok(())
    }
  // Write to rocket_toml
    fn write_to_rocket_toml(&self) -> Result<(), Error> {
        let config = format!(
            "[global.databases]
        default = {{ url = \"sqlite:./db/{}.db\" }}
        temp_dir = \"./src/
        ",
            &self.name
        );

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.rocket_toml)?;

        file.write_all(config.as_bytes())?;
        Ok(())
    }

    // Write to package.json
    fn write_to_package_json(&self) -> Result<(), Error> {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(&self.package_json)
            .expect("Failed to open package.json");

        file.write_all(
            "{{
  \"name\": \"rustyroad\",
  \"version\": \"1.0.0\",
  \"main\": \"index.js\",
  \"repository\": \"https://github.com/Riley-Seaburg/RustyRoad.git\",
  \"author\": \"Riley Seaburg <riley@rileyseaburg.com>\",
  \"license\": \"MIT\",
  \"scripts\": {{
    \"server\": \"cargo run\",
    \"tailwind:dev\": \"npx tailwindcss -i ./src/tailwind.css -o ./static/styles.css --watch\",
    \"tailwind:build\": \"npx tailwindcss -i ./src/tailwind.css -o ./static/styles.css --minify\",
    \"dev\": \"concurrently \\\"yarn tailwind:dev\\\" \\\" yarn server\\\"\"
  }},
  \"devDependencies\": {{
    \"@tailwindcss/forms\": \"^0.5.3\",
    \"concurrently\": \"^7.6.0\",
    \"tailwindcss\": \"^3.2.4\"
  }}
}}"
            .as_bytes(),
        )
        .expect("Failed to write to package.json");
        Ok(())
    }
    // Write to README.md
    fn write_to_readme(&self) -> Result<(), Error> {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(&self.readme)
            .expect("Failed to open README.md");

        file.write_all(
            format!(
                "# {}
This project was created using Rusty Road. Rusty Road is Rails for Rust. It is a CLI tool that allows you to create a new Rust project with a few commands. It also comes with TailwindCSS and Rocket pre-installed.

## Getting Started

### Configure TailwindCSS

```bash
npx tailwindcss init -p
```

To get started, run `yarn dev` to start the server and watch for changes to your TailwindCSS files.

## Contributing

If you would like to contribute to this project, please fork the repository and submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details."
                , self.name
            )
                .as_bytes(),
        )
            .expect("Failed to write to README.md");

        Ok(())
    }
    // Write to .gitignore
    fn write_to_gitignore(&self) -> Result<(), Error> {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(&self.gitignore)
            .expect("Failed to open .gitignore");

        file.write_all(
            b"target/
Cargo.lock
.DS_Store
.env
.db
node_modules/
static/styles.css
",
        )
        .expect("Failed to write to .gitignore");

        Ok(())
    }

    // Write to models/users.rs
    fn write_to_user_models(&self) -> Result<(), Error> {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(&self.user_model)?;
        file.write_all(
            b"use rocket::http::{Cookie, CookieJar};
use rocket::response::{Flash, Redirect};
use rocket_db_pools::{sqlx, Connection, Database};
use rocket_dyn_templates::{context, Template};

// let conn = SqliteConnection::connect('sqlite::memory:').await?;

pub struct User {
    id: i32,
    username: String,
    password: String,
    email: String,
    created_at: String,
    updated_at: String,
}
#[derive(FromForm, Debug)]

pub struct UserLogin {
    username: String,
    password: String,
}

impl UserLogin {
    pub fn user_login(&self) -> Result<Template, Flash<Redirect>> {
        if self.username == 'root' && self.password == 'root' {
            Ok(Template::render(
                'welcome',
                context! {
                    username: &self.username,
                },
            ))
        } else {
            Err(Flash::error(
                Redirect::to('/'),
                'Invalid username or password',
            ))
        }
    }
}",
        )?;

        Ok(())
    }

    // Write to index.js
    fn write_to_index_js(&self) -> Result<(), Error> {
        let contents = format!(
            "// Rusty Road
        class RustyRoad {{
            constructor() {{
                this.name = \"{}\";
        function greet() {{
            console.log(\"Welcome to {} powered by Rusty Road\");
        }}
            }}
        }}

        const rusty-road = new RustyRoad();

        rusty-road.greet();
        ",
            self.name, self.name
        );

        write_to_file(&self.index_js, contents.as_bytes())?;

        Ok(())
    }
    // Write to dev.env
    fn write_to_dev_dot_env(&self) -> Result<(), Error> {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(&self.config_dev_env)
            .expect("Failed to open dev.env");
        file.write_all(
            b"ROCKET_ENV=dev
            ROCKET_ADDRESS=
            ROCKET_PORT=8000
            ROCKET_LOG=normal
            ROCKET_WORKERS=1
            ROCKET_SECRET_KEY=
            ROCKET_TEMPLATES=
            ROCKET_DATABASES=
            ROCKET_TLS=
            ROCKET_TLS_CERTS=
",
        )
        .expect("Failed to write to dev.env");

        Ok(())
    }
    // Write to prod.env
    fn write_to_prod_dot_env(&self) -> Result<(), Error> {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(&self.config_prod_env)
            .expect("Failed to open prod.env");
        file.write_all(
            b"ROCKET_ENV=prod
            ROCKET_ADDRESS=
            ROCKET_PORT=8000
            ROCKET_LOG=normal
            ROCKET_WORKERS=1
            ROCKET_SECRET_KEY=
            ROCKET_TEMPLATES=
            ROCKET_DATABASES=
            ROCKET_TLS=
            ROCKET_TLS_CERTS=
",
        )
        .expect("Failed to write to prod.env");

        Ok(())
    }

    // Write to tailwind.css
    fn write_to_tailwind_css(&self) -> Result<(), Error> {
        let contents = "@tailwind base;
@tailwind components;
@tailwind utilities;";

        write_to_file(&self.tailwind_css.to_string(), contents.as_bytes()).unwrap_or_else(|why| {
            println!(
                "Couldn't write to {}: {}",
                self.tailwind_css.to_string(),
                why.to_string()
            );
        });
        Ok(())
    }
    // Write to tailwind.config.js
    fn write_to_tailwind_config(&self) -> Result<(), Error> {
        let contents = "module.exports = {
        darkMode: 'media',
        content: ['./templates/**/*.{html.tera,js}'],
        theme: {
            extend: {
            },
        },
        plugins: [
            require('@tailwindcss/forms'),
        ],
        };";

        write_to_file(&self.tailwind_config.to_string(), contents.as_bytes()).unwrap_or_else(
            |why| {
                println!(
                    "Couldn't write to {}: {}",
                    self.tailwind_config.to_string(),
                    why.to_string()
                );
            },
        );
        Ok(())
    }
    // Write to postcss.config.js
    fn write_to_postcss_config(&self) -> Result<(), Error> {
        let contents = "module.exports = {{
            plugins: [
                require('tailwindcss'),
                require('autoprefixer'),
            ],
        }};";

        write_to_file(&self.postcss_config.to_string(), contents.as_bytes()).unwrap_or_else(
            |why| {
                println!(
                    "Couldn't write to {}: {}",
                    self.postcss_config,
                    why.to_string()
                );
            },
        );
        Ok(())
    }


    /// Creates a new project
    /// Takes an optional name <String> and db_type <String>
    /// If no name is provided, it will default to "rustyroad"
    /// If a name is provided, it will create a new directory with that name
    /// and create a new project in that directory
    /// If a directory with the same name already exists, it will return an error
    /// and ask the user to choose a different name
    /// If a db_type is provided, it will create a new database with that type
    /// If no db_type is provided, it will default to "sqlite"
    /// If a db_type is provided that is not supported, it will return an error
    /// and ask the user to choose a different db_type
    /// Allow unused variables because the db_type is not used yet
    #[allow(unused_variables)]
    pub async fn create_new_project(
        name: String,
        database_data: Database,
    ) -> Result<Project, Error> {
        // If name is provided, create a new directory with that name
        // If no name is provided, run the rest of the code in the function
        // write the database data to the rustyroad.toml file

        // Create new project with name
        let mut project = new(name);

        // Create the project directory
        create_directory(&project).unwrap_or_else(|why| {
            println!("Couldn't create directory: {:?}", why.kind());
        });

        // Create the files
        create_files(&project).unwrap_or_else(|why| {
            panic!("Couldn't create files: {:?}", why.kind());
        });

        // Write to rustyroad.toml file
        Self::write_to_rustyroad_toml(&project, &database_data)
            .expect("Failed to write to rustyroad.toml");

        // Write to the cargo.toml file
        write_to_cargo_toml(&project, &database_data).expect("Failed to write to cargo.toml");

        // Write to main.rs file
        write_to_main_rs(&project).expect("Failed to write to main.rs");

        // Write to package.json file
        Self::write_to_package_json(&project).expect("Failed to write to package.json");

        // Write to README.md file
        Self::write_to_readme(&project).expect("Failed to write to README.md");

        // Write to index.js file
        Self::write_to_index_js(&project).unwrap_or_else(|why| {
            println!("Failed to write to index.js: {:?}", why.kind());
        });
        // Write to index.html.tera file
        write_to_index_html(&project).unwrap_or_else(|why| {
            println!("Failed to write to index.html: {:?}", why.kind());
        });
        // Write to base.html.tera file
        write_to_base_html(&project.base_html).unwrap_or_else(|why| {
            println!("Failed to write to base.html: {:?}", why.kind());
        });

        // Write to dev.env file
        Self::write_to_dev_dot_env(&project).unwrap_or_else(|why| {
            println!("Failed to write to dev.env: {:?}", why.kind());
        });

        // Write to prod.env file
        Self::write_to_prod_dot_env(&project).unwrap_or_else(|why| {
            println!("Failed to write to prod.env: {:?}", why.kind());
        });

        // Write to tailwind.css file
        Self::write_to_tailwind_css(&project).unwrap_or_else(|why| {
            println!("Failed to write to tailwind.css: {:?}", why.kind());
        });
        // need to create the function
        // Write to tailwind.config.js file
        Self::write_to_tailwind_config(&project).unwrap_or_else(|why| {
            println!("Failed to write to tailwind.config.js: {:?}", why.kind());
        });

        // Write to postcss.config.js file
        Self::write_to_postcss_config(&project).unwrap_or_else(|why| {
            println!("Failed to write to postcss.config.js: {:?}", why.kind());
        });

        // Write to index.html route
        write_to_index_route(&project).unwrap_or_else(|why| {
            println!("Failed to write to index.html: {:?}", why.kind());
        });

        // Write to gitignore file
        Self::write_to_gitignore(&project).unwrap_or_else(|why| {
            println!("Failed to write to .gitignore: {:?}", why.kind());
        });

        // Write to user models file
        Self::write_to_user_models(&project).unwrap_or_else(|why| {
            println!("Failed to write to user_models.rs: {:?}", why.kind());
        });
        write_to_routes_mod(&project.routes_module, "index".to_string()).unwrap_or_else(|why| {
            println!("Failed to write to routes/mod: {:?}", why.kind());
        });
        // Write to Header
        write_to_header(&project.header_section).unwrap_or_else(|why| {
            println!("Failed to write to header: {:?}", why.kind());
        });

        // write to navbar
        write_to_navbar(&project).unwrap_or_else(|why| {
            println!("Failed to write to navbar: {:?}", why.kind());
        });

        // write to the dashboard page
        write_to_dashboard(project.clone()).unwrap_or_else(|why| {
            println!("Failed to write to dashboard: {:?}", why.kind());
        });

        // write to the login page
        write_to_login_page(project.clone()).unwrap_or_else(|why| {
            println!("Failed to write to login: {:?}", why.kind());
        });

        // We need to tell Diesel where to find our database. We do this by setting the DATABASE_URL environment variable.
        // We can do this by running the following command in the terminal:
        let temp_database = &database_data.clone();
        // Embed migrations from the "migrations" directory
        // Use the embed_migrations macro to embed migrations into the binary
        // Adjust the path to point to the location of your migration files

        match temp_database.database_type {
            DatabaseType::Sqlite => {
                // Create the database URL
                let database_url = format!("{}", project.config_dev_db);
                println!("database_url: {}", database_url);

                // In SQLite, creating a connection to a non-existent database
                // automatically creates the database file, so we don't need to
                // explicitly create the database.

                // Generate the SQL content for the new project
                let sql_content =
                    initial_sql_loader::load_sql_for_new_project(&project, database_data.clone())
                        .await?;

                // Establish a connection to the new database
                let connection_result = SqliteConnectOptions::new()
                    .filename(&database_url)
                    .connect()
                    .await;

                // Check if the connection was successful
                let mut connection = match connection_result {
                    Ok(conn) => conn,
                    Err(why) => {
                        panic!("Failed to establish connection: {}", why.to_string());
                    }
                };

                // Iterate through the vector of SQL commands and execute them one at a time
                for sql_command in sql_content {
                    // Execute the SQL command
                    sqlx::query(&sql_command)
                        .execute(&mut connection)
                        .await
                        .unwrap_or_else(|why| {
                            panic!("Failed to execute SQL command: {}", why.to_string())
                        });
                }
            }

            DatabaseType::Postgres => {
                // Replace this line with the correct URL for the default "postgres" database
                let admin_database_url = format!(
                    "postgres://{}:{}@{}:{}/postgres",
                    database_data.username,
                    database_data.password,
                    database_data.host,
                    database_data.port,
                );

                // Call the function with the admin_database_url
                create_database_if_not_exists(&admin_database_url, database_data.clone())
                    .await
                    .unwrap_or_else(|why| {
                        panic!("Failed to create database: {:?}", why);
                    });

                // Create the database URL
                let database_url = format!(
                    "postgres://{}:{}@{}:{}/{}",
                    database_data.username,
                    database_data.password,
                    database_data.host,
                    database_data.port,
                    database_data.name
                );

                // Update the DATABASE_URL environment variable to point to the new 'test' database
                env::set_var(
                    "DATABASE_URL",
                    database_url.replace(&database_data.name, "test"),
                );

                project.config_dev_db = database_url.clone();

                println!("database_url: {}", database_url);

                // Generate the SQL content for the new project
                let sql_content =
                    initial_sql_loader::load_sql_for_new_project(&project, database_data.clone())
                        .await?;

                // Establish a connection to the new database
                let connection_result = PgConnectOptions::new()
                    .username(&database_data.username)
                    .password(&database_data.password)
                    .host(&database_data.host)
                    .port(database_data.port.parse::<u16>().unwrap())
                    .database(&database_data.name)
                    .connect()
                    .await;

                // Check if the connection was successful
                let mut connection = match connection_result {
                    Ok(conn) => conn,
                    Err(why) => {
                        panic!("Failed to establish connection: {}", why.to_string());
                    }
                };

                // Iterate through the vector of SQL commands and execute them one at a time
                for sql_command in sql_content {
                    // Execute the SQL command
                    sqlx::query(&sql_command)
                        .execute(&mut connection)
                        .await
                        .unwrap_or_else(|why| {
                            panic!("Failed to execute SQL command: {}", why.to_string())
                        });
                }
            }
            // ... (rest of the code) ...
            DatabaseType::Mysql => {
                // Create the database URL for the default "mysql" database
                let admin_database_url = format!(
                    "mysql://{}:{}@{}:{}/mysql",
                    database_data.username,
                    database_data.password,
                    database_data.host,
                    database_data.port,
                );

                // Call the function with the admin_database_url
                create_database_if_not_exists(&admin_database_url, database_data.clone())
                    .await
                    .unwrap_or_else(|why| {
                        panic!("Failed to create database: {:?}", why);
                    });

                // Create the database URL for the new database
                let database_url = format!(
                    "mysql://{}:{}@{}:{}/{}",
                    database_data.username,
                    database_data.password,
                    database_data.host,
                    database_data.port,
                    database_data.name
                );

                // Update the DATABASE_URL environment variable to point to the new 'test' database
                env::set_var(
                    "DATABASE_URL",
                    database_url.replace(&database_data.name, "test"),
                );

                project.config_dev_db = database_url.clone();

                println!("database_url: {}", database_url);

                // Generate the SQL content for the new project
                let sql_content =
                    initial_sql_loader::load_sql_for_new_project(&project, database_data.clone())
                        .await?;

                // Establish a connection to the new database
                let connection_result = MySqlConnectOptions::new()
                    .username(&database_data.username)
                    .password(&database_data.password)
                    .host(&database_data.host)
                    .port(database_data.port.parse::<u16>().unwrap())
                    .database(&database_data.name)
                    .connect()
                    .await;

                // Check if the connection was successful
                let mut connection = match connection_result {
                    Ok(conn) => conn,
                    Err(why) => {
                        panic!("Failed to establish connection: {}", why.to_string());
                    }
                };

                // Iterate through the vector of SQL commands and execute them one at a time
                for sql_command in sql_content {
                    println!("Executing SQL command: {}", sql_command); // Log the SQL command being executed
                                                                        // Execute the SQL command
                    match sqlx::query(&sql_command).execute(&mut connection).await {
                        Ok(_) => {
                            println!("Successfully executed SQL command: {}", sql_command);
                        }
                        Err(why) => {
                            println!(
                                "Failed to execute SQL command: {}, Error: {}",
                                sql_command,
                                why.to_string()
                            );
                            // Optionally, return an error instead of panicking
                            // return Err(why.into());
                        }
                    }
                }
            }

            DatabaseType::Mongo => {
                // Create the database
                let database_url = format!(
                    "DATABASE_URL=mongodb://localhost:27017/{}",
                    &database_data.clone().name
                );
                println!("database_url: {}", database_url);
                let output = std::process::Command::new("diesel")
                    .arg("setup")
                    .env("DATABASE_URL", database_url)
                    .output()
                    .expect("Failed to execute process");
                println!("output: {:?}", output);
            }
        }

        println!("Project {} created!", &project.name);

        // Create the database
        Ok(project)
    } // End of create_new_project function

    pub fn create_new_route(route_name: String) -> Result<(), Error> {
        // the route will need to check the current directory to see if it is a rustyroad project
        // if it is not, it will return an error and ask the user to run the command in a rustyroad project
        // if it is a rustyroad project, it will create a new directory with the routeName
        // it will create a new file with the routeName.rs

        // check if the current directory is a rustyroad project
        let current_dir = fs::read_dir(".").unwrap();
        let mut has_rustyroad_toml = false;

        // check if the current directory has a rustyroad.toml file
        for entry in current_dir {
            let entry = entry.unwrap();
            let file_name = entry.file_name();
            let file_name = file_name.to_str().unwrap();
            if file_name == "rustyroad.toml" {
                has_rustyroad_toml = true;
            }
        }
        // rustyroad.toml file will be used to store the project name and other project information
        // if the current directory does not have a rustyroad.toml file, it will return an error
        if !has_rustyroad_toml {
            println!(
                "This is not a rustyroad project. Please run this command in a rustyroad project."
            );
            // end the function
            return Ok(());
        }

        // Create a new directory with the routeName
        create_dir(format!("./src/routes/{}", &route_name)).unwrap_or_else(|why| {
            println!("Failed to create directory: {:?}", why.kind());
        });
        // Create a new route using the routeName
        // Update the routes/mod.rs file
        let full_file_name = format!("./src/routes/mod.rs");
        write_to_routes_mod(&full_file_name, route_name.clone()).unwrap_or_else(|why| {
            println!("Failed to write to routes/mod: {:?}", why.kind());
        });

        // create the routes/mod.rs file
        create_file(&format!("./src/routes/{}/mod.rs", route_name)).unwrap_or_else(|why| {
            println!("Failed to create file: {:?}", why.kind());
        });

        let mut components = Vec::new();
        // Create a vector and push the routeName to the vector
        components.push(route_name.clone().to_string());

        // Write to mod.rs file
        writers::write_to_module(&format!("./src/routes/{}/mod.rs", &route_name), components)
            .unwrap_or_else(|why| {
                println!("Failed to write to mod.rs: {:?}", why.kind());
            });

        // Create a new file with the routeName.rs
        create_file(&format!("./src/routes/{}/{}.rs", route_name, route_name)).unwrap_or_else(
            |why| {
                println!("Failed to create file: {:?}", why.kind());
            },
        );
        // Write to routeName.rs file
        write_to_route_name_rs(route_name.clone()).unwrap_or_else(|why| {
            println!("Failed to write to routeName.rs: {:?}", why.kind());
        });

        // Create a new file with the routeName.html.tera
        create_file(&format!("./templates/pages/{}.html.tera", route_name)).unwrap_or_else(|why| {
            println!("Failed to create file: {:?}", why.kind());
        });
        // Write to routeName.html.tera file
        write_to_route_name_html(route_name.clone()).unwrap_or_else(|why| {
            println!("Failed to write to routeName.html.tera: {:?}", why.kind());
        });
        

        // update main.rs file
        add_new_route_to_main_rs(&route_name)?;
        // Create a new file with the routeName.css
        // Create a new file with the routeName.js
        // Create a new file with the routeName.test.js
        Ok(())
    }

    pub fn exit_program() {
        println!("Exiting...");
        std::process::exit(0);
    }

    pub fn cli() -> Command {
        Command::new("RustyRoad")
            .about("CLI for Rusty Road")
            .subcommand_required(true)
            .arg_required_else_help(true)
            .allow_external_subcommands(true)
            .subcommand(
                Command::new("new")
                    .about("Creates a new project")
                    .arg(arg!(<name> "The name of the project"))
                    .arg_required_else_help(true),
            )
            .subcommand(
                Command::new("generate")
                    .about("Generates a new route, model, or controller")
                    .subcommand(
                        Command::new("route")
                            .about("Generates a new route")
                            .arg(arg!(<name> "The name of the route")),
                    )
                    .subcommand(
                        Command::new("model")
                            .about("Generates a new model")
                            .arg(arg!(<name> "The name of the model")),
                    )
                    .subcommand(
                        Command::new("controller")
                            .about("Generates a new controller")
                            .arg(arg!(<name> "The name of the controller")),
                    )
                    .subcommand(
                        Command::new("migration")
                            .about("Generates a new migration")
                            .arg(arg!(<name> "The name of the migration"
                            ))
                            .subcommand_help_heading("SUBCOMMANDS:")
                            // if no subcommand is provided, print help
                            .subcommand_required(true)
                            .arg_required_else_help(true)
                            .allow_external_subcommands(true),
                    ),
            )
            .subcommand(
                Command::new("migrate")
                    .about("Runs migrations")
                    .subcommand(
                        Command::new("up")
                            .about("Runs all migrations")
                            .arg(arg!(<name> "The name of the migration")),
                    )
                    .subcommand(
                        Command::new("down")
                            .about("Rolls back the last migration")
                            .arg(arg!(<name> "The name of the migration")),
                    )
                    .subcommand(
                        Command::new("redo")
                            .about("Rolls back the last migration and runs it again")
                            .arg(arg!(<name> "The name of the migration")),
                    )
                    .subcommand(
                        Command::new("reset")
                            .about("Rolls back all migrations")
                            .arg(arg!(<name> "The name of the migration")),
                    )
                    .subcommand(
                        Command::new("status")
                            .about("Prints the status of all migrations")
                            .arg(arg!(<name> "The name of the migration")),
                    )
                    .subcommand(
                        Command::new("create")
                            .about("Creates a new migration")
                            .arg(arg!(<name> "The name of the migration")),
                    )
                    .subcommand_help_heading("SUBCOMMANDS:")
                    // if no subcommand is provided, print help
                    .subcommand_required(true)
                    .arg_required_else_help(true)
                    .allow_external_subcommands(true),
            )
    }

    pub fn push_args() -> Vec<Arg> {
        vec![arg!(-m --message <MESSAGE>)]
    }

    /// Runs the CLI
    /// # Examples
    /// ```
    /// rusty_road::cli::run();
    /// ```
    pub async fn run() {
        let matches = Self::cli().get_matches();
        match matches.subcommand() {
            Some(("new", matches)) => {
                let name = matches.get_one::<String>("name").unwrap().to_string();
                // ask what database they would like to use "postgres, mysql, sqlite, or none"
                // print a selection menu for the database
                // if they select postgres, mysql, or sqlite, ask for the database name, username, and password
                // if they select none, continue
                // create a new project with the name and database information

                // ask what database they would like to use "postgres, mysql, sqlite, or none"
                // print a selection menu for the database
                println!("What database would you like to use?");
                println!("1. Postgres");
                println!("2. MySQL");
                println!("3. SQLite");
                println!("4. MongoDB");
                println!("5. None");
                let mut database_choice = String::new();
                std::io::stdin()
                    .read_line(&mut database_choice)
                    .expect("Failed to read line");
                let database_choice_int = database_choice.trim().parse::<u32>().unwrap();

                // match the database choice
                match database_choice_int {
                    1 => {
                        // ask for the database name, username, and password
                        println!("What is the database name?");
                        let mut database_name = String::new();
                        std::io::stdin()
                            .read_line(&mut database_name)
                            .expect("Failed to read line");
                        let database_name = database_name.trim().to_string();
                        println!("What is the database username?");
                        let mut database_username = String::new();
                        std::io::stdin()
                            .read_line(&mut database_username)
                            .expect("Failed to read line");
                        let database_username = database_username.trim().to_string();
                        println!("What is the database password?");
                        let mut database_password = String::new();
                        std::io::stdin()
                            .read_line(&mut database_password)
                            .expect("Failed to read line");
                        let database_password = database_password.trim().to_string();
                        println!("What is the database port?");
                        let mut database_port = String::new();
                        std::io::stdin()
                            .read_line(&mut database_port)
                            .expect("Failed to read line");
                        let database_port = database_port.trim().parse::<u32>().unwrap();
                        println!("What is the database host?");
                        let mut database_host = String::new();
                        std::io::stdin()
                            .read_line(&mut database_host)
                            .expect("Failed to read line");
                        let database_host = database_host.trim().to_string();
                        database_choice = "Postgres".to_string();
                        // create a new project with the name and database information
                        let database: Database = Database::new(
                            database_name,
                            database_username,
                            database_password,
                            database_host,
                            database_port.to_string(),
                            database_choice.to_string(),
                        );
                        Self::create_new_project(name, database).await.err();
                    }
                    2 => {
                        // ask for the database name, username, and password
                        println!("What is the database name?");
                        let mut database_name = String::new();
                        std::io::stdin()
                            .read_line(&mut database_name)
                            .expect("Failed to read line");
                        let database_name = database_name.trim().to_string();
                        println!("What is the database username?");
                        let mut database_username = String::new();
                        std::io::stdin()
                            .read_line(&mut database_username)
                            .expect("Failed to read line");
                        let database_username = database_username.trim().to_string();
                        println!("What is the database password?");
                        let mut database_password = String::new();
                        std::io::stdin()
                            .read_line(&mut database_password)
                            .expect("Failed to read line");
                        let database_password = database_password.trim().to_string();
                        println!("What is the database port?");
                        let mut database_port = String::new();
                        std::io::stdin()
                            .read_line(&mut database_port)
                            .expect("Failed to read line");
                        let database_port = database_port.trim().parse::<u32>().unwrap();
                        println!("What is the database host?");
                        let mut database_host = String::new();
                        std::io::stdin()
                            .read_line(&mut database_host)
                            .expect("Failed to read line");
                        let database_host = database_host.trim().to_string();
                        database_choice = "mysql".to_string();
                        // create a new project with the name and database information
                        let database: Database = Database::new(
                            database_name,
                            database_username,
                            database_password,
                            database_host,
                            database_port.to_string(),
                            database_choice.to_string(),
                        );
                        Self::create_new_project(name, database).await.err();
                    }
                    3 => {
                        database_choice = "SQLite".to_string();
                        // Since we are using Rusqlite, we don't need to ask for a username or password port or database name
                        // create a new project with the name and database information
                        let database: Database = Database::new(
                            database_choice.to_string(),
                            "Sqlite Local DB".to_string(),
                            "Not Needed".to_string(),
                            "localhost".to_string(),
                            "Not Needed".to_string(),
                            "sqlite".to_string(),
                        );
                        Self::create_new_project(name, database).await.err();
                    }
                    4 => {
                        // ask for the database name, username, and password
                        println!("What is the database name?");
                        let mut database_name = String::new();
                        std::io::stdin()
                            .read_line(&mut database_name)
                            .expect("Failed to read line");
                        let database_name = database_name.trim().to_string();
                        println!("What is the database username?");
                        let mut database_username = String::new();
                        std::io::stdin()
                            .read_line(&mut database_username)
                            .expect("Failed to read line");
                        let database_username = database_username.trim().to_string();
                        println!("What is the database password?");
                        let mut database_password = String::new();
                        std::io::stdin()
                            .read_line(&mut database_password)
                            .expect("Failed to read line");
                        let database_password = database_password.trim().to_string();
                        println!("What is the database port?");
                        let mut database_port = String::new();
                        std::io::stdin()
                            .read_line(&mut database_port)
                            .expect("Failed to read line");
                        let database_port = database_port.trim().parse::<u32>().unwrap();
                        println!("What is the database host?");
                        let mut database_host = String::new();
                        std::io::stdin()
                            .read_line(&mut database_host)
                            .expect("Failed to read line");
                        let database_host = database_host.trim().to_string();
                        database_choice = "MongoDB".to_string();
                        // create a new project with the name and database information
                        let database: Database = Database::new(
                            database_choice.to_string(),
                            database_name,
                            database_username,
                            database_password,
                            database_port.to_string(),
                            database_host,
                        );
                        Self::create_new_project(name, database).await.err();
                    }
                    5 => {
                        // create a new project with the name and database information
                        let database: Database = Database::new(
                            database_choice.to_string(),
                            "".to_string(),
                            "".to_string(),
                            "".to_string(),
                            "".to_string(),
                            "".to_string(),
                        );
                        Self::create_new_project(name, database).await.err();
                    }
                    _ => {
                        println!("Invalid database choice");
                    }
                };
            }
            Some(("generate", matches)) => {
                match matches.subcommand() {
                    Some(("route", matches)) => {
                        let name = matches.get_one::<String>("name").unwrap().to_string();
                        match Self::create_new_route(name) {
                            // This is always going to be okay becase the error will be handled in the console
                            Ok(_) => return,
                            Err(e) => println!("Error generating route: {}", e.kind()),
                        }
                    }
                    Some(("model", matches)) => {
                        let name = matches.get_one::<String>("name").unwrap().to_string();
                        match Self::create_new_route(name) {
                            Ok(_) => println!("Model created!"),
                            Err(e) => println!("Error creating model: {}", e),
                        }
                    }
                    Some(("controller", matches)) => {
                        let name = matches.get_one::<String>("name").unwrap().to_string();
                        match Self::create_new_route(name) {
                            Ok(_) => println!("Controller created!"),
                            Err(e) => println!("Error creating controller: {}", e),
                        }
                    }
                    Some(("migration", matches)) => {
                        let name = matches.get_one::<String>("name").unwrap().to_string();
                        match create_migration(&name) {
                            Ok(_) => println!("Migration created!"),
                            Err(e) => println!("Error creating migration: {}", e),
                        }
                    }
                    _ => {
                        println!("Invalid generate choice");
                    }
                }
            }
            _ => {
                println!("Invalid project choice");
            }
        }
    }
}
