use actix_web::{get, web, HttpResponse, HttpRequest, Error};
use tera::{Context, Tera};
use crate::models;
use rustyroad::database::Database;
use models::user::UserLogin;
use actix_identity::Identity;

#[get("/dashboard")]
async fn dashboard_controller( tmpl: web::Data<Tera>,
    user: Option<Identity>
) -> HttpResponse {
    if let Some(user) = user {
    let mut context = Context::new();
    context.insert("controller_name", "dashboard");
    context.insert("username", &user.id().unwrap());
    let rendered = tmpl.render("pages/dashboard.html.tera", &context).unwrap();
    HttpResponse::Ok().body(rendered)
    } else {
        let mut context = Context::new();
        context.insert("title", "Login");
        context.insert("route_name", "login");
        context.insert("error", "You must be logged in to view the dashboard.");
        HttpResponse::Found()
            .append_header((actix_web::http::header::LOCATION, "/login"))
            .finish()
    }
}