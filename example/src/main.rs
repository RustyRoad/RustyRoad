
use actix_files::Files;
use actix_web::{
    web::{self, Data},
    App, HttpServer,
};
use tera::Tera;
mod routes;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    println!("Starting Actix web server...");

    HttpServer::new(move || {
        // Load tera templates from the specified directory
        let tera = Tera::new("templates/**/*").unwrap();
        println!("Initializing Actix web application...");

        App::new()
            .app_data(Data::new(tera.clone())) // Updated line
            .service(routes::index::index)
            .service(routes::dashboard::dashboard_route)
            .service(routes::login::login_route)
            .service(routes::login::login_function)
            .service(Files::new("/static", "./static"))  // Add this line
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
