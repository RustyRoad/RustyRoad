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

use std::io::Write;

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

/** Fast and easy queue abstraction. **/

/** Provides an abstraction over a queue.  When the abstraction is used
 there are these advantages:
- Fast
 - [`Easy`]

 [`Easy`]: http://thatwaseasy.example.com
**/

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
    config_database: String,
    config_dev_db: String,
    config_prod_db: String,
    routes: String,
    controllers: String,
    models: String,
    migrations: String,
    seeders: String,
    tests: String,
    config_initializers: String,
    config_initializers_assets: String,
    config_initializers_db: String,
    config_initializers_default: String,
    index_html: String,
    styles_css: String,
    not_found_html: String,
    server_error_html: String,
    favicon_ico: String,
    robots_txt: String,
    login_page_html: String,
    signup_page_html: String,
    reset_password_page_html: String,
    forgot_password_page_html: String,
    dashboard_page_html: String,
    user_controller: String,
    user_model: String,
    user_migration: String,
    user_seeder: String,
    user_test: String,
    user_routes: String,
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
    pub fn new(name: String) -> Project {
        let name = name.trim().to_string();
        let src_dir = format!("{}/src", name);
        let cargo_toml = format!("{}/Cargo.toml", name);
        let main_rs = format!("{}/src/main.rs", name);
        let package_json = format!("{}/package.json", name);
        let readme = format!("{}/README.md", name);
        let gitignore = format!("{}/.gitignore", name);
        let templates = format!("{}/templates", name);
        let static_dir = format!("{}/static", name);
        let template_components = format!("{}/templates/components", name);
        let template_layouts = format!("{}/templates/layouts", name);
        let template_pages = format!("{}/templates/pages", name);
        let static_css = format!("{}/static/css", name);
        let static_js = format!("{}/static/js", name);
        let index_js = format!("{}/static/js/index.js", name);
        let static_images = format!("{}/static/images", name);
        let config = format!("{}/config", name);
        let config_env = format!("{}/config/environments", name);
        let config_dev_env = format!("{}/config/environments/dev.env", name);
        let config_prod_env = format!("{}/config/environments/prod.env", name);
        let config_test_env = format!("{}/config/environments/test.env", name);
        let config_default_env = format!("{}/config/environments/default.env", name);
        let config_database = format!("{}/config/database", name);
        let config_dev_db = format!("{}/config/database/dev.db", name);
        let config_prod_db = format!("{}/config/database/prod.db", name);
        let routes = format!("{}/routes", name);
        let controllers = format!("{}/controllers", name);
        let models = format!("{}/models", name);
        let migrations = format!("{}/migrations", name);
        let seeders = format!("{}/seeders", name);
        let tests = format!("{}/tests", name);
        let config_initializers = format!("{}/config/initializers", name);
        let config_initializers_assets = format!("{}/config/initializers/assets.rs", name);
        let config_initializers_db = format!("{}/config/initializers/db.rs", name);
        let config_initializers_default = format!("{}/config/initializers/default.rs", name);
        let index_html = format!("{}/templates/pages/index.html.tera", name);
        let styles_css = format!("{}/static/css/styles.css", name);
        let not_found_html = format!("{}/templates/pages/404.html.tera", name);
        let server_error_html = format!("{}/templates/pages/500.html.tera", name);
        let favicon_ico = format!("{}/static/images/favicon.ico", name);
        let robots_txt = format!("{}/static/robots.txt", name);
        let login_page_html = format!("{}/templates/pages/login.html.tera", name);
        let signup_page_html = format!("{}/templates/pages/signup.html.tera", name);
        let reset_password_page_html = format!("{}/templates/pages/reset_password.html.tera", name);
        let forgot_password_page_html =
            format!("{}/templates/pages/forgot_password.html.tera", name);
        let dashboard_page_html = format!("{}/templates/pages/dashboard.html.tera", name);
        let user_controller = format!("{}/controllers/user.rs", name);
        let user_model = format!("{}/models/user.rs", name);
        let user_migration = format!("{}/migrations/00000000000000_create_users_table.rs", name);
        let user_seeder = format!("{}/seeders/00000000000000_create_users_table.rs", name);
        let user_test = format!("{}/tests/user.rs", name);
        let user_routes = format!("{}/routes/user.rs", name);

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
            config_database,
            config_dev_db,
            config_prod_db,
            routes,
            controllers,
            models,
            migrations,
            seeders,
            tests,
            config_initializers,
            config_initializers_assets,
            config_initializers_db,
            config_initializers_default,
            index_html,
            styles_css,
            not_found_html,
            server_error_html,
            favicon_ico,
            robots_txt,
            login_page_html,
            signup_page_html,
            reset_password_page_html,
            forgot_password_page_html,
            dashboard_page_html,
            user_controller,
            user_model,
            user_migration,
            user_seeder,
            user_test,
            user_routes,
        }
    }

    pub fn create_directories(&self) -> Result<()> {
        std::fs::create_dir(&self.name).expect("Failed to create directory");
        std::fs::create_dir(&self.src_dir).expect("Failed to create src directory");
std::fs::create_dir(&self.templates).expect("Failed to create templates directory");
        std::fs::create_dir(&self.static_dir).expect("Failed to create static directory");
        std::fs::create_dir(&self.template_components)
            .expect("Failed to create template components directory");
        std::fs::create_dir(&self.template_layouts)
            .expect("Failed to create template layouts directory");
        std::fs::create_dir(&self.template_pages)
            .expect("Failed to create template pages directory");
        std::fs::create_dir(&self.static_css).expect("Failed to create static css directory");
        std::fs::create_dir(&self.static_js).expect("Failed to create static js directory");
        std::fs::create_dir(&self.static_images)
            .expect("Failed to create static images directory");
        std::fs::create_dir(&self.config).expect("Failed to create config directory");
        std::fs::create_dir(&self.config_env).expect("Failed to create config environments directory");
        std::fs::create_dir(&self.config_database)
            .expect("Failed to create config database directory");
        std::fs::create_dir(&self.routes).expect("Failed to create routes directory");
        std::fs::create_dir(&self.controllers).expect("Failed to create controllers directory");
        std::fs::create_dir(&self.models).expect("Failed to create models directory");
        std::fs::create_dir(&self.migrations).expect("Failed to create migrations directory");
        std::fs::create_dir(&self.seeders).expect("Failed to create seeders directory");
        std::fs::create_dir(&self.tests).expect("Failed to create tests directory");
        std::fs::create_dir(&self.config_initializers)
            .expect("Failed to create config initializers directory");
        std::fs::create_dir(&self.user_controller).expect("Failed to create user controller directory");
        std::fs::create_dir(&self.user_model).expect("Failed to create user model directory");
        std::fs::create_dir(&self.user_migration)
            .expect("Failed to create user migration directory");
        std::fs::create_dir(&self.user_seeder).expect("Failed to create user seeder directory");
        std::fs::create_dir(&self.user_test).expect("Failed to create user test directory");
        std::fs::create_dir(&self.user_routes).expect("Failed to create user routes directory");
        Ok(())
    }

    pub fn create_files(&self) -> Result<()> {
        std::fs::File::create(&self.cargo_toml).expect("Failed to create Cargo.toml");
        std::fs::File::create(&self.main_rs).expect("Failed to create main.rs");
        std::fs::File::create(&self.package_json).expect("Failed to create package.json");
        std::fs::File::create(&self.readme).expect("Failed to create README.md");
        std::fs::File::create(&self.gitignore).expect("Failed to create .gitignore");
        std::fs::File::create(&self.index_html).expect("Failed to create index.html.tera");
        std::fs::File::create(&self.styles_css).expect("Failed to create styles.css");
        std::fs::File::create(&self.not_found_html).expect("Failed to create 404.html.tera");
        std::fs::File::create(&self.server_error_html).expect("Failed to create 500.html.tera");
        std::fs::File::create(&self.favicon_ico).expect("Failed to create favicon.ico");
        std::fs::File::create(&self.robots_txt).expect("Failed to create robots.txt");
        std::fs::File::create(&self.login_page_html).expect("Failed to create login.html.tera");
        std::fs::File::create(&self.signup_page_html).expect("Failed to create signup.html.tera");
        std::fs::File::create(&self.reset_password_page_html)
            .expect("Failed to create reset_password.html.tera");
        std::fs::File::create(&self.forgot_password_page_html)
            .expect("Failed to create forgot_password.html.tera");
        std::fs::File::create(&self.dashboard_page_html)
            .expect("Failed to create dashboard.html.tera");
        std::fs::File::create(&self.user_controller).expect("Failed to create user.rs");
        std::fs::File::create(&self.user_model).expect("Failed to create user.rs");
        std::fs::File::create(&self.user_migration).expect("Failed to create user.rs");
        std::fs::File::create(&self.user_seeder).expect("Failed to create user.rs");
        std::fs::File::create(&self.user_test).expect("Failed to create user.rs");
        std::fs::File::create(&self.user_routes).expect("Failed to create user.rs");
        std::fs::File::create(&self.config_dev_env).expect("Failed to create dev.env");
        std::fs::File::create(&self.config_prod_env).expect("Failed to create prod.env");
        std::fs::File::create(&self.config_test_env).expect("Failed to create test.env");
        std::fs::File::create(&self.config_dev_db).expect("Failed to create dev.db");
        std::fs::File::create(&self.config_prod_db).expect("Failed to create prod.db");
        std::fs::File::create(&self.config_test_db).expect("Failed to create test.db");
        std::fs::File::create(&self.config_initializers_db)
            .expect("Failed to create db.rs");
        std::fs::File::create(&self.config_initializers_routes)
            .expect("Failed to create routes.rs");
        Ok(())
    }

    fn write_to_main_rs(&self) -> Result<()> {
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(&self.main_rs)
            .expect("Failed to open main.rs");

        file.write_all(
            b"
#[macro_use] extern crate rocket;

#[get(\"/\")]
fn index() -> &'static str {
    \"Hello, world!\"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount(\"/\", routes![index])
}",
        )
        .expect("Failed to write to main.rs");
        Ok(())
    }

    fn write_to_toml(&self) -> Result<()> {
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(&self.cargo_toml)
            .expect("Failed to open Cargo.toml");

        file.write_all(
            format!(
                "[package]
name = \"{}\"
version = \"0.1.0\"
authors = [\"RustyRocket\"]
edition = \"2021\"
[dependencies]
rocket = \"0.5.0-rc.1\"",
                self.name
            )
            .as_bytes(),
        )
        .expect("Failed to write to Cargo.toml");
        Ok(())
    }

    fn write_to_package_json(&self) -> Result<()> {
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(format!("{}/package.json", &self.package_json))
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
    fn write_to_readme(&self) -> Result<()> {
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(format!("{}/README.md", &self.readme))
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
    fn write_to_gitignore(&self) -> Result<()> {
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(format!("{}/.gitignore", &self.gitignore))
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
    fn write_to_indexjs(&self) -> Result<()> {
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(format!("{}/index.js", &self.indexjs))
            .expect("Failed to open index.js");

        file.write_all(
            format!(
                "// Rusty Roadster
class RustyRoadster {{
    constructor() {{
        this.name = \"{}\";
function greet() {{
    console.log(\"Welcome to {} powered by Rusty Roadster\");
}}
    }}
}}

const rustyroadster = new RustyRoadster();

rustyroadster.greet();
",
                self.name, self.name
            )
            .as_bytes(),
        )
        .expect("Failed to write to index.js");

        Ok(())
    }

    // Write to dev.env
    fn write_to_dev_dot_env(&self) -> Result<()> {
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(format!("{}/dev.env", &self.config_dev_env))
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
    fn write_to_prod_dot_env(&self) -> Result<()> {
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(format!("{}/prod.env", &self.config_prod_env))
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

    fn create_new_project() {
        println!("What would you like to name your project?");

        let mut project_name_user_input = String::new();

        // Get user input and assign it to project_name_user_input
        std::io::stdin()
            .read_line(&mut project_name_user_input)
            .expect("Failed to read line");
        // Place the user input as a value into the new() function
        let project = Self::new(project_name_user_input);

        writeln!(std::io::stdout(), "Creating project {}", project.name).unwrap();

        // Create the project directory
        Self::create_directories(&project).expect("Failed to create directories");

        // Create the files
        Self::create_files(&project).expect("Failed to create files");

        // Write to the cargo.toml file
        Self::write_to_toml(&project).expect("Failed to write to Cargo.toml");

        // Write to main.rs file
        Self::write_to_main_rs(&project).expect("Failed to write to main.rs");

        // Write to package.json file
        Self::write_to_package_json(&project).expect("Failed to write to package.json");

        // Write to README.md file
        Self::write_to_readme(&project).expect("Failed to write to README.md");

        println!("Project {} created!", &project.name);
    }

    fn create_new_route() {
        println!("What would you like to name your route?");
    }

    pub fn initial_prompt() -> Result<(), Box<dyn std::error::Error>> {
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
            1 => Ok(Self::create_new_project()),
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
}
