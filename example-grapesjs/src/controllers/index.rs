use models::{Page};
use crate::{models};

use actix_web::{get, web, HttpResponse};
use tera::{Context, Tera};

#[get("/")]
async fn index(tmpl: web::Data<Tera>) -> HttpResponse {
    let mut context = Context::new();
    context.insert("foo", "123");
    
      let result = Page::get_page_by_slug("index".to_string()).await;
      match result {
            Ok(page) => {
                context.insert("title", "Create Page");
                context.insert("route_name", "create_page");
                context.insert("html_content", &page.html_content);
                context.insert("page_id", &page.id);
                let s = tmpl.render("pages/page.html.tera", &context).unwrap();
                HttpResponse::Ok()
                    .content_type("text/html; charset=utf-8")
                    .body(s)
            }
            Err(e) => {
                context.insert("error", &e.to_string());
                let s = tmpl.render("pages/404.html.tera", &context).unwrap();
                HttpResponse::Ok()
                    .content_type("text/html; charset=utf-8")
                    .body(s)
            }
        }
    
}
