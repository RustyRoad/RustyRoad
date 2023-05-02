use actix_web::{get, web, HttpResponse, Responder};
use tera::{Context, Tera};

#[get("/")]
async fn index(tmpl: web::Data<Tera>) -> HttpResponse {
    let mut context = Context::new();
    context.insert("foo", "123");
    let rendered = tmpl.render("pages/index.html.tera", &context).unwrap();
    HttpResponse::Ok().body(rendered)
}