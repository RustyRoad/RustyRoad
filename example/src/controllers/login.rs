use actix_web::{get, web, HttpResponse, HttpRequest, Error};
use tera::{Context, Tera};
use crate::models;
use models::user::UserLogin;

#[get("/login")]
async fn login_controller(tmpl: web::Data<Tera>) -> HttpResponse {
    let mut context = Context::new();
    context.insert("controller_name", "login");
    let rendered = tmpl.render("pages/login.html.tera", &context).unwrap();
    HttpResponse::Ok().body(rendered)
}

 use actix_web::post;

#[post("/login")]
async fn login_function(
    form: web::Form<UserLogin>,
    tmpl: web::Data<Tera>, // Updated line
) -> Result<HttpResponse, actix_web::Error> {
    // get database data from rustyroad.toml

    let database = rustyroad::database::Database::get_database_from_rustyroad_toml().unwrap();

    form.user_login(tmpl, database).await
}


#[get("/logout")]
async fn user_logout(
    tmpl: web::Data<Tera>,
    req: HttpRequest, // Add the HttpRequest
) -> Result<HttpResponse, Error> {
 let database = rustyroad::database::Database::get_database_from_rustyroad_toml().unwrap();
    UserLogin::user_logout(tmpl, database, req).await
}
