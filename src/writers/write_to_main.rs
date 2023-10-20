use crate::writers::write_to_file;
use crate::Project;
use color_eyre::eyre::Result;
use regex::Regex;
use std::fs;
use std::io::Error;

/// This function writes initial content to the main.rs file of a new RustyRoad project.
/// The content includes setting up an Actix web server with three controllers: index, dashboard, and login.
///
/// # Arguments
///
/// * `project` - A reference to the Project struct containing the paths to the project files.
///
/// # Returns
///
/// * `Ok(())` if the content was successfully written to the main.rs file, or an Error if something went wrong.
/// # Example
/// ```rust
/// use rustyroad::writers::write_to_main::write_to_main_rs;
/// use rustyroad::Project;
///
/// let project = Project::new();
/// write_to_main_rs(&project);
/// ```
pub fn write_to_main_rs(project: &Project) -> Result<(), Error> {
    // Define the contents to be written to the main.rs file
    // This includes importing necessary Actix Web modules, defining the main function, setting up the HTTP server,
    // binding it to the localhost on port 8000, and defining three controllers: index, dashboard, and login
    let contents = r#"use actix_cors::Cors;
use actix_files::Files;
use actix_identity::IdentityMiddleware;
use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;
use actix_web::{
    web::{self},
    App, HttpServer,
};
use color_eyre::eyre::Result;
use rustyroad::database::Database;
use std::env;
use tera::Tera;
mod controllers;
mod models;

fn get_secret_key() -> Result<Key, Box<dyn std::error::Error>> {
    let secret_key_from_env = env::var("SECRET_KEY")?;
    if secret_key_from_env.len() < 32 {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Secret key must be at least 32 characters",
        )));
    }
    let key = Key::from(secret_key_from_env.as_bytes());
    Ok(key)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    let database = web::Data::new(Database::get_database_from_rustyroad_toml().unwrap());

    println!("Starting Actix web server...");

    HttpServer::new(move || {
        let cors = Cors::permissive();
        // Load tera views from the specified directory
        let tera = Tera::new("src/views/**/*").unwrap();
        println!("Initializing Actix web application...");

        let secret_key = get_secret_key().unwrap();

        let session_mw = SessionMiddleware::builder(CookieSessionStore::default(), secret_key)
            // disable secure cookie for local testing
            .cookie_secure(false)
            .build();

        App::new()
            .wrap(
                actix_web::middleware::Logger::default()
                    .exclude("/static")
                    .exclude("/favicon.ico"),
            )
            .wrap(cors)
            .wrap(IdentityMiddleware::default())
            .app_data(database.clone())
            .wrap(session_mw)
            .app_data(web::Data::new(tera.clone())) // Updated line
            .service(controllers::index::index)
            .service(controllers::dashboard::dashboard_controller)
            .service(controllers::login::login_controller)
            .service(controllers::login::login_function)
            .service(controllers::login::user_logout)
            .service(Files::new("/static", "./static")) // Add this line
    })
    .bind(("127.0.0.1", 80))
    .unwrap()
    .workers(2)
    .run()
    .await
}
"#;

    // Write the contents to the main.rs file
    // The write_to_file function is assumed to be a function that takes a path and a byte slice and writes the bytes to the file at the path
    // If the file doesn't exist, the function will create it, and if it does exist, the function will overwrite it

    write_to_file(&project.main_rs, contents.as_bytes())?;

    // Return Ok if everything succeeded
    Ok(())
}

/// This function adds a new controller to the main.rs file of a RustyRoad project.
/// It first verifies that the current project is indeed a RustyRoad project by checking for the presence of a rustyroad.toml file.
/// Then it reads the main.rs file, identifies the last .service() call, and adds a new .service() call for the provided controller name after the last .service() call.
/// If successful, it overwrites the main.rs file with the updated content.
///
/// # Arguments
///
/// * `controller_name` - A string slice that holds the name of the new controller to be added.
///
/// # Returns
///
/// * `Ok(())` if the new controller was successfully added to the main.rs file, or an Error if something went wrong.
/// # Example
/// ```rust
/// use rustyroad::writers::write_to_main::add_new_controller_to_main_rs;
///
/// add_new_controller_to_main_rs("about");
/// ```
pub fn add_new_controller_to_main_rs(
    folder_name: Option<&str>,
    controller_name: &str,
) -> Result<(), Error> {
    println!("CONTROLLER NAME: {}", &controller_name);
    // Check for the current working directory
    let current_dir = std::env::current_dir().unwrap();

    // Ensure that the project is a rustyroad project by looking for the rustyroad.toml file in the root directory
    match std::fs::read_to_string(current_dir.join("rustyroad.toml")) {
        Ok(_) => {}
        Err(_) => {
            return Err(Error::new(
                std::io::ErrorKind::InvalidData,
                "This is not a RustyRoad project",
            ))
        }
    }

    // Construct the path to the main.rs file
    let main_rs_path = current_dir.join("src/main.rs");

    // Read the file into a string
    let mut contents = fs::read_to_string(&main_rs_path)?;

    // Prepare the new controller
    let new_controller = format!(
        "\n.service({})",
        if let Some(folder_name) = folder_name {
            format!("controllers::{}::{}", folder_name, controller_name)
        } else {
            format!("controllers::{}", controller_name)
        }
    );

    println!("{}", new_controller);
    // Prepare the regular expression to find the last .service() call
    let re = Regex::new(r".service\(controllers::\w+::\w+\)").unwrap();

    // Find the last .service() call and its end position
    let last_service_end_pos = re
        .find_iter(&contents)
        .last()
        .ok_or(Error::new(
            std::io::ErrorKind::InvalidData,
            "Could not find the position to insert new controller",
        ))?
        .end();

    // Insert the new controller after the last .service() call
    contents.insert_str(last_service_end_pos, &new_controller);

    // Write the string back to the file
    fs::write(main_rs_path, contents)?;

    Ok(())
}

/// # Name: add_new_controller_to_existing_module_in_main_rs
/// # Description: This function adds a new controller to an existing module in the main.rs file of a RustyRoad project.
/// # Arguments
/// * `existing_controller_name` - A string slice that holds the name of the existing controller to which the new controller will be added.
/// * `new_controller_name` - A string slice that holds the name of the new controller to be added.
/// # Returns
/// * `Ok(())` if the new controller was successfully added to the main.rs file, or an Error if something went wrong.
/// # Example
/// ```rust
/// use rustyroad::writers::write_to_main::add_new_controller_to_existing_module_in_main_rs;
///
/// add_new_controller_to_existing_module_in_main_rs("page", "about");
/// ```
pub fn add_new_controller_to_existing_module_in_main_rs(
    existing_controller_name: &str,
    new_controller_name: &str,
) -> Result<(), Error> {
    // Check for the current working directory
    let current_dir = std::env::current_dir().unwrap();

    // Ensure that the project is a rustyroad project by looking for the rustyroad.toml file in the root directory
    match std::fs::read_to_string(current_dir.join("rustyroad.toml")) {
        Ok(_) => {}
        Err(_) => {
            return Err(Error::new(
                std::io::ErrorKind::InvalidData,
                "This is not a RustyRoad project",
            ))
        }
    }

    // Construct the path to the main.rs file
    let main_rs_path = current_dir.join("src/main.rs");

    // Read the file into a string
    let mut contents = fs::read_to_string(&main_rs_path)?;

    // Prepare the new controller
    let new_controller = format!(
        ".service({}::{})",
        existing_controller_name, new_controller_name
    );

    // Prepare the regular expression to find the last .service() call
    let re = Regex::new(r"\.service\((\w+::)*\w+::\w+\)").unwrap();

    // Find the last .service() call and its end position
    let last_service_end_pos = re
        .find_iter(&contents)
        .last()
        .ok_or(Error::new(
            std::io::ErrorKind::InvalidData,
            "Could not find the position to insert new controller",
        ))?
        .end();

    // Insert the new controller after the last .service() call
    contents.insert_str(last_service_end_pos, &new_controller);

    // Write the string back to the file
    fs::write(main_rs_path, contents)?;

    Ok(())
}
