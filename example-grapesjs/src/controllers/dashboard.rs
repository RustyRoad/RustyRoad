use serde_json::{json};
use actix_identity::{Identity};
use tera::{Tera, Context};
use actix_web::{Error, get, web::Data, HttpResponse, web, http::header::LOCATION};
use models::{Page};
use crate::{models};


#[get("/dashboard")]
pub async fn dashboard_controller(
    tmpl: Data<Tera>,
    user: Option<Identity>,
) -> Result<HttpResponse, Error> {
    if let Some(user) = user {
        let mut context = Context::new();
        context.insert("username", &user.id().unwrap());
        let rendered = tmpl.render("pages/dashboard.html.tera", &context).unwrap();
        Ok(HttpResponse::Ok().body(rendered))
    } else {
        let mut context = Context::new();
        context.insert("error", "You must be logged in to view this page.");
        Ok(HttpResponse::Found()
            .append_header((LOCATION, "/login"))
            .finish())
    }
}


#[get("/page_dashboard")]
async fn page_dashboard(tmpl: Data<Tera>, user: Option<Identity>) -> HttpResponse {
    if let Some(user) = user {
        let pages_result = Page::get_all_pages().await; // Await the future

        let mut context = Context::new();
        match pages_result {
            Ok(pages) => {
                let pages_data = pages.get("data").unwrap().as_array().unwrap();
                context.insert("pages", pages_data);
                context.insert("error", ""); // Insert an empty string for the error variable
            }
            Err(e) => {
                println!("Error: {}", e);
                let message = json!({"error": e.to_string()});
                context.insert("error", &message);
                context.insert("pages", &Vec::<Page>::new()); // Insert an empty vector for the pages variable
            }
        };
        context.insert("username", &user.id().unwrap());
        context.insert("title", "Dashboard");
        context.insert("controller_name", "page_dashboard");

        let rendered = tmpl
            .render(
                "layouts/authenticated_page/page/page_dashboard.html.tera",
                &context,
            )
            .expect("Failed to render template");

        HttpResponse::Ok().body(rendered)
    } else {
        let mut context = Context::new();
        context.insert("title", "Login");
        context.insert("route_name", "login");
        context.insert("error", "You must be logged in to view this page.");
        HttpResponse::Found()
            .append_header((LOCATION, "/login"))
            .finish()
    }
}




#[get("/create_page")]
async fn create_page_dashboard(tmpl: Data<Tera>, user: Option<Identity>) -> HttpResponse {
    if let Some(user) = user {
        let mut context = Context::new();
        context.insert("title", "Create Page");
        context.insert("route_name", "create_page");
        let mut page = Page::new();
        page.id = Some(0);
        let html_content = "<h1 class=\"text-center\">Welcome To The Page Builder</h1><p class=\"text-center\">Drag a block from the right side to this area to get started.</p>";
        page.html_content = html_content.to_string();
        context.insert("page", &page);
        context.insert("username", &user.id().unwrap());
        context.insert("html_content", &page.html_content);
        let s = tmpl
            .render(
                "layouts/authenticated_page/page/page_details.html.tera",
                &context,
            )
            .unwrap();
        HttpResponse::Ok().body(s)
    } else {
        let mut context = Context::new();
        context.insert("title", "Login");
        context.insert("route_name", "login");
        context.insert("error", "You must be logged in to view this page.");
        HttpResponse::Found()
            .append_header((LOCATION, "/login"))
            .finish()
    }
}
