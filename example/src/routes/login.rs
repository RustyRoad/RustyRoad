use actix_web::{get, web, HttpResponse, Responder};
use tera::{Context, Tera};

#[get("/login")]
async fn login_route(tmpl: web::Data<Tera>) -> HttpResponse {
    let mut context = Context::new();
    context.insert("route_name", "login");
    let rendered = tmpl.render("pages/login.html.tera", &context).unwrap();
    HttpResponse::Ok().body(rendered)
}

    use actix_web::{post};
    use serde::Deserialize;

#[derive(Deserialize)]
struct LoginForm {
    username: String,
    password: String,
}

#[post("/login")]
async fn login_function(form: web::Form<LoginForm>, tmpl: web::Data<Tera>) -> HttpResponse {
    let mut context = Context::new();
    if form.username == "admin" && form.password == "admin" {
        context.insert("route_name", "dashboard");
        let rendered = tmpl.render("pages/dashboard.html.tera", &context).unwrap();
        HttpResponse::Ok().body(rendered)
    } else {
        context.insert("route_name", "login");
        context.insert("error_message", "Invalid username or password");
let rendered = tmpl.render("pages/login.html.tera", &context).unwrap();
HttpResponse::Ok().body(rendered)
    }
}