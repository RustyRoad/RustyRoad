use crate::models;
use actix_web::{get, web, HttpResponse};
use actix_identity::Identity;
use tera::Tera;
// I want this to be only after the page is created
// we can create another controller for just loading the editor instead of get_by_id
#[get("/page/{id}")]
async fn get_page_by_id(
    tmpl: web::Data<Tera>,
    id: web::Path<i32>,
    user: Option<Identity>
) -> HttpResponse {
    if let Some(user) = user {
        let result = models::Page::get_page_by_id(id.into_inner()).await;
        match result {
            Ok(page) => {
                let mut context = tera::Context::new();
                context.insert("title", "Page");
                context.insert("route_name", "page");
                context.insert("page", &page);
                context.insert("html_content", &page.html_content);
                context.insert("page_id", &page.id);
                context.insert("username", &user.id().unwrap());
                let s = tmpl.render("layouts/authenticated_page/page/edit_page.html.tera", &context).unwrap();
                println!("rendering page: Edit Page");
                HttpResponse::Ok().body(s)
            }
            Err(e) => {
                let mut context = tera::Context::new();
                context.insert("message", "create your page");
                let s = tmpl.render("layouts/authenticated_page/page/create_page.html.tera", &context).unwrap();
                println!("rendering page: Create Page");
                HttpResponse::Ok().body(s)
            }
        }
    } else {
        let result = models::Page::get_page_by_id(id.into_inner()).await;m
        match result {
            Ok(page) => {
                let mut context = tera::Context::new();
                context.insert("title", "Create Page");
                context.insert("route_name", "create_page");
                context.insert("html_content", &page.html_content);
                context.insert("page_id", &page.id);
                let s = tmpl.render("pages/page.html.tera", &context).unwrap();
                println!("rendering page: Page {:?}", &page.id);
                HttpResponse::Ok().body(s)
            }
            Err(e) => {
                let mut context = tera::Context::new();
                context.insert("error", &e.to_string());
                let s = tmpl.render("pages/404.html.tera", &context).unwrap();
                println!("rendering page: 404");
                HttpResponse::Ok().body(s)
            }
        }   
    }
}    
