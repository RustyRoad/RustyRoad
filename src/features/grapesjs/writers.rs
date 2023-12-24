use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use crate::generators::create_file;
use crate::writers::{add_new_controller_to_main_rs, write_to_file};
use color_eyre::eyre::Result;
use eyre::Error;
use crate::helpers::helpers::{add_or_update_import, determine_controller_path, prompt_to_create_controller};

pub fn write_to_get_page_by_id() -> Result<(), Error> {
    let contents = r#"
#[get("/page/{id}")]
async fn get_page_by_id(tmpl: Data<Tera>, id: Path<i32>, user: Option<Identity>) -> HttpResponse {
    let result = Page::get_page_by_id(id.into_inner()).await;
    match result {
        Ok(page) => {
            let mut context = Context::new();
            if let Some(_user) = user {
                context.insert("username", &_user.id().unwrap());
            }
            context.insert("title", "Create Page");
            context.insert("route_name", "create_page");
            context.insert("html_content", &page.html_content);
            context.insert("page_id", &page.id.unwrap());
            let s = tmpl.render("pages/page.html.tera", &context).unwrap();
            HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(s)
        }
        Err(e) => {
            let mut context = Context::new();
            context.insert("error", &e.to_string());
            if let Some(_user) = user {
                context.insert("username", &_user.id().unwrap());
            }
            let s = tmpl.render("pages/404.html.tera", &context).unwrap();
            HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(s)
        }
    }
}
"#;

    // Determine the controller file path
    let path = determine_controller_path("page");

    // Ensure the controller file exists, or create it
    // Ensure the controller file exists, or create it
    let file_exists = PathBuf::from(&path).exists();
    if !file_exists {
        prompt_to_create_controller(&path).expect("Error prompting to create controller");
    }

    // Read and update the file contents
    let mut file_contents = fs::read_to_string(&path)?;

    // Update imports
    file_contents = add_or_update_import(&file_contents, "actix_web", "get");
    file_contents = add_or_update_import(&file_contents, "tera", "Tera");
    file_contents = add_or_update_import(&file_contents, "tera", "Context");
    file_contents = add_or_update_import(&file_contents, "models", "Page");
    file_contents = add_or_update_import(&file_contents, "actix_web", "HttpResponse");
    file_contents = add_or_update_import(&file_contents, "actix_identity", "Identity");
    file_contents = add_or_update_import(&file_contents, "actix_web", "web::Data");
    file_contents = add_or_update_import(&file_contents, "actix_web", "web::Path");

    // Add the new controller to the file
    file_contents.push_str("\n\n");
    file_contents.push_str(contents);

    println!("Writing the get_page_by_id controller to the file...");
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&path)
        .unwrap();
    // Write the updated contents to the file
    writeln!(file, "{}", file_contents)?;

    file.flush()?;

    // Add the get_page_by_id controller to the main.rs file
    println!("Adding the get_page_by_id controller to the main.rs file...");
    add_new_controller_to_main_rs(None,Some("page"), "get_page_by_id")
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


pub fn update_cargo_toml_for_grapesjs() -> Result<(), Error> {
    let contents = r##"
    [dependencies]
actix-cors = "0.6.4"
actix-web = "4.0.0-beta.8"
actix-files = "0.6.2"
actix-session = {version = "0.7.2", features = ["cookie-session"]}
actix-identity = "0.5.2"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0.82"
random-string = "1.0.0"
env_logger = "0.10.0"
local-ip-address = "0.5.0"
futures = "0.3.23"
tera = "1.17.1"
reqwest = "0.11"
rustyroad = "0.1.8-beta-2.0.5"
rand = "0.8.5"
chrono = { version = "0.4.24", features = ["serde"] }
base64 = "0.21.0"
dotenv = "0.15.0"
bcrypt = "0.14.0"
color-eyre = "0.6.2"
serde_derive = "1.0.189"
tz = "0.2.1"
actix-multipart = "0.6.1"
mime = "0.3.17"
image = "0.24.7"
sanitize-filename = "0.5"

[dependencies.sqlx]
features = ["postgres", "macros", "chrono", "json", "uuid", "runtime-tokio", "time"]
version = "0.7.2"
    "##;

    println!("Updating the Cargo.toml file...");

    // Read the existing Cargo.toml file
    let mut cargo_toml = fs::read_to_string("./Cargo.toml")?;

    // Remove the current dependencies section
    if let Some(dependencies_start) = cargo_toml.find("[dependencies]") {
        cargo_toml.truncate(dependencies_start);
    }

    // Append the new dependencies
    cargo_toml.push_str("\n\n");
    cargo_toml.push_str(contents);

    // Write the updated contents to the file
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("./Cargo.toml")?;

    writeln!(file, "{}", cargo_toml)?;

    file.flush()?;

    println!("Cargo.toml updated successfully.");


    Ok(())
}


