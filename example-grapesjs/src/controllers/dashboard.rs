use crate::models::{self, user};
use actix_identity::Identity;
use actix_web::{get, web, HttpResponse};
use tera::{Context, Tera};

#[get("/dashboard")]
async fn dashboard_controller(tmpl: web::Data<Tera>, user: Option<Identity>) -> HttpResponse {
    if let Some(user) = user {
        let mut context = tera::Context::new();
        context.insert("username", &user.id().unwrap());
        context.insert("title", "Dashboard");
        context.insert("controller_name", "dashboard");
        let rendered = tmpl.render("layouts/authenticated/dashboard.html.tera", &context).unwrap();
        HttpResponse::Ok().body(rendered)
    } else {
        let mut context = tera::Context::new();
        context.insert("title", "Login");
        context.insert("route_name", "login");
        context.insert("error", "You must be logged in to view this page.");
        let rendered = tmpl.render("pages/login.html.tera", &context).unwrap();
        HttpResponse::Found()
            .status(actix_web::http::StatusCode::UNAUTHORIZED)
            .body(rendered)
    }
}
