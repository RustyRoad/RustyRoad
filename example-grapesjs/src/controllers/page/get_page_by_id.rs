use crate::models;
use actix_web::{get, web, HttpRequest, HttpResponse};
use color_eyre::eyre::Error;
use tera::Tera;

#[get("/page/{id}")]
async fn get_page_by_id(
    tmpl: web::Data<Tera>,
    id: web::Path<i32>
) -> HttpResponse {
    let result = models::Page::get_page_by_id(id.into_inner()).await;
    match result {
        Ok(page) => {
            let mut context = tera::Context::new();

            let html_content = page.html_content;

            context.insert("html_content", &html_content);
            let s = tmpl.render("pages/page.html.tera", &context).unwrap();
            HttpResponse::Ok().body(s)
        }
        Err(e) => {
            let mut context = tera::Context::new();
            context.insert("error", &e.to_string());
            let s = tmpl.render("pages/404.html.tera", &context).unwrap();
            HttpResponse::Ok().body(s)
        }
    }
}
