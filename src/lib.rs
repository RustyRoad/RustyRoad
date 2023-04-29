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
    migrations::initial_sql_loader, templates::project_creation::create_file::create_files,
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
use crate::writers::templates::navbar::write_to_navbar;
use crate::writers::templates::{new, write_to_base_html, write_to_header, write_to_dashboard, write_to_login_page};
use crate::writers::{
    add_new_route_to_main_rs, create_database_if_not_exists, write_to_file, write_to_main_rs,
    write_to_route_name_html, write_to_route_name_rs, write_to_routes_mod,
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
    // Write to cargo_toml
    // if the database is postgres, add the postgres crate to the dependencies
    // if the database is mysql, add the mysql crate to the dependencies
    // if the database is sqlite, add the rusqlite crate to the dependencies
    fn write_to_cargo_toml(&self, database_data: &Database) -> Result<(), Error> {
        match database_data.database_type {
            DatabaseType::Postgres => {
                let config = format!(
                    "[package]
name = \"{}\"
version = \"0.1.0\"
authors = [\"RustyRoad\"]
edition = \"2021\"
[dependencies]
rocket = \"0.5.0-rc.2\"
tokio = {{ version = \"1\", features = [\"macros\", \"rt-multi-thread\"] }}
serde = {{ version = \"1.0\", features = [\"derive\"] }}
serde_json = \"1.0.82\"
random-string = \"1.0.0\"
env_logger = \"0.10.0\"
local-ip-address = \"0.5.0\"
futures = \"0.3.23\"
tera = \"1.17.1\"
reqwest = \"0.11\"
rocket_dyn_templates = {{version = \"0.1.0-rc.2\", features = [\"tera\"]}}
rustyroad = \"0.1.3\"
[dependencies.sqlx]
features = [\"postgres\", \"macros\", \"chrono\", \"json\", \"uuid\", \"offline\"]
version = \"0.6.2\"",
                    &self.name
                );
                let mut file = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(&self.cargo_toml)?;

                file.write_all(config.as_bytes())?;

                return Ok(());
            }

            DatabaseType::Mysql => {
                let config = format!(
                    "[package]
name = \"{}\"
version = \"0.1.0\"
authors = [\"RustyRoad\"]
edition = \"2021\"
[dependencies]
rocket = \"0.5.0-rc.2\"
tokio = {{ version = \"1\", features = [\"macros\", \"rt-multi-thread\"] }}
serde = {{ version = \"1.0\", features = [\"derive\"] }}
serde_json = \"1.0.82\"
random-string = \"1.0.0\"
env_logger = \"0.10.0\"
local-ip-address = \"0.5.0\"
futures = \"0.3.23\"
tera = \"1.17.1\"
reqwest = \"0.11\"
rocket_dyn_templates = {{version = \"0.1.0-rc.2\", features = [\"tera\"]}}
rustyroad = \"0.1.3\"
[dependencies.sqlx]
features = [\"mysql\", \"macros\", \"chrono\", \"json\", \"uuid\", \"offline\"]
version = \"0.6.2\"",
                    &self.name
                );
                let mut file = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(&self.cargo_toml)?;

                file.write_all(config.as_bytes())?;

                Ok(())
            }

            DatabaseType::Sqlite => {
                let config = format!(
                    "[package]
name = \"{}\"
version = \"0.1.0\"
authors = [\"RustyRoad\"]
edition = \"2021\"
[dependencies]
rocket = \"0.5.0-rc.2\"
tokio = {{ version = \"1\", features = [\"macros\", \"rt-multi-thread\"] }}
serde = {{ version = \"1.0\", features = [\"derive\"] }}
serde_json = \"1.0.82\"
random-string = \"1.0.0\"
env_logger = \"0.10.0\"
local-ip-address = \"0.5.0\"
futures = \"0.3.23\"
tera = \"1.17.1\"
reqwest = \"0.11\"
rocket_dyn_templates = {{version = \"0.1.0-rc.2\", features = [\"tera\"]}}
rustyroad = \"0.1.3\"
[dependencies.rusqlite]
version = \"0.28.0\"
features = [\"bundled\"]",
                    &self.name
                );
                let mut file = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(&self.cargo_toml)?;

                file.write_all(config.as_bytes())?;

                Ok(())
            }

            DatabaseType::Mongo => {
                let config = format!(
                    "[package]
name = \"{}\"
version = \"0.1.0\"
authors = [\"RustyRoad\"]
edition = \"2021\"
[dependencies]
rocket = \"0.5.0-rc.2\"
tokio = {{ version = \"1\", features = [\"macros\", \"rt-multi-thread\"] }}
serde = {{ version = \"1.0\", features = [\"derive\"] }}
serde_json = \"1.0.82\"
random-string = \"1.0.0\"
env_logger = \"0.10.0\"
local-ip-address = \"0.5.0\"
futures = \"0.3.23\"
tera = \"1.17.1\"
reqwest = \"0.11\"
rocket_dyn_templates = {{version = \"0.1.0-rc.2\", features = [\"tera\"]}}
rustyroad = \"0.1.3\"
[dependencies.mongodb]
version =  \"2.4.0\"
default-features = false
features = [\"sync\", \"bson\", \"tls\"]",
                    &self.name
                );
                let mut file = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(&self.cargo_toml)?;

                file.write_all(config.as_bytes())?;

                Ok(())
            }
        }
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
    // Write to index.html.tera
    fn write_to_index_html(&self) -> Result<(), Error> {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(&self.index_html)
            .expect("Failed to open index.html");

        file.write_all(
            b"{% extends 'base' %}
{% block title %}Index{% endblock title %}
{% block head %}
{{ super() }}
{% endblock head %}
{% block content %}
<div class='bg-gray-900 pt-10 sm:pt-16 lg:overflow-hidden lg:pt-8 lg:pb-14'>
  <div class='mx-auto max-w-7xl lg:px-8'>
    <div class='lg:grid lg:grid-cols-2 lg:gap-8'>
      <div class='mx-auto max-w-md px-6 sm:max-w-2xl sm:text-center lg:flex lg:items-center lg:px-0 lg:text-left'>
        <div class='lg:py-24'>
          <a href='#'
            class='inline-flex items-center rounded-full bg-black p-1 pr-2 text-white hover:text-gray-200 sm:text-base lg:text-sm xl:text-base'>
            <span
              class='rounded-full bg-gradient-to-r from-teal-500 to-cyan-600 px-3 py-0.5 text-sm font-semibold leading-5 text-white'>We\xE2\x80\x99re
              hiring</span>
            <span class='ml-4 text-sm'>Visit our careers page</span>
            <!-- Heroicon name: mini/chevron-right -->
            <svg class='ml-2 h-5 w-5 text-gray-500' xmlns='http://www.w3.org/2000/svg' viewBox='0 0 20 20'
              fill='currentColor' aria-hidden='true'>
              <path fill-rule='evenodd'
                d='M7.21 14.77a.75.75 0 01.02-1.06L11.168 10 7.23 6.29a.75.75 0 111.04-1.08l4.5 4.25a.75.75 0 010 1.08l-4.5 4.25a.75.75 0 01-1.06-.02z'
                clip-rule='evenodd' />
            </svg>
          </a>
          <h1 class='mt-4 text-4xl font-bold tracking-tight text-white sm:mt-5 sm:text-6xl lg:mt-6 xl:text-6xl'>
            <span class='block'>A better way to</span>
            <span
              class='block bg-gradient-to-r from-teal-200 to-cyan-400 bg-clip-text pb-3 text-transparent sm:pb-5'>ship
              web apps</span>
          </h1>
          <p class='text-base text-gray-300 sm:text-xl lg:text-lg xl:text-xl'>Anim aute id magna aliqua ad ad non
            deserunt sunt. Qui irure qui Lorem cupidatat commodo. Elit sunt amet fugiat veniam occaecat fugiat.</p>
          <div class='mt-10 sm:mt-12'>
            <form action='#' class='sm:mx-auto sm:max-w-xl lg:mx-0'>
              <div class='sm:flex'>
                <div class='min-w-0 flex-1'>
                  <label for='email' class='sr-only'>Email address</label>
                  <input id='email' type='email' placeholder='Enter your email'
                    class='block w-full rounded-md border-0 px-4 py-3 text-base text-gray-900 placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-cyan-400 focus:ring-offset-2 focus:ring-offset-gray-900'>
                </div>
                <div class='mt-3 sm:mt-0 sm:ml-3'>
                  <button type='submit'
                    class='block w-full rounded-md bg-gradient-to-r from-teal-500 to-cyan-600 py-3 px-4 font-medium text-white shadow hover:from-teal-600 hover:to-cyan-700 focus:outline-none focus:ring-2 focus:ring-cyan-400 focus:ring-offset-2 focus:ring-offset-gray-900'>Start
                    free trial</button>
                </div>
              </div>
              <p class='mt-3 text-sm text-gray-300 sm:mt-4'>Start your free 14-day trial, no credit card necessary. By
                providing your email, you agree to our <a href='#' class='font-medium text-white'>terms of service</a>.
              </p>
            </form>
          </div>
        </div>
      </div>
      <div class='mt-12 -mb-16 sm:-mb-48 lg:relative lg:m-0'>
        <div class='mx-auto max-w-md px-6 sm:max-w-2xl lg:max-w-none lg:px-0'>
          <!-- Illustration taken from Lucid Illustrations: https://lucid.pixsellz.io/ -->
          <img class='w-full lg:absolute lg:inset-y-0 lg:left-0 lg:h-full lg:w-auto lg:max-w-none'
            src='https://tailwindui.com/img/component-images/cloud-illustration-teal-cyan.svg' alt=''>
        </div>
      </div>
    </div>
  </div>
</div>

<main>
  <!-- Feature section with screenshot -->
  <!-- Ignore if not present -->
  {% include 'sections/feature-section-w-screenshot'  ignore missing %}

  <!-- Feature section with grid -->
  <div class='relative bg-white py-16 sm:py-24 lg:py-32'>
    <div class='mx-auto max-w-md px-6 text-center sm:max-w-3xl lg:max-w-7xl lg:px-8'>
      <h2 class='text-lg font-semibold text-cyan-600'>Deploy faster</h2>
      <p class='mt-2 text-3xl font-bold tracking-tight text-gray-900 sm:text-4xl'>Everything you need to deploy your app
      </p>
      <p class='mx-auto mt-5 max-w-prose text-xl text-gray-500'>Phasellus lorem quam molestie id quisque diam aenean nulla
        in. Accumsan in quis quis nunc, ullamcorper malesuada. Eleifend condimentum id viverra nulla.</p>
      <div class='mt-12'>
        <div class='grid grid-cols-1 gap-8 sm:grid-cols-2 lg:grid-cols-3'>
          <div class='pt-6'>
            <div class='flow-root rounded-lg bg-gray-50 px-6 pb-8'>
              <div class='-mt-6'>
                <div>
                  <span
                    class='inline-flex items-center justify-center rounded-md bg-gradient-to-r from-teal-500 to-cyan-600 p-3 shadow-lg'>
                    <!-- Heroicon name: outline/cloud-arrow-up -->
                    <svg class='h-6 w-6 text-white' xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 24 24'
                      stroke-width='1.5' stroke='currentColor' aria-hidden='true'>
                      <path stroke-linecap='round' stroke-linejoin='round'
                        d='M12 16.5V9.75m0 0l3 3m-3-3l-3 3M6.75 19.5a4.5 4.5 0 01-1.41-8.775 5.25 5.25 0 0110.233-2.33 3 3 0 013.758 3.848A3.752 3.752 0 0118 19.5H6.75z' />
                    </svg>
                  </span>
                </div>
                <h3 class='mt-8 text-lg font-medium tracking-tight text-gray-900'>Push to Deploy</h3>
                <p class='mt-5 text-base text-gray-500'>Ac tincidunt sapien vehicula erat auctor pellentesque rhoncus. Et
                  magna sit morbi vitae lobortis.</p>
              </div>
            </div>
          </div>

          <div class='pt-6'>
            <div class='flow-root rounded-lg bg-gray-50 px-6 pb-8'>
              <div class='-mt-6'>
                <div>
                  <span
                    class='inline-flex items-center justify-center rounded-md bg-gradient-to-r from-teal-500 to-cyan-600 p-3 shadow-lg'>
                    <!-- Heroicon name: outline/lock-closed -->
                    <svg class='h-6 w-6 text-white' xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 24 24'
                      stroke-width='1.5' stroke='currentColor' aria-hidden='true'>
                      <path stroke-linecap='round' stroke-linejoin='round'
                        d='M16.5 10.5V6.75a4.5 4.5 0 10-9 0v3.75m-.75 11.25h10.5a2.25 2.25 0 002.25-2.25v-6.75a2.25 2.25 0 00-2.25-2.25H6.75a2.25 2.25 0 00-2.25 2.25v6.75a2.25 2.25 0 002.25 2.25z' />
                    </svg>
                  </span>
                </div>
                <h3 class='mt-8 text-lg font-medium tracking-tight text-gray-900'>SSL Certificates</h3>
                <p class='mt-5 text-base text-gray-500'>Qui aut temporibus nesciunt vitae dicta repellat sit dolores
                  pariatur. Temporibus qui illum aut.</p>
              </div>
            </div>
          </div>

          <div class='pt-6'>
            <div class='flow-root rounded-lg bg-gray-50 px-6 pb-8'>
              <div class='-mt-6'>
                <div>
                  <span
                    class='inline-flex items-center justify-center rounded-md bg-gradient-to-r from-teal-500 to-cyan-600 p-3 shadow-lg'>
                    <!-- Heroicon name: outline/arrow-path -->
                    <svg class='h-6 w-6 text-white' xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 24 24'
                      stroke-width='1.5' stroke='currentColor' aria-hidden='true'>
                      <path stroke-linecap='round' stroke-linejoin='round'
                        d='M4.5 12c0-1.232.046-2.453.138-3.662a4.006 4.006 0 013.7-3.7 48.678 48.678 0 017.324 0 4.006 4.006 0 013.7 3.7c.017.22.032.441.046.662M4.5 12l-3-3m3 3l3-3m12 3c0 1.232-.046 2.453-.138 3.662a4.006 4.006 0 01-3.7 3.7 48.657 48.657 0 01-7.324 0 4.006 4.006 0 01-3.7-3.7c-.017-.22-.032-.441-.046-.662M19.5 12l-3 3m3-3l3 3' />
                    </svg>
                  </span>
                </div>
                <h3 class='mt-8 text-lg font-medium tracking-tight text-gray-900'>Simple Queues</h3>
                <p class='mt-5 text-base text-gray-500'>Rerum quas incidunt deleniti quaerat suscipit mollitia. Amet
                  repellendus ut odit dolores qui.</p>
              </div>
            </div>
          </div>

          <div class='pt-6'>
            <div class='flow-root rounded-lg bg-gray-50 px-6 pb-8'>
              <div class='-mt-6'>
                <div>
                  <span
                    class='inline-flex items-center justify-center rounded-md bg-gradient-to-r from-teal-500 to-cyan-600 p-3 shadow-lg'>
                    <!-- Heroicon name: outline/shield-check -->
                    <svg class='h-6 w-6 text-white' xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 24 24'
                      stroke-width='1.5' stroke='currentColor' aria-hidden='true'>
                      <path stroke-linecap='round' stroke-linejoin='round'
                        d='M9 12.75L11.25 15 15 9.75m-3-7.036A11.959 11.959 0 013.598 6 11.99 11.99 0 003 9.749c0 5.592 3.824 10.29 9 11.623 5.176-1.332 9-6.03 9-11.622 0-1.31-.21-2.571-.598-3.751h-.152c-3.196 0-6.1-1.248-8.25-3.285z' />
                    </svg>
                  </span>
                </div>
                <h3 class='mt-8 text-lg font-medium tracking-tight text-gray-900'>Advanced Security</h3>
                <p class='mt-5 text-base text-gray-500'>Ullam laboriosam est voluptatem maxime ut mollitia commodi. Et
                  dignissimos suscipit perspiciatis.</p>
              </div>
            </div>
          </div>

          <div class='pt-6'>
            <div class='flow-root rounded-lg bg-gray-50 px-6 pb-8'>
              <div class='-mt-6'>
                <div>
                  <span
                    class='inline-flex items-center justify-center rounded-md bg-gradient-to-r from-teal-500 to-cyan-600 p-3 shadow-lg'>
                    <!-- Heroicon name: outline/cog -->
                    <svg class='h-6 w-6 text-white' xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 24 24'
                      stroke-width='1.5' stroke='currentColor' aria-hidden='true'>
                      <path stroke-linecap='round' stroke-linejoin='round'
                        d='M4.5 12a7.5 7.5 0 0015 0m-15 0a7.5 7.5 0 1115 0m-15 0H3m16.5 0H21m-1.5 0H12m-8.457 3.077l1.41-.513m14.095-5.13l1.41-.513M5.106 17.785l1.15-.964m11.49-9.642l1.149-.964M7.501 19.795l.75-1.3m7.5-12.99l.75-1.3m-6.063 16.658l.26-1.477m2.605-14.772l.26-1.477m0 17.726l-.26-1.477M10.698 4.614l-.26-1.477M16.5 19.794l-.75-1.299M7.5 4.205L12 12m6.894 5.785l-1.149-.964M6.256 7.178l-1.15-.964m15.352 8.864l-1.41-.513M4.954 9.435l-1.41-.514M12.002 12l-3.75 6.495' />
                    </svg>
                  </span>
                </div>
                <h3 class='mt-8 text-lg font-medium tracking-tight text-gray-900'>Powerful API</h3>
                <p class='mt-5 text-base text-gray-500'>Ab a facere voluptatem in quia corrupti veritatis aliquam.
                  Veritatis labore quaerat ipsum quaerat id.</p>
              </div>
            </div>
          </div>

          <div class='pt-6'>
            <div class='flow-root rounded-lg bg-gray-50 px-6 pb-8'>
              <div class='-mt-6'>
                <div>
                  <span
                    class='inline-flex items-center justify-center rounded-md bg-gradient-to-r from-teal-500 to-cyan-600 p-3 shadow-lg'>
                    <!-- Heroicon name: outline/server -->
                    <svg class='h-6 w-6 text-white' xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 24 24'
                      stroke-width='1.5' stroke='currentColor' aria-hidden='true'>
                      <path stroke-linecap='round' stroke-linejoin='round'
                        d='M21.75 17.25v-.228a4.5 4.5 0 00-.12-1.03l-2.268-9.64a3.375 3.375 0 00-3.285-2.602H7.923a3.375 3.375 0 00-3.285 2.602l-2.268 9.64a4.5 4.5 0 00-.12 1.03v.228m19.5 0a3 3 0 01-3 3H5.25a3 3 0 01-3-3m19.5 0a3 3 0 00-3-3H5.25a3 3 0 00-3 3m16.5 0h.008v.008h-.008v-.008zm-3 0h.008v.008h-.008v-.008z' />
                    </svg>
                  </span>
                </div>
                <h3 class='mt-8 text-lg font-medium tracking-tight text-gray-900'>Database Backups</h3>
                <p class='mt-5 text-base text-gray-500'>Quia qui et est officia cupiditate qui consectetur. Ratione
                  similique et impedit ea ipsum et.</p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>

  <!-- Testimonial section -->
  <div class='bg-gradient-to-r from-teal-500 to-cyan-600 pb-16 lg:relative lg:z-10 lg:pb-0'>
    <div class='lg:mx-auto lg:grid lg:max-w-7xl lg:grid-cols-3 lg:gap-8 lg:px-8'>
      <div class='relative lg:-my-8'>
        <div aria-hidden='true' class='absolute inset-x-0 top-0 h-1/2 bg-white lg:hidden'></div>
        <div class='mx-auto max-w-md px-6 sm:max-w-3xl lg:h-full lg:p-0'>
          <div
            class='aspect-w-10 aspect-h-6 overflow-hidden rounded-xl shadow-xl sm:aspect-w-16 sm:aspect-h-7 lg:aspect-none lg:h-full'>
            <img class='object-cover lg:h-full lg:w-full'
              src='https://images.unsplash.com/photo-1520333789090-1afc82db536a?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=crop&w=2102&q=80'
              alt=''>
          </div>
        </div>
      </div>
      <div class='mt-12 lg:col-span-2 lg:m-0 lg:pl-8'>
        <div class='mx-auto max-w-md px-6 sm:max-w-2xl lg:max-w-none lg:px-0 lg:py-20'>
          <blockquote>
            <div>
              <svg class='h-12 w-12 text-white opacity-25' fill='currentColor' viewBox='0 0 32 32' aria-hidden='true'>
                <path
                  d='M9.352 4C4.456 7.456 1 13.12 1 19.36c0 5.088 3.072 8.064 6.624 8.064 3.36 0 5.856-2.688 5.856-5.856 0-3.168-2.208-5.472-5.088-5.472-.576 0-1.344.096-1.536.192.48-3.264 3.552-7.104 6.624-9.024L9.352 4zm16.512 0c-4.8 3.456-8.256 9.12-8.256 15.36 0 5.088 3.072 8.064 6.624 8.064 3.264 0 5.856-2.688 5.856-5.856 0-3.168-2.304-5.472-5.184-5.472-.576 0-1.248.096-1.44.192.48-3.264 3.456-7.104 6.528-9.024L25.864 4z' />
              </svg>
              <p class='mt-6 text-2xl font-medium text-white'>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed
                urna nulla vitae laoreet augue. Amet feugiat est integer dolor auctor adipiscing nunc urna, sit.</p>
            </div>
            <footer class='mt-6'>
              <p class='text-base font-medium text-white'>Judith Black</p>
              <p class='text-base font-medium text-cyan-100'>CEO at PureInsights</p>
            </footer>
          </blockquote>
        </div>
      </div>
    </div>
  </div>

  <!-- Blog section -->
  <div class='relative bg-gray-50 py-16 sm:py-24 lg:py-32'>
    <div class='relative'>
      <div class='mx-auto max-w-md px-6 text-center sm:max-w-3xl lg:max-w-7xl lg:px-8'>
        <h2 class='text-lg font-semibold text-cyan-600'>Learn</h2>
        <p class='mt-2 text-3xl font-bold tracking-tight text-gray-900 sm:text-4xl'>Helpful Resources</p>
        <p class='mx-auto mt-5 max-w-prose text-xl text-gray-500'>Phasellus lorem quam molestie id quisque diam aenean
          nulla in. Accumsan in quis quis nunc, ullamcorper malesuada. Eleifend condimentum id viverra nulla.</p>
      </div>
      <div class='mx-auto mt-12 grid max-w-md gap-8 px-6 sm:max-w-lg lg:max-w-7xl lg:grid-cols-3 lg:px-8'>
        <div class='flex flex-col overflow-hidden rounded-lg shadow-lg'>
          <div class='flex-shrink-0'>
            <img class='h-48 w-full object-cover'
              src='https://images.unsplash.com/photo-1496128858413-b36217c2ce36?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=crop&w=1679&q=80'
              alt=''>
          </div>
          <div class='flex flex-1 flex-col justify-between bg-white p-6'>
            <div class='flex-1'>
              <p class='text-sm font-medium text-cyan-600'>
                <a href='#' class='hover:underline'>Article</a>
              </p>
              <a href='#' class='mt-2 block'>
                <p class='text-xl font-semibold text-gray-900'>Boost your conversion rate</p>
                <p class='mt-3 text-base text-gray-500'>Lorem ipsum dolor sit amet consectetur adipisicing elit.
                  Architecto accusantium praesentium eius, ut atque fuga culpa, similique sequi cum eos quis dolorum.</p>
              </a>
            </div>
            <div class='mt-6 flex items-center'>
              <div class='flex-shrink-0'>
                <a href='#'>
                  <img class='h-10 w-10 rounded-full'
                    src='https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80'
                    alt='Roel Aufderehar'>
                </a>
              </div>
              <div class='ml-3'>
                <p class='text-sm font-medium text-gray-900'>
                  <a href='#' class='hover:underline'>Roel Aufderehar</a>
                </p>
                <div class='flex space-x-1 text-sm text-gray-500'>
                  <time datetime='2020-03-16'>Mar 16, 2020</time>
                  <span aria-hidden='true'>&middot;</span>
                  <span>6 min read</span>
                </div>
              </div>
            </div>
          </div>
        </div>

        <div class='flex flex-col overflow-hidden rounded-lg shadow-lg'>
          <div class='flex-shrink-0'>
            <img class='h-48 w-full object-cover'
              src='https://images.unsplash.com/photo-1547586696-ea22b4d4235d?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=crop&w=1679&q=80'
              alt=''>
          </div>
          <div class='flex flex-1 flex-col justify-between bg-white p-6'>
            <div class='flex-1'>
              <p class='text-sm font-medium text-cyan-600'>
                <a href='#' class='hover:underline'>Video</a>
              </p>
              <a href='#' class='mt-2 block'>
                <p class='text-xl font-semibold text-gray-900'>How to use search engine optimization to drive sales</p>
                <p class='mt-3 text-base text-gray-500'>Lorem ipsum dolor sit amet consectetur adipisicing elit. Velit
                  facilis asperiores porro quaerat doloribus, eveniet dolore. Adipisci tempora aut inventore optio animi.,
                  tempore temporibus quo laudantium.</p>
              </a>
            </div>
            <div class='mt-6 flex items-center'>
              <div class='flex-shrink-0'>
                <a href='#'>
                  <img class='h-10 w-10 rounded-full'
                    src='https://images.unsplash.com/photo-1550525811-e5869dd03032?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80'
                    alt='Brenna Goyette'>
                </a>
              </div>
              <div class='ml-3'>
                <p class='text-sm font-medium text-gray-900'>
                  <a href='#' class='hover:underline'>Brenna Goyette</a>
                </p>
                <div class='flex space-x-1 text-sm text-gray-500'>
                  <time datetime='2020-03-10'>Mar 10, 2020</time>
                  <span aria-hidden='true'>&middot;</span>
                  <span>4 min read</span>
                </div>
              </div>
            </div>
          </div>
        </div>

        <div class='flex flex-col overflow-hidden rounded-lg shadow-lg'>
          <div class='flex-shrink-0'>
            <img class='h-48 w-full object-cover'
              src='https://images.unsplash.com/photo-1492724441997-5dc865305da7?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=crop&w=1679&q=80'
              alt=''>
          </div>
          <div class='flex flex-1 flex-col justify-between bg-white p-6'>
            <div class='flex-1'>
              <p class='text-sm font-medium text-cyan-600'>
                <a href='#' class='hover:underline'>Case Study</a>
              </p>
              <a href='#' class='mt-2 block'>
                <p class='text-xl font-semibold text-gray-900'>Improve your customer experience</p>
                <p class='mt-3 text-base text-gray-500'>Lorem ipsum dolor sit amet consectetur adipisicing elit. Sint
                  harum rerum voluptatem quo recusandae magni placeat saepe molestiae, sed excepturi cumque corporis
                  perferendis hic.</p>
              </a>
            </div>
            <div class='mt-6 flex items-center'>
              <div class='flex-shrink-0'>
                <a href='#'>
                  <img class='h-10 w-10 rounded-full'
                    src='https://images.unsplash.com/photo-1487412720507-e7ab37603c6f?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80'
                    alt='Daniela Metz'>
                </a>
              </div>
              <div class='ml-3'>
                <p class='text-sm font-medium text-gray-900'>
                  <a href='#' class='hover:underline'>Daniela Metz</a>
                </p>
                <div class='flex space-x-1 text-sm text-gray-500'>
                  <time datetime='2020-02-12'>Feb 12, 2020</time>
                  <span aria-hidden='true'>&middot;</span>
                  <span>11 min read</span>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>

  <!-- CTA Section -->
  <div class='relative bg-gray-900'>
    <div class='relative h-56 bg-indigo-600 sm:h-72 md:absolute md:left-0 md:h-full md:w-1/2'>
      <img class='h-full w-full object-cover'
        src='https://images.unsplash.com/photo-1525130413817-d45c1d127c42?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=crop&w=1920&q=60&sat=-100'
        alt=''>
      <div aria-hidden='true' class='absolute inset-0 bg-gradient-to-r from-teal-500 to-cyan-600 mix-blend-multiply'>
      </div>
    </div>
    <div class='relative mx-auto max-w-md py-12 px-6 sm:max-w-7xl sm:py-20 md:py-28 lg:px-8 lg:py-32'>
      <div class='md:ml-auto md:w-1/2 md:pl-10'>
        <h2 class='text-lg font-semibold text-gray-300'>Award winning support</h2>
        <p class='mt-2 text-3xl font-bold tracking-tight text-white sm:text-4xl'>We\xE2\x80\x99re here to help</p>
        <p class='mt-3 text-lg text-gray-300'>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Et, egestas tempus
          tellus etiam sed. Quam a scelerisque amet ullamcorper eu enim et fermentum, augue. Aliquet amet volutpat quisque
          ut interdum tincidunt duis.</p>
        <div class='mt-8'>
          <div class='inline-flex rounded-md shadow'>
            <a href='#'
              class='inline-flex items-center justify-center rounded-md border border-transparent bg-white px-5 py-3 text-base font-medium text-gray-900 hover:bg-gray-50'>
              Visit the help center
              <!-- Heroicon name: mini/arrow-top-right-on-square -->
              <svg class='-mr-1 ml-3 h-5 w-5 text-gray-400' xmlns='http://www.w3.org/2000/svg' viewBox='0 0 20 20'
                fill='currentColor' aria-hidden='true'>
                <path fill-rule='evenodd'
                  d='M4.25 5.5a.75.75 0 00-.75.75v8.5c0 .414.336.75.75.75h8.5a.75.75 0 00.75-.75v-4a.75.75 0 011.5 0v4A2.25 2.25 0 0112.75 17h-8.5A2.25 2.25 0 012 14.75v-8.5A2.25 2.25 0 014.25 4h5a.75.75 0 010 1.5h-5z'
                  clip-rule='evenodd' />
                <path fill-rule='evenodd'
                  d='M6.194 12.753a.75.75 0 001.06.053L16.5 4.44v2.81a.75.75 0 001.5 0v-4.5a.75.75 0 00-.75-.75h-4.5a.75.75 0 000 1.5h2.553l-9.056 8.194a.75.75 0 00-.053 1.06z'
                  clip-rule='evenodd' />
              </svg>
            </a>
          </div>
        </div>
      </div>
    </div>
  </div>
</main>
{% endblock content %}",)?;
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
    // Write to index route
    fn write_to_index_route(&self) -> Result<(), Error> {
        let contents = format!(
            r#"use rocket::fs::{{relative, FileServer}};
use rocket_dyn_templates::{{context, Template}};

#[get("/")]
pub fn index() -> Template {{
    Template::render(
        "pages/index",
        context! {{
            foo: 123,
        }},
    )
}}"#
        );

        write_to_file(&self.index_route.to_string(), contents.as_bytes()).unwrap_or_else(|why| {
            println!(
                "Couldn't write to {}: {}",
                self.index_route.to_string(),
                why.to_string()
            );
        });
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
        Self::write_to_cargo_toml(&project, &database_data).expect("Failed to write to cargo.toml");

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
        Self::write_to_index_html(&project).unwrap_or_else(|why| {
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
        Self::write_to_index_route(&project).unwrap_or_else(|why| {
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
