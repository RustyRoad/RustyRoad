use actix_cors::Cors;
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
            .service(controllers::page::create_page)
            .service(controllers::page::update_page)
            .service(controllers::page::get_page_by_id)
            .service(Files::new("/static", "./static")) // Add this line
    })
    .bind(("127.0.0.1", 80))
    .unwrap()
    .workers(2)
    .run()
    .await
}
