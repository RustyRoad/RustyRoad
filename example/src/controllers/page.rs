use crate::models;
use crate::models::HtmlGrapesJs;
use actix_web::{get, web, HttpRequest, HttpResponse};
use color_eyre::eyre::Error;
use tera::Tera;

#[get("/page/{id}")]
async fn get_page_by_id(
    tmpl: web::Data<Tera>,
    req: HttpRequest,
    id: web::Path<i32>,
) -> HttpResponse {
    let database = rustyroad::database::Database::get_database_from_rustyroad_toml().unwrap();
    let result = HtmlGrapesJs::get_page_by_id(id.into_inner()).await;
    println!("{:?}", result);
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

            let s = tmpl.render("pages/404.html.tera", &context).unwrap();
            HttpResponse::Ok().body(s)
        }
    }
}
