use std::fs::create_dir;
use crate::generators::create_file;
use crate::writers::{
    add_new_controller_to_main_rs, write_to_controllers_mod, write_to_file, write_to_module,
};
use color_eyre::eyre::Result;
use eyre::Error;

pub async fn write_to_get_page_by_id() -> Result<(), Error> {
    let contents = r#"use crate::models;
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
"#;
    create_dir("src/controllers/page").expect("Error creating the page controllers directory");
    println!("Page controllers directory created successfully!");
    println!("adding the page controllers module to the controllers module...");
    write_to_controllers_mod(&"src/controllers/mod.rs".to_string(), "page".to_string())
        .expect("Error writing the page controllers module to the controllers module");
    // create the get_page_by_id controller file
    println!("Creating the get_page_by_id controller file...");
    create_file("src/controllers/page/get_page_by_id.rs")
        .expect("Error creating the get_page_by_id controller file");
    // write the get_page_by_id controller to the file
    println!("Writing the get_page_by_id controller to the file...");
    write_to_file(
        "src/controllers/page/get_page_by_id.rs",
        contents.as_bytes(),
    )
    .expect("Error writing to the get_page_by_id controller file");
    // Add the get_page_by_id module to the controllers module
    let mut components = Vec::new();
    // add the get_page_by_id module to the components vector
    components.push("get_page_by_id".to_string());
    // define the module path
    let module_path = "src/controllers/page/mod.rs".to_string();
    // write the get_page_by_id module to the controllers module
    println!("Writing the get_page_by_id module to the controllers module...");
    write_to_module(&module_path, components)
        .expect("Error writing the get_page_by_id module to the controllers module");
    // Add the get_page_by_id controller to the main.rs file
    println!("Adding the get_page_by_id controller to the main.rs file...");
    add_new_controller_to_main_rs(Some("page"), "get_page_by_id")
        .expect("Error adding the get_page_by_id controller to the main.rs file");
    // call the html writer
    write_to_get_page_by_id_html().expect("Error writing the page.html.tera file");
    Ok(())
}

pub fn write_to_get_page_by_id_html() -> Result<(), Error> {
    let contents = r#"{% extends 'base.html.tera' %}
        {% block title %}Page Title Here{% endblock title %}
        {% block head %}
        {{ super() }}
        {% endblock head %}
        {% block content %}
        {{html_content|safe}}
        {% endblock content %}"#;
    println!("Creating the page.html.tera file...");
    create_file("src/views/pages/page.html.tera").expect("Error creating the page.html.tera file");
    println!("Writing the page.html.tera file...");
    write_to_file("src/views/pages/page.html.tera", contents.as_bytes())
        .expect("Error writing the page.html.tera file");

    Ok(())
}
