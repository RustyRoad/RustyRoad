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

use clap::{arg, Arg, Command, Parser};
use std::io::Error;
use std::{fs::OpenOptions, io::Write};

pub(crate) mod generators;
pub(crate) mod writers;
use crate::generators::create_directory;
use crate::generators::create_file;
use crate::writers::templates::navbar::write_to_navbar;
use crate::writers::{write_to_file, write_to_main_rs};

/** Fast and easy queue abstraction. **/

/** Provides an abstraction over a queue.  When the abstraction is used
 there are these advantages:
- Fast
 - [`Easy`]

 [`Easy`]: http://thatwaseasy.example.com
**/
#[derive(Parser, Debug)]
pub struct Project {
    name: String,
    src_dir: String,
    cargo_toml: String,
    main_rs: String,
    package_json: String,
    readme: String,
    gitignore: String,
    templates: String,
    static_dir: String,
    template_components: String,
    template_layouts: String,
    template_pages: String,
    static_css: String,
    static_js: String,
    index_js: String,
    static_images: String,
    config: String,
    config_env: String,
    config_dev_env: String,
    config_prod_env: String,
    config_test_env: String,
    config_default_env: String,
    db: String,
    config_dev_db: String,
    config_prod_db: String,
    config_test_db: String,
    routes: String,
    routes_module: String,
    controllers: String,
    models: String,
    models_module: String,
    migrations: String,
    seeders: String,
    tests: String,
    config_initializers: String,
    config_initializers_assets: String,
    config_initializers_db: String,
    config_initializers_default: String,
    config_initializers_middleware: String,
    config_initializers_routes: String,
    index_html: String,
    base_html: String,
    tailwind_css: String,
    tailwind_config: String,
    postcss_config: String,
    not_found_html: String,
    server_error_html: String,
    favicon_ico: String,
    robots_txt: String,
    login_page_html: String,
    signup_page_html: String,
    reset_password_page_html: String,
    forgot_password_page_html: String,
    dashboard_page_html: String,
    user_controller_directory: String,
    user_controller: String,
    user_controller_module: String,
    user_model_directory: String,
    user_model: String,
    user_model_module: String,
    user_migration_directory: String,
    user_migration: String,
    user_migration_module: String,
    user_seeder: String,
    user_test: String,
    user_route: String,
    index_route: String,
    login_route: String,
    signup_route: String,
    reset_password_route: String,
    forgot_password_route: String,
    dashboard_route: String,
    navbar_component: String,
}

/// # RustyRocket Project Builder
/// Description: This is the main file for the RustyRocket project.
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
    /// Takes a name and a path as arguments.
    /// The {name} is the name of the project.
    /// The {path} is the path to the directory where the project will be created.
    /// If a path is not provided, the project will be created in the current directory.
    pub fn new(name: String) -> Project {
        let src_dir = format!("{}/src", name);
        let cargo_toml = format!("{}/Cargo.toml", name);
        let main_rs = format!("{}/main.rs", src_dir);
        let package_json = format!("{}/package.json", name);
        let readme = format!("{}/README.md", name);
        let gitignore = format!("{}/.gitignore", name);
        let templates = format!("{}/templates", name);
        let static_dir = format!("{}/static", name);
        let template_components = format!("{}/components", templates);
        let template_layouts = format!("{}/layouts", templates);
        let template_pages = format!("{}/pages", templates);
        let static_css = format!("{}/css", static_dir);
        let static_js = format!("{}/js", static_dir);
        let index_js = format!("{}/index.js", static_js);
        let static_images = format!("{}/images", static_dir);
        let config = format!("{}/config", name);
        let config_env = format!("{}/environments", config);
        let config_dev_env = format!("{}/dev.env", config_env);
        let config_prod_env = format!("{}/prod.env", config_env);
        let config_test_env = format!("{}/test.env", config_env);
        let config_default_env = format!("{}/default.env", config_env);
        let db = format!("{}/database", config);
        let config_dev_db = format!("{}/dev.db", db);
        let config_prod_db = format!("{}/prod.db", db);
        let config_test_db = format!("{}/test.db", db);
        let routes = format!("{}/routes", src_dir);
        let routes_module = format!("{}/components", routes);
        let controllers = format!("{}/controllers", src_dir);
        let models = format!("{}/models", src_dir);
        let models_module = format!("{}/components", models);
        let migrations = format!("{}/migrations", db);
        let seeders = format!("{}/seeders", name);
        let tests = format!("{}/tests", name);
        let config_initializers = format!("{}/initializers", config);
        let config_initializers_assets = format!("{}/assets.rs", config_initializers);
        let config_initializers_db = format!("{}/db.rs", config_initializers);
        let config_initializers_default = format!("{}/default.rs", config_initializers);
        let config_initializers_middleware = format!("{}/middleware.rs", config_initializers);
        let config_initializers_routes = format!("{}/routes.rs", config_initializers);
        let index_html = format!("{}/index.html.tera", template_pages);
        let base_html = format!("{}/base.html.tera", templates);
        let tailwind_css = format!("{}/tailwind.css", src_dir);
        let tailwind_config = format!("{}/tailwind.config.js", name);
        let postcss_config = format!("{}/postcss.config.js", name);
        let not_found_html = format!("{}/404.html.tera", template_pages);
        let server_error_html = format!("{}/500.html.tera", template_pages);
        let favicon_ico = format!("{}/favicon.ico", static_images);
        let robots_txt = format!("{}/robots.txt", static_dir);
        let login_page_html = format!("{}/login.html.tera", template_pages);
        let signup_page_html = format!("{}/signup.html.tera", template_pages);
        let reset_password_page_html = format!("{}/reset_password.html.tera", template_pages);
        let forgot_password_page_html = format!("{}/forgot_password.html.tera", template_pages);
        let dashboard_page_html = format!("{}/dashboard.html.tera", template_pages);
        let user_controller_directory = format!("{}/user", controllers);
        let user_controller = format!("{}/user.rs", user_controller_directory);
        let user_controller_module = format!("{}/components", user_controller_directory);
        let user_model_directory = format!("{}/user", models);
        let user_model = format!("{}/user.rs", user_model_directory);
        let user_model_module = format!("{}/components", user_model_directory);
        let user_migration_directory = format!("{}/user", migrations);
        let user_migration = format!(
            "{}/00000000000000_create_users_table.rs",
            user_migration_directory
        );
        let user_migration_module = format!("{}/components", user_migration_directory);
        let user_seeder = format!("{}/seeders/00000000000000_create_users_table.rs", name);
        let user_test = format!("{}/tests/user.rs", name);
        let user_route = format!("{}/user.rs", routes);
        let index_route = format!("{}/index.rs", routes);
        let login_route = format!("{}/login.rs", routes);
        let signup_route = format!("{}/signup.rs", routes);
        let reset_password_route = format!("{}/reset_password.rs", routes);
        let forgot_password_route = format!("{}/forgot_password.rs", routes);
        let dashboard_route = format!("{}/dashboard.rs", routes);
        let navbar_component = format!("{}/navbar.html.tera", template_components);

        Project {
            name,
            src_dir,
            cargo_toml,
            main_rs,
            package_json,
            readme,
            gitignore,
            templates,
            static_dir,
            template_components,
            template_layouts,
            template_pages,
            static_css,
            static_js,
            index_js,
            static_images,
            config,
            config_env,
            config_dev_env,
            config_prod_env,
            config_test_env,
            config_default_env,
            db,
            config_dev_db,
            config_prod_db,
            config_test_db,
            routes,
            routes_module,
            controllers,
            models,
            models_module,
            migrations,
            seeders,
            tests,
            config_initializers,
            config_initializers_assets,
            config_initializers_db,
            config_initializers_default,
            config_initializers_middleware,
            config_initializers_routes,
            index_html,
            base_html,
            tailwind_css,
            tailwind_config,
            postcss_config,
            not_found_html,
            server_error_html,
            favicon_ico,
            robots_txt,
            login_page_html,
            signup_page_html,
            reset_password_page_html,
            forgot_password_page_html,
            dashboard_page_html,
            user_controller_directory,
            user_controller,
            user_controller_module,
            user_model_directory,
            user_model,
            user_model_module,
            user_migration_directory,
            user_migration,
            user_migration_module,
            user_seeder,
            user_test,
            user_route,
            index_route,
            login_route,
            signup_route,
            reset_password_route,
            forgot_password_route,
            dashboard_route,
            navbar_component,
        }
    }

    pub fn create_files(&self) -> Result<(), Error> {
        let files = vec![
            &self.cargo_toml,
            &self.main_rs,
            &self.package_json,
            &self.readme,
            &self.gitignore,
            &self.config_dev_env,
            &self.config_prod_env,
            &self.config_test_env,
            &self.config_default_env,
            &self.config_dev_db,
            &self.config_prod_db,
            &self.config_test_db,
            &self.routes_module,
            &self.models_module,
            &self.user_controller_module,
            &self.user_model_module,
            &self.user_migration_module,
            &self.index_html,
            &self.base_html,
            &self.tailwind_css,
            &self.tailwind_config,
            &self.postcss_config,
            &self.not_found_html,
            &self.server_error_html,
            &self.favicon_ico,
            &self.robots_txt,
            &self.login_page_html,
            &self.signup_page_html,
            &self.reset_password_page_html,
            &self.forgot_password_page_html,
            &self.dashboard_page_html,
            &self.user_controller,
            &self.user_model,
            &self.user_migration,
            &self.user_seeder,
            &self.user_test,
            &self.user_route,
            &self.index_route,
            &self.login_route,
            &self.signup_route,
            &self.reset_password_route,
            &self.forgot_password_route,
            &self.dashboard_route,
            &self.index_js,
        ];

        for file in files {
            create_file(file)?;
        }

        Ok(())
    }
    // Write to cargo_toml
    fn write_to_cargo_toml(&self) -> Result<(), Error> {
        let config = format!(
            "[package]
name = \"{}\"
version = \"0.1.0\"
authors = [\"RustyRocket\"]
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


[dependencies.sqlx]
version = \"0.5\"
default-features = false
features = [\"macros\", \"offline\", \"migrate\"]

[dependencies.rocket_db_pools]
version = \"0.1.0-rc.2\"
features = [\"sqlx_sqlite\"]",
            &self.name,
        );

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.cargo_toml)?;

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
            format!(
                "{{
  \"name\": \"rustyrocket\",
  \"version\": \"1.0.0\",
  \"main\": \"index.js\",
  \"repository\": \"https://github.com/Riley-Seaburg/RustyRocket.git\",
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
            )
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
This project was created using Rusty Roadster. Rusty Roadster is Rails for Rust. It is a CLI tool that allows you to create a new Rust project with a few commands. It also comes with TailwindCSS and Rocket pre-installed.

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
  {% include 'sections/feature-section-w-screenshot' %}

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
    // Write to routes/components
    fn write_to_routes_module(&self) -> Result<(), Error> {
        let contents = format!(
            "pub mod index;
pub mod user;

pub use index::*;
pub use user::*;"
        );

        write_to_file(&self.routes_module, contents.as_bytes()).unwrap_or_else(|why| {
            panic!(
                "couldn't write to {}: {}",
                &self.routes_module,
                why.to_string()
            )
        });

        Ok(())
    }
    // Write to models/components
    fn write_models_mod_rs(&self) -> Result<(), Error> {
        let contents = format!(
            "pub mod users;
        pub use users::*;"
        );

        write_to_file(&self.models_module, &contents.as_bytes()).unwrap_or_else(|why| {
            panic!(
                "couldn't create {}: {}",
                self.models_module,
                why.to_string()
            )
        });
        Ok(())
    }
    // Write to models/users.rs
    fn write_models_users_rs(&self) -> Result<(), Error> {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open("src/models/users.rs")?;
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
    // Write to base.html.tera
    fn write_to_base_html(&self) -> Result<(), Error> {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(&self.base_html)
            .expect("Failed to open index.html");
        file.write_all(
            b"<!DOCTYPE html>
<html class='bg-gray-50 h-full' lang='en'>

  <head>
    {% block head %}
    {% include 'sections/header' ignore missing %}
    {% endblock head %}
  </head>

  <body id='app' class='h-full'>
    {% include 'sections/components/navbar'%}
    <div id='content'>{% block content %}{% endblock content %}</div>
    <div id='footer'>
      {% block footer %}
      {% include 'sections/footer' ignore missing %}
      {% endblock footer %}
    </div>
  </body>
  <script src='/js/index.js'></script>
</html>",
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
    /// Takes an optional name <String> as a parameter
    /// If no name is provided, it will default to "rustyroad"
    /// If a name is provided, it will create a new directory with that name
    /// and create a new project in that directory
    /// If a directory with the same name already exists, it will return an error
    /// and ask the user to choose a different name
    pub fn create_new_project(name: String) -> Result<(), Error> {
        // If name is provided, create a new directory with that name
        // If no name is provided, run the rest of the code in the function

        // Create new project with name
        let project = Self::new(name);

        // Create the project directory
        create_directory(&project).unwrap_or_else(|why| {
            println!("Couldn't create directory: {:?}", why.kind());
        });

        // Create the files
        Self::create_files(&project).expect("Failed to create files");

        // Write to the cargo.toml file
        Self::write_to_cargo_toml(&project).expect("Failed to write to Cargo.toml");

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
        Self::write_to_base_html(&project).unwrap_or_else(|why| {
            println!("Failed to write to base.html.tera: {:?}", why.kind());
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
        // Write to routes module
        Self::write_to_routes_module(&project).unwrap_or_else(|why| {
            println!("Failed to write to routes/components: {:?}", why.kind());
        });

        // write to navbar
        write_to_navbar(&project).unwrap_or_else(|why| {
            println!("Failed to write to navbar: {:?}", why.kind());
        });

        println!("Project {} created!", &project.name);

        Ok(())
    } // End of create_new_project function

    fn create_new_route() {
        println!("What would you like to name your route?");
    }

    pub fn initial_prompt() -> Result<(), Box<Error>> {
        println!("What would you like to do?");
        println!("1. Create a new project");
        println!("2. Create a route");
        println!("3. CLI help");
        println!("4. Exit");

        let mut project_name = String::new();

        std::io::stdin()
            .read_line(&mut project_name)
            .expect("Failed to read line");

        let project_name: u32 = match project_name.trim().parse() {
            Ok(num) => num,
            Err(_) => 0,
        };

        match project_name {
            1 => Ok(Self::create_new_project(String::from("rustyroad"))?),
            2 => Ok(Self::create_new_route()),
            3 => Ok(println!("Helping you...")),
            // print exit message then exit the program
            4 => Ok(Self::exit_program()),
            _ => Ok(println!("Invalid project_name")),
        }
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
                Command::new("push")
                    .args_conflicts_with_subcommands(true)
                    .args(Self::push_args())
                    .subcommand(Command::new("push").args(Self::push_args()))
                    .subcommand(Command::new("pop").arg(arg!([STASH])))
                    .subcommand(Command::new("apply").arg(arg!([STASH]))),
            )
    }

    pub fn push_args() -> Vec<Arg> {
        vec![arg!(-m --message <MESSAGE>)]
    }

    pub fn run() {
        let matches = Self::cli().get_matches();

        match matches.subcommand() {
            Some(("new", matches)) => {
                let name = matches.get_one::<String>("name").unwrap().to_string();
                match Self::create_new_project(name) {
                    Ok(_) => println!("Project created!"),
                    Err(e) => println!("Error: {}", e),
                }
            }
            Some(("push", matches)) => {
                let message = matches.get_one::<String>("message").unwrap().to_string();
                Self::create_new_route()
            }
            _ => unreachable!(),
        }
    }
}
