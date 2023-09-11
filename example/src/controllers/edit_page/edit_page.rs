use actix_web::{get, post, web, HttpResponse, Responder};
use serde_json::json;
use tera::{Context, Tera};

use crate::models::HtmlGrapesJs;

#[get("/edit_page")]
async fn edit_page(tmpl: web::Data<Tera>) -> impl Responder {
    let mut context = Context::new();
    context.insert("controller_name", "edit_page");
    let rendered = tmpl
        .render(
            "layouts/authenticated/grapesjs/edit_page.html.tera",
            &context,
        )
        .unwrap();
    HttpResponse::Ok().body(rendered)
}

// save the html to the database
// this should have a unique id for the page
// for now we will just save to the main page
#[post("/save_page")]
async fn save_page(html: web::Json<HtmlGrapesJs>) -> impl Responder {
    println!("html: {:?}", html.html_content);
    println!("Created At: {:?}", html.created_at);
    println!("Updated At: {:?}", html.updated_at);
    println!("Associated User Id: {:?}", html.associated_user_id);
    // create the page
    let new_page = HtmlGrapesJs::create_new_database_page(html.into_inner())
        .await
        .unwrap();

    let json_response = json!({
        "status": "success",
        "message": "Page saved successfully",
        "data": new_page
    });

    HttpResponse::Ok().json(json_response)
}
