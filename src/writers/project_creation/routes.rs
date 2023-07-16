use crate::writers::write_to_file;
use crate::Project;
use std::io::Error;

// Write to index route
pub fn write_to_index_route(project: &Project) -> Result<(), Error> {
    let contents = r#"use actix_web::{get, web, HttpResponse, Responder};
use tera::{Context, Tera};

#[get("/")]
async fn index(tmpl: web::Data<Tera>) -> HttpResponse {
    let mut context = Context::new();
    context.insert("foo", "123");
    let rendered = tmpl.render("pages/index.html.tera", &context).unwrap();
    HttpResponse::Ok().body(rendered)
}"#
    .to_string();

    write_to_file(&project.index_route.to_string(), contents.as_bytes()).unwrap_or_else(|why| {
        println!("Couldn't write to {}: {}", project.index_route, why);
    });
    Ok(())
}

// Write to dashboard route
pub fn write_to_dashboard_route(project: &Project) -> Result<(), Error> {
    let contents = r#" use actix_web::{get, web, HttpResponse, Responder};
         use actix_identity::Identity;
use actix_web::{get, web, Error, HttpRequest, HttpResponse, Responder};

use tera::{Context, Tera};

#[get("/dashboard")]
pub async fn dashboard_route(
    tmpl: web::Data<Tera>,
    user: Option<Identity>,
) -> Result<HttpResponse, actix_web::Error> {
    if let Some(user) = user {
        let mut context = Context::new();
        context.insert("username", &user.id().unwrap());
        let rendered = tmpl.render("pages/dashboard.html.tera", &context).unwrap();
        Ok(HttpResponse::Ok().body(rendered))
    } else {
        let mut context = Context::new();
        context.insert("error", "You must be logged in to view this page.");
        Ok(HttpResponse::Found()
            .append_header((actix_web::http::header::LOCATION, "/login"))
            .finish())
    }
}"#
    .to_string();
    write_to_file(&project.dashboard_route.to_string(), contents.as_bytes()).unwrap_or_else(
        |why| {
            println!("Couldn't write to {}: {}", project.index_route, why);
        },
    );
    Ok(())
}

// Write to not_found route
pub fn write_to_not_found_route(project: &Project) -> Result<(), Error> {
    let contents = r#"use actix_web::{get, web, HttpResponse};
use tera::Tera;

#[get("/not_found")]
async fn not_found(tmpl: web::Data<Tera>) -> HttpResponse {
    let mut context = tera::Context::new();
    context.insert("route_name", "not_found");
    let rendered = tmpl
        .render("pages/404.html.tera", &context)
        .unwrap_or_else(|err| {
            eprintln!("Template rendering error: {}", err);
            String::from("Server error")
        });

    HttpResponse::NotFound().body(rendered)
}"#
    .to_string();
    write_to_file(&project.not_found_route.to_string(), contents.as_bytes()).unwrap_or_else(
        |why| {
            println!("Couldn't write to {}: {}", project.index_route, why);
        },
    );
    Ok(())
}

