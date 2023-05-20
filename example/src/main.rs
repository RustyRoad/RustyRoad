use std::env;

use actix_files::Files;
use actix_session::storage::CookieSessionStore;
use actix_web::cookie::Key;
use actix_web::{web, App, HttpServer};
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
            .service(routes::index::index)
            .service(Files::new("/static", "./static").show_files_listing())
            .service(Files::new("/vite", "./client/dist/client").show_files_listing())
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
