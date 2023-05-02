use std::env;

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
}