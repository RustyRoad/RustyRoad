use actix_web::{HttpResponse, web::Data, get};
use tera::{Context, Tera};


#[get("/not_found")]
async fn not_found(tmpl: Data<Tera>) -> HttpResponse {
    let mut context = tera::Context::new();
    context.insert("controller_name", "not_found");
    let rendered = tmpl
        .render("pages/404.html.tera", &context)
        .unwrap_or_else(|err| {
            eprintln!("Template rendering error: {}", err);
            String::from("Server error")
        });

    HttpResponse::NotFound().body(rendered)
}