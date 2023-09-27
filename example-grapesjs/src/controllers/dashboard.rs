use crate::models;
use actix_web::{get, web, HttpResponse};
use models::user::UserLogin;
use tera::{Context, Tera};

#[get("/dashboard")]
async fn dashboard_controller(tmpl: web::Data<Tera>) -> HttpResponse {
    let mut context = Context::new();
    context.insert("controller_name", "dashboard");
    let rendered = tmpl.render("pages/dashboard.html.tera", &context).unwrap();
    HttpResponse::Ok().body(rendered)
}
