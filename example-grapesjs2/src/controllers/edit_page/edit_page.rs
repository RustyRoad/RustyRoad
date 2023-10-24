use actix_identity::Identity;
use actix_web::{get, web, HttpResponse, Responder};
use tera::{Context, Tera};
use crate::models::Page;

#[get("/page/{id}/edit")]
async fn edit_page(tmpl: web::Data<Tera>,  id: web::Path<i32>, user: Option<Identity>) -> impl Responder {
if let Some(user) = user
{
      let mut context = Context::new();
      let page = Page::get_page_by_id(id.into_inner()).await.unwrap();
      context.insert("username", &user.id().unwrap());
      context.insert("title", "edit_page");
      context.insert("controller_name", "edit_page");
      context.insert("page_id", &page.id.clone());
      context.insert("html_content", &page.html_content);
      let rendered = tmpl.render("layouts/authenticated/page/edit_page.html.tera", &context).unwrap();
      HttpResponse::Ok().body(rendered)
} else {
    let mut context = Context::new();
    context.insert("title", "Login");
    context.insert("route_name", "login");
    context.insert("error", "You must be logged in to view this page.");
    HttpResponse::Found()
        .append_header((actix_web::http::header::LOCATION, "/login"))
        .finish()
    }
}