use std::io::Error;
use crate::Project;
use crate::writers::write_to_file;

// Write to index route
pub fn write_to_index_route(project: &Project) -> Result<(), Error> {
    let contents = r#"use actix_web::{get, web, HttpResponse, Responder};
use tera::{Context, Tera};

#[get("/")]
async fn index(tmpl: web::Data<Tera>) -> HttpResponse {
    let mut context = Context::new();
    context.insert("foo", "123");
    let rendered = tmpl.render("pages/index.html.tera", &context).unwrap();
    HttpResponse::Ok().body(rendered)
}"#.to_string();

    write_to_file(&project.index_route.to_string(), contents.as_bytes()).unwrap_or_else(|why| {
        println!(
            "Couldn't write to {}: {}",
            project.index_route,
            why
        );
    });
    Ok(())
}
