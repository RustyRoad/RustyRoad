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
use color_eyre::eyre::Result;
use dialoguer::Confirm;
use eyre::Error;
use serde::Deserialize;
use sqlx::mysql::MySqlConnectOptions;
use sqlx::postgres::PgConnectOptions;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::ConnectOptions;
use std::env;
use std::{fs::OpenOptions, io::Write};
use tokio::io;

pub mod database;
pub mod features;
pub mod generators;

use crate::features::add_feature;
use database::*;

pub mod helpers;
pub mod writers;
use crate::generators::create_directories_for_new_project;
use crate::writers::*;
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

/**
 * # Struct Project
 * ## Description
 * This struct is used to configure the project.
 * This is specfically used to read the rustyroad.toml file and
 * and decode the toml into a struct.
 */
#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Hash, Copy, Parser)]
pub enum CRUDType {
    Create,
    Read,
    Update,
    Delete,
}

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
    pub env: String,
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
    pub auth_template_layouts: String,
    pub authenticated_layout: String,
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
    pub controllers: String,
    pub controllers_module: String,
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
    pub config_initializers_controllers: String,
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
    pub user_model: String,
    pub initial_migration_directory: String,
    pub initial_migration_up: String,
    pub initial_migration_down: String,
    pub user_test: String,
    pub not_found_controller: String,
    pub index_controller: String,
    pub login_controller: String,
    pub signup_controller: String,
    pub reset_password_controller: String,
    pub forgot_password_controller: String,
    pub dashboard_controller: String,
    pub navbar_component: String,
    pub sidebar_component: String,
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
            "[rustyroad_project]
name = \"{}\"
[database]
database_name = \"{}\"
database_user = \"{}\"
database_password = \"{}\"
database_host = \"{}\"
database_port = \"{}\"
database_type = \"{}\"",
            self.name,
            database_data.clone().name,
            database_data.username,
            database_data.password,
            database_data.host,
            database_data.port,
            database_data.database_type.to_string().to_ascii_lowercase()
        );
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
            "{
                \"name\": \"rustyroad\",
                \"version\": \"1.0.0\",
                \"main\": \"index.js\",
                \"repository\": \"https://github.com/Riley-Seaburg/RustyRoad.git\",
                \"author\": \"Riley Seaburg <riley@rileyseaburg.com>\",
                \"license\": \"MIT\",
                \"scripts\": {
                  \"server\": \"cargo run\",
                  \"tailwind:dev\": \"npx tailwindcss -i ./src/tailwind.css -o ./static/css/styles.css --watch\",
                  \"tailwind:build\": \"npx tailwindcss -i ./src/tailwind.css -o ./static/css/styles.css --minify\",
                  \"dev\": \"concurrently \\\"yarn tailwind:dev\\\" \\\" yarn server\\\"\"
                },
                \"devDependencies\": {
                  \"@tailwindcss/forms\": \"^0.5.3\",
                  \"concurrently\": \"^7.6.0\",
                  \"tailwindcss\": \"^3.2.4\"
                }
              }"
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
This project was created using Rusty Road. Rusty Road is Rails for Rust. It is a CLI tool that allows you to create a new Rust project with a few commands. It also comes with TailwindCSS and Actix pre-installed.

## Getting Started

### Configure TailwindCSS

```bash
npx tailwindcss init -p
```

### Set Environment Variables

```bash
cp .env.example .env
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
        content: ['./views/**/*.{html.tera,js}'],
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
        create_directories_for_new_project(&project).unwrap_or_else(|why| {
            println!("Couldn't create directory: {:?}", why.to_string());
        });

        // Create the files
        create_files(&project).unwrap_or_else(|why| {
            panic!("Couldn't create files: {:?}", why.to_string());
        });

        // write to the .env file
        let value = set_env(&project).unwrap();
        write_to_file(&project.env, value.as_bytes()).unwrap_or_else(|why| {
            println!("Couldn't write to .env: {:?}", why.to_string());
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
            println!("Failed to write to index.js: {:?}", why.to_string());
        });
        // Write to index.html.tera file
        write_to_index_html(&project).unwrap_or_else(|why| {
            println!("Failed to write to index.html: {:?}", why.to_string());
        });
        // Write to base.html.tera file
        write_to_base_html(&project.base_html).unwrap_or_else(|why| {
            println!("Failed to write to base.html: {:?}", why.to_string());
        });

        // Write to tailwind.css file
        Self::write_to_tailwind_css(&project).unwrap_or_else(|why| {
            println!("Failed to write to tailwind.css: {:?}", why.to_string());
        });
        // need to create the function
        // Write to tailwind.config.js file
        Self::write_to_tailwind_config(&project).unwrap_or_else(|why| {
            println!(
                "Failed to write to tailwind.config.js: {:?}",
                why.to_string()
            );
        });

        // Write to postcss.config.js file
        Self::write_to_postcss_config(&project).unwrap_or_else(|why| {
            println!(
                "Failed to write to postcss.config.js: {:?}",
                why.to_string()
            );
        });

        // Write to index.html controller
        write_to_index_controller(&project).unwrap_or_else(|why| {
            println!("Failed to write to index.html: {:?}", why.to_string());
        });

        // Write to gitignore file
        Self::write_to_gitignore(&project).unwrap_or_else(|why| {
            println!("Failed to write to .gitignore: {:?}", why.to_string());
        });

        write_to_controllers_mod(&project.controllers_module, "index".to_string()).unwrap_or_else(
            |why| {
                println!("Failed to write to controllers/mod: {:?}", why.to_string());
            },
        );
        // Write to Header
        write_to_header(&project.header_section).unwrap_or_else(|why| {
            println!("Failed to write to header: {:?}", why.to_string());
        });

        // write to navbar
        write_to_navbar(&project).unwrap_or_else(|why| {
            println!("Failed to write to navbar: {:?}", why.to_string());
        });

        // write to sidebar
        write_to_sidebar(&project).unwrap_or_else(|why| {
            println!("Failed to write to sidebar: {:?}", why.to_string());
        });

        // write to the dashboard page
        write_to_dashboard(project.clone()).unwrap_or_else(|why| {
            println!("Failed to write to dashboard: {:?}", why.to_string());
        });

        // write to the login page
        write_to_login_page(project.clone()).unwrap_or_else(|why| {
            println!("Failed to write to login: {:?}", why.to_string());
        });

        write_to_authenticated_layout(project.clone()).unwrap_or_else(|why| {
            println!("Failed to write to authenticated layout: {:?}", why.to_string());
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
                let database_url = project.config_dev_db.to_string();
                println!("database_url: {database_url}");

                // In SQLite, creating a connection to a non-existent database
                // automatically creates the database file, so we don't need to
                // explicitly create the database.

                // Generate the SQL content for the new project
                let sql_content = load_sql_for_new_project(&project, database_data.clone()).await?;

                // Establish a connection to the new database
                let connection_result = SqliteConnectOptions::new()
                    .filename(&database_url)
                    .connect()
                    .await;

                // Check if the connection was successful
                let mut connection = match connection_result {
                    Ok(conn) => conn,
                    Err(why) => {
                        panic!("Failed to establish connection: {why}");
                    }
                };

                // Iterate through the vector of SQL commands and execute them one at a time
                for sql_command in sql_content {
                    // Execute the SQL command
                    sqlx::query(&sql_command)
                        .execute(&mut connection)
                        .await
                        .unwrap_or_else(|why| panic!("Failed to execute SQL command: {why}"));
                }

                write_to_sqlite_user_models(&project).unwrap_or_else(|why| {
                    println!("Failed to write to user models: {:?}", why.to_string());
                });
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
                        panic!("Failed to create database: {why}");
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

                println!("database_url: {database_url}");

                // Generate the SQL content for the new project
                let sql_content =
                    initial_sql_loader::load_sql_for_new_project(&project, database_data.clone())
                        .await?;

                // Establish a connection to the new database
                let connection_result = PgConnectOptions::new()
                    .username(&database_data.username)
                    .password(&database_data.password)
                    .host(&database_data.host)
                    .port(database_data.port.to_string().parse::<u16>().unwrap())
                    .database(&database_data.name)
                    .connect()
                    .await;

                // Check if the connection was successful
                let mut connection = match connection_result {
                    Ok(conn) => conn,
                    Err(why) => {
                        panic!("Failed to establish connection: {why}");
                    }
                };

                // Iterate through the vector of SQL commands and execute them one at a time
                for sql_command in sql_content {
                    // Execute the SQL command
                    sqlx::query(&sql_command)
                        .execute(&mut connection)
                        .await
                        .unwrap_or_else(|why| panic!("Failed to execute SQL command: {why}"));
                }

                /* Write to user models file */
                write_to_postgres_user_models(&project).unwrap_or_else(|why| {
                    println!("Failed to write to user models: {why}");
                });
            }

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

                println!("database_url: {database_url}");

                // Generate the SQL content for the new project
                let sql_content =
                    initial_sql_loader::load_sql_for_new_project(&project, database_data.clone())
                        .await?;

                // Establish a connection to the new database
                let connection_result = MySqlConnectOptions::new()
                    .username(&database_data.username)
                    .password(&database_data.password)
                    .host(&database_data.host)
                    .port(database_data.port)
                    .database(&database_data.name)
                    .connect()
                    .await;

                // Check if the connection was successful
                let mut connection = match connection_result {
                    Ok(conn) => conn,
                    Err(why) => {
                        panic!("Failed to establish connection: {why}");
                    }
                };

                // Iterate through the vector of SQL commands and execute them one at a time
                for sql_command in sql_content {
                    println!("Executing SQL command: {sql_command}"); // Log the SQL command being executed
                                                                      // Execute the SQL command
                    match sqlx::query(&sql_command).execute(&mut connection).await {
                        Ok(_) => {
                            println!("Successfully executed SQL command: {sql_command}");
                        }
                        Err(why) => {
                            println!("Failed to execute SQL command: {sql_command}, Error: {why}");
                            // Optionally, return an error instead of panicking
                            // return Err(why.into());
                        }
                    }
                }

                write_to_mysql_user_models(&project).unwrap_or_else(|why| {
                    println!("Failed to write to user models: {:?}", why.to_string());
                });
            }

            DatabaseType::Mongo => {
                // Create the database
                let database_url = format!(
                    "DATABASE_URL=mongodb://localhost:27017/{}",
                    &database_data.clone().name
                );
                println!("database_url: {database_url}");
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
                    .about("Generates a new controller, model, or controller")
                    .subcommand(
                        Command::new("controller")
                            .about("Generates a new controller")
                            .subcommand_required(false)
                            .arg_required_else_help(false)
                            .allow_external_subcommands(false)
                           ,
                    )
                    .subcommand(
                        Command::new("model")
                            .about("Generates a new model")
                            .arg(arg!(<name> "The name of the model"))
                            .subcommand_help_heading("SUBCOMMANDS:")
                            // if no subcommand is provided, print help
                            .subcommand_required(true)
                            .arg_required_else_help(true)
                            .allow_external_subcommands(true),
                    )
                    .subcommand(
                        Command::new("controller")
                            .about("Generates a new controller"),
                    )
                    .subcommand(
                        Command::new("migration")
                            .about("Generates a new migration")
                            .arg(arg!(<name> "The name of the migration"
                            ))
                            .arg_required_else_help(true),
                    )
                    .after_help(
                        "EXAMPLES:
                To generate a new controller:
                    rustyroad generate controller <name>
                To generate a new model:
                    rustyroad generate model <name>
                To generate a new controller:
                    rustyroad generate controller <name>
                To generate a new migration:
                    rustyroad generate migration <name>",
                    )
                    .subcommand_required(true),
            )
            .subcommand(
                Command::new("migration")
                    .about("Runs migrations")
                    .subcommand(
                        Command::new("generate")
                            .about("Generates a migration")
                            .arg(arg!(<name> "The name of the migration")),
                    )
                    .subcommand(
                        Command::new("run all")
                            .about("Runs all the migrations in the migration directory"),
                    )
                    .subcommand(
                        Command::new("run")
                            .about("Run a specific migration by name")
                            .arg(arg!(<name> "The name of the migration to run.")),
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
                    .subcommand_help_heading("SUBCOMMANDS:")
                    // if no subcommand is provided, print help
                    .subcommand_required(true)
                    .arg_required_else_help(true)
                    .allow_external_subcommands(true),
            )
            .subcommand(
                Command::new("feature")
                    .about("Adds a feature to the project")
                    .subcommand(
                        Command::new("add")
                            .about("Adds a feature to the project")
                            .subcommand(
                                Command::new("grapesjs").about("Adds grapesjs to the project"),
                            )
                            .subcommand_required(true)
                            .arg_required_else_help(true)
                            .allow_external_subcommands(true),
                    )
                    .subcommand_required(true)
                    .arg_required_else_help(true)
                    .allow_external_subcommands(true),
            )
    }

    pub fn push_args() -> Vec<Arg> {
        vec![arg!(-m --message <MESSAGE>)]
    }

    pub async fn run() {
        let matches = Self::cli().get_matches();
        match matches.subcommand() {
            // New Project Case
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
                        let database_port = database_port.trim().parse::<u16>().unwrap();
                        println!("What is the database host?");
                        let mut database_host = String::new();
                        std::io::stdin()
                            .read_line(&mut database_host)
                            .expect("Failed to read line");
                        let database_host = database_host.trim().to_string();
                        database_choice = "postgres".to_string();
                        // create a new project with the name and database information
                        let database: Database = Database::new(
                            database_name,
                            database_username,
                            database_password,
                            database_host,
                            database_port,
                            database_choice.as_str(),
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
                        let database_port = database_port.trim().parse::<u16>().unwrap();
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
                            database_port,
                            database_choice.as_str(),
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
                            0,
                            "sqlite".trim_end(),
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
                        let database_port = database_port.trim().parse::<u16>().unwrap();
                        println!("What is the database host?");
                        let mut database_host = String::new();
                        std::io::stdin()
                            .read_line(&mut database_host)
                            .expect("Failed to read line");
                        let database_host = database_host.trim().to_string();
                        database_choice = "mongodb".to_string();
                        // create a new project with the name and database information
                        let database: Database = Database::new(
                            database_choice.to_string(),
                            database_name,
                            database_username,
                            database_password,
                            database_port,
                            database_host.as_str(),
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
                            0,
                            "".to_string().as_str(),
                        );
                        Self::create_new_project(name, database).await.err();
                    }
                    _ => {
                        println!("Invalid database choice");
                    }
                };
            }
            // Generate new controllers, models, controllers and migrations
            Some(("generate", matches)) => match matches.subcommand() {
                Some(("controller", _matches)) => {
                    // because we removed the arguments from the user, we need to auto define the controller name below
                    println!("What type of controller would you like to create?");
                    println!("1. GET");
                    println!("2. POST");
                    println!("3. PUT");
                    println!("4. DELETE");
                    let mut controller_type_choice = String::new();
                    std::io::stdin()
                        .read_line(&mut controller_type_choice)
                        .expect("Failed to read line");
                    let controller_type_choice_int =
                        controller_type_choice.trim().parse::<u32>().unwrap();
                    let crud_type = match controller_type_choice_int {
                        1 => CRUDType::Read,
                        2 => CRUDType::Create,
                        3 => CRUDType::Update,
                        4 => CRUDType::Delete,
                        _ => {
                            println!("Invalid controller type choice");
                            return;
                        }
                    };

                    // ask the user the name of the controller
                    println!("What is the name of the model you want to create a controller for?: ");
                    println!("In order to work out of the box, ensure the model already exists.");
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).unwrap();
                    // this pattern needs to be repeated for each CRUD type
                    let model_name = input.trim();

                    let _ = create_new_controller(model_name.to_string(), crud_type)
                        .await
                        .expect("Error creating controller");

                }
                Some(("model", _matches)) => {
                    todo!("Implement this");
                }
                Some(("migration", matches)) => {
                    let name = matches.get_one::<String>("name").unwrap().to_string();
                    create_migration(&name)
                        .await
                        .expect("Error creating migration");
                }
                _ => {
                    println!("Invalid generate choice");
                }
            },
            // Migration Case - Can generate migrations, run migrations, and rollback migrations
            Some(("migration", matches)) => match matches.subcommand() {
                Some(("generate", matches)) => {
                    let name = matches.get_one::<String>("name").unwrap().to_string();
                    create_migration(&name)
                        .await
                        .expect("Error creating migration");
                }
                Some(("run all", _)) => {
                    todo!("Implement this");
                }
                Some(("run", matches)) => {
                    let name = matches.get_one::<String>("name").unwrap().to_string();

                    let confirmation = Confirm::new()
                        .with_prompt(&format!(
                            "Are you sure you want to execute the '{}' migration?",
                            name.clone()
                        ))
                        .interact()
                        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))
                        .expect("Error executing migration: ");

                    if confirmation {
                        println!("Executing the '{}' migration...", name.clone());
                        run_migration(name.clone(), MigrationDirection::Up)
                            .await
                            .expect("Error running migration");
                        println!("'{}' migration completed successfully!", name.clone());
                    } else {
                        println!("'{}' migration canceled by user.", name);
                    }
                }
                Some(("rollback", matches)) => {
                    let name = matches.get_one::<String>("name").unwrap().to_string();
                    // Create a confirmation prompt
                    let confirmation = Confirm::new()
                        .with_prompt(&format!(
                            "Are you sure you want to rollback the '{}' migration?",
                            name
                        ))
                        .interact()
                        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))
                        .expect("Error rolling back migration: ");

                    if confirmation {
                        println!("Rolling back the '{}' migration...", name.clone());
                        run_migration(name.clone(), MigrationDirection::Down)
                            .await
                            .expect("Error rolling back migration");
                        println!(
                            "'{}' migration rollback completed successfully!",
                            name.clone()
                        );
                    } else {
                        println!("'{}' migration rollback canceled by user.", name);
                    }
                }
                _ => {
                    println!("Invalid migration choice");
                }
            },
            // Add Feature Case
            Some(("feature", matches)) => match matches.subcommand() {
                Some(("add", matches)) => match matches.subcommand() {
                    Some(("grapesjs", _matches)) => {
                        // ask the user if they are sure they want to add grapesjs to the project
                        // if they are sure, add grapesjs to the project
                        // if they are not sure, cancel the command
                        let confirmation = Confirm::new()
                            .with_prompt("Are you sure you want to add grapesjs to the project?")
                            .interact()
                            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))
                            .expect("Error adding grapesjs to the project: ");

                        if confirmation {
                            add_feature("grapesjs".to_string())
                                .await
                                .expect("Error adding grapesjs to the project");
                        }
                    }
                    _ => {}
                },
                _ => {
                    println!("Invalid add choice");
                }
            },

            _ => {
                println!("Invalid add choice");
            }
        }
    }
}
