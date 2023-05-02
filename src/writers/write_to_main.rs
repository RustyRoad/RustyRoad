use regex::Regex;

use crate::writers::write_to_file;
use crate::Project;
use std::fs;
use std::io::Error;
use std::path::PathBuf;

/// This function writes initial content to the main.rs file of a new RustyRoad project.
/// The content includes setting up an Actix web server with three routes: index, dashboard, and login.
///
/// # Arguments
///
/// * `project` - A reference to the Project struct containing the paths to the project files.
///
/// # Returns
///
/// * `Ok(())` if the content was successfully written to the main.rs file, or an Error if something went wrong.
pub fn write_to_main_rs(project: &Project) -> Result<(), Error> {
    // Define the contents to be written to the main.rs file
    // This includes importing necessary Actix Web modules, defining the main function, setting up the HTTP server,
    // binding it to the localhost on port 8000, and defining three routes: index, dashboard, and login
    let contents = r#"use std::env;

use actix_files::Files;
use actix_session::storage::CookieSessionStore;
use actix_web::cookie::Key;
use actix_web::{
    web::{self},
    App, HttpServer,
};
use tera::Tera;
mod models;
mod routes;

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

    println!("Starting Actix web server...");

    HttpServer::new(move || {
        // Load tera templates from the specified directory
        let tera = Tera::new("templates/**/*").unwrap();
        println!("Initializing Actix web application...");

        App::new()
            .wrap(
                actix_web::middleware::Logger::default()
                    .exclude("/static")
                    .exclude("/favicon.ico"),
            )
            .wrap(actix_session::SessionMiddleware::new(
                CookieSessionStore::default(),
                get_secret_key().expect("Failed to generate secret key"),
            ))
            .app_data(web::Data::new(tera.clone())) // Updated line
            .service(routes::index::index)
            .service(routes::dashboard::dashboard_route)
            .service(routes::login::login_route)
            .service(routes::login::login_function)
             .service(routes::login::user_logout)
            .service(Files::new("/static", "./static")) // Add this line
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}"#;

    // Write the contents to the main.rs file
    // The write_to_file function is assumed to be a function that takes a path and a byte slice and writes the bytes to the file at the path
    // If the file doesn't exist, the function will create it, and if it does exist, the function will overwrite it

    write_to_file(&project.main_rs, contents.as_bytes())?;

    // Return Ok if everything succeeded
    Ok(())
}


/// This function adds a new route to the main.rs file of a RustyRoad project.
/// It first verifies that the current project is indeed a RustyRoad project by checking for the presence of a rustyroad.toml file.
/// Then it reads the main.rs file, identifies the last .service() call, and adds a new .service() call for the provided route name after the last .service() call.
/// If successful, it overwrites the main.rs file with the updated content.
///
/// # Arguments
///
/// * `route_name` - A string slice that holds the name of the new route to be added.
///
/// # Returns
///
/// * `Ok(())` if the new route was successfully added to the main.rs file, or an Error if something went wrong.
pub fn add_new_route_to_main_rs(route_name: &str) -> Result<(), Error> {
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
    let main_rs: PathBuf = [current_dir, PathBuf::from("src/main.rs")].iter().collect();

    // Read the file into a string
    let mut contents = fs::read_to_string(&main_rs)?;

    // Prepare the new route
    let new_route = format!(".service(routes::{}::{})", route_name, route_name);

    // Prepare the regular expression to find the last .service() call
    let re = Regex::new(r".service\(routes::\w+::\w+\)").unwrap();

    // Find the last .service() call and its end position
    let last_service_end_pos = re
        .find_iter(&contents)
        .last()
        .ok_or(Error::new(
            std::io::ErrorKind::InvalidData,
            "Could not find the position to insert new route",
        ))?
        .end();

    // Insert the new route after the last .service() call
    contents.insert_str(last_service_end_pos, &new_route);

    // Write the string back to the file
    fs::write(main_rs, contents)?;

    Ok(())
}
