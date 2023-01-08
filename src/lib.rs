// Libray Crate

// RustyRocket

// This is the main file for the RustyRocket project.
// It is the entry point for the program.

use std::io::Write;

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

pub struct Project {
    name: String,
    src_dir: String,
    cargo_toml: String,
    main_rs: String,
}

impl Project {
    pub fn new(name: String) -> Project {
        let name = name.trim().to_string();
        let src_dir = format!("{}/src", name);
        let cargo_toml = format!("{}/Cargo.toml", name);
        let main_rs = format!("{}/src/main.rs", name);

        Project {
            name,
            src_dir,
            cargo_toml,
            main_rs,
        }
    }

    pub fn create_directories(&self) -> Result<()> {
        std::fs::create_dir(&self.name).expect("Failed to create directory");
        std::fs::create_dir(&self.src_dir).expect("Failed to create src directory");
        Ok(())
    }

    pub fn create_files(&self) -> Result<()> {
        std::fs::File::create(&self.cargo_toml).expect("Failed to create Cargo.toml");
        std::fs::File::create(&self.main_rs).expect("Failed to create main.rs");
        // create package.json
        std::fs::File::create(format!("{}/package.json", &self.name))
            .expect("Failed to create package.json");
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
            .open(format!("{}/package.json", &self.name))
            .expect("Failed to open package.json");

        file.write_all(
            format!(
                "{{
  \"name\": \"RustyRocket\",
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

        // Create a main.rs file
        Self::write_to_main_rs(&project).expect("Failed to write to main.rs");

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
