use actix_web::{get, web, HttpResponse, HttpRequest, Error};
use tera::{Context, Tera};
use crate::models;
use rustyroad::database::Database;
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
    db: web::Data<Database>,
    req: HttpRequest
) -> Result<HttpResponse, actix_web::Error> {
     form.user_login(req, tmpl, db.get_ref().clone()).await
}


#[get("/logout")]
async fn user_logout(
    tmpl: web::Data<Tera>,
    user: Option<actix_identity::Identity>,
) -> Result<HttpResponse, Error> {
    if let Some(user) = user {
        UserLogin::user_logout(tmpl, user).await
   } else {
         let mut context = Context::new();
         context.insert("controller_name", "login");
         context.insert("error", "You must be logged in to logout.");
         let rendered = tmpl.render("pages/login.html.tera", &context).unwrap();
         Ok(HttpResponse::Ok().body(rendered))
   }
}
