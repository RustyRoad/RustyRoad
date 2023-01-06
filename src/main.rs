// CrabbyRails

// This is the main file for the CrabbyRails project.
// It is the entry point for the program.

use std::io::Write;

fn greet_user() {
    println!("What would you like to do?");
    println!("1. Create a new project");
    println!("2. CLI help");
    println!("3. Exit");

    let mut project_name = String::new();

    std::io::stdin()
        .read_line(&mut project_name)
        .expect("Failed to read line");

    let project_name: u32 = match project_name.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    match project_name {
        1 => create_new_project(),
        2 => println!("Helping you..."),
        3 => println!("Exiting..."),
        _ => println!("Invalid project_name"),
    }
}

fn create_new_project() {
    println!("What would you like to name your project?");
    let mut project_name = String::new();

    std::io::stdin()
        .read_line(&mut project_name)
        .expect("Failed to read line");

    let project_name = project_name.trim();

    println!("Creating project {}...", project_name);

    // Create the project directory
    std::fs::create_dir(project_name).expect("Failed to create directory");

    // Create a cargo.toml file
    let cargo_toml = format!("{}/Cargo.toml", project_name);
    std::fs::File::create(&cargo_toml).expect("Failed to create Cargo.toml");

    // Create a src directory
    let src_dir = format!("{}/src", project_name);
    std::fs::create_dir(src_dir).expect("Failed to create src directory");

    // Create a main.rs file
    let main_rs = format!("{}/src/main.rs", project_name);
    std::fs::File::create(&main_rs).expect("Failed to create main.rs");

    // Start a rocket web server in main.rs
    // clone main.rs to a new variable

    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(&main_rs)
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

    // Add Standard Text to Cargo.toml
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(&cargo_toml)
        .expect("Failed to open Cargo.toml");

    file.write_all(
        format!(
            "[package]
name = \"{}\"
version = \"0.1.0\"
authors = [\"CrabbyRails\"]
edition = \"2021\"

[dependencies]
rocket = \"0.5.0-rc.1\"",
            project_name
        )
        .as_bytes(),
    )
    .expect("Failed to write to Cargo.toml");

    println!("Project {} created!", project_name);
}

fn main() {
    greet_user();
}
