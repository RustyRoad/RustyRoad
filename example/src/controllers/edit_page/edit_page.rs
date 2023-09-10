use actix_web::{get, web, HttpResponse, Responder};
use tera::{Context, Tera};

#[get("/edit_page")]
async fn edit_page(tmpl: web::Data<Tera>) -> impl Responder {
    let mut context = Context::new();
    context.insert("controller_name", "edit_page");
    let rendered = tmpl.render("layouts/authenticated/grapesjs/edit_page.html.tera", &context).unwrap();
    HttpResponse::Ok().body(rendered)
}
