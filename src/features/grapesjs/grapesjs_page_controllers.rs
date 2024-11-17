use std::fs;
use std::fs::{OpenOptions, read_to_string};
use std::io::Write;
use std::path::PathBuf;
use color_eyre::owo_colors::OwoColorize;
use eyre::Error;
use crate::features::write_to_get_page_by_id;
use crate::generators::create_file;
use crate::helpers::helpers::{add_or_update_import, determine_controller_path, prompt_to_create_controller};
use crate::writers::{add_module_declaration, add_new_controller_to_main_rs, write_to_edit_page_get_controller, write_to_new_get_all_controller, write_to_new_post_controller, write_to_new_update_controller, write_to_page_dashboard_get_controller};

pub fn write_to_get_page_details_controller() -> Result<(), Error> {
    let contents = r#"
    #[get("/page/{id}/details")]
pub async fn get_page_details(tmpl: Data<Tera>, user: Option<Identity>, id: Path<i32> ) -> HttpResponse {
    let mut context = Context::new();
    context.insert("title", "page");
    context.insert("controller_name", "page");

    let rendered = if let Some(user) = user {
        context.insert("username", &user.id().unwrap());

        let page = Page::get_page_by_id(id.into_inner()).await;

        match page {
            Ok(page) => {
                context.insert("page", &page);
                context.insert("html_content", &page.html_content);
                tmpl.render(
                    "layouts/authenticated_page/page/page_details.html.tera",
                    &context,
                )
                .unwrap()
            }
            Err(e) => {
                let mut context = Context::new();
                context.insert("error", &e.to_string());
                let page = Page::new();
                context.insert("page", &page);
                context.insert("html_content", &page.html_content);
                context.insert("title", "Create Page");
                tmpl.render(
                    "layouts/authenticated_page/page/page_details.html.tera",
                    &context,
                )
                .unwrap()
            }
        }
    } else {
        // redirect to login page
        let mut context = Context::new();
        context.insert("title", "Login");
        context.insert("route_name", "login");
        context.insert("error", "You must be logged in to create a new page.");
        tmpl.render(
            "pages/login.html.tera",
            &context,
        )
        .unwrap()
    };

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(rendered)

}
"#.to_string();
    // Determine the controller file path
    let path = determine_controller_path(&"page");

    // Ensure the controller file exists, or create it
    let file_exists = PathBuf::from(&path).exists();
    if !file_exists {
        prompt_to_create_controller(&path).expect("Error prompting to create controller");
    }
    // Read and update the file contents
    let mut  file_contents = fs::read_to_string(&path).expect("Couldn't read get_page_details controller");


    file_contents = add_or_update_import(&file_contents, "actix_web", "get");
    file_contents = add_or_update_import(&file_contents, "actix_web", "web::Path");
    file_contents = add_or_update_import(&file_contents, "actix_web", "HttpResponse");
    file_contents = add_or_update_import(&file_contents, "tera", "Tera");
    file_contents = add_or_update_import(&file_contents, "tera", "Context");
    file_contents = add_or_update_import(&file_contents, "models", "Page");
    file_contents = add_or_update_import(&file_contents, "actix_identity", "Identity");
    file_contents = add_or_update_import(&file_contents, "actix_web", "web::Data");

    // Add the new controller content to the file
    file_contents.push_str("\n\n");
    file_contents.push_str(&contents);

    // Write to file
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&path)
        .unwrap();
    // Write the updated contents to the file
    writeln!(file, "{}", file_contents)?;

    file.flush()?;

    add_new_controller_to_main_rs(None, Some("page"), "get_page_details")?;

    Ok(())
}

pub fn write_to_get_page_by_slug_controller() -> Result<(), Error> {
    let contents = r#"

#[get("/page/{slug}")]
pub async fn get_page_by_slug(tmpl: Data<Tera>, slug: Path<String>, user: Option<Identity>) -> HttpResponse {
    let result = Page::get_page_by_slug(slug.into_inner()).await;
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
"#.to_string();

    // Determine the controller file path
    let path = determine_controller_path(&"page");

    // Ensure the controller file exists, or create it
    let file_exists = PathBuf::from(&path).exists();
    if !file_exists {
        prompt_to_create_controller(&path).expect("Error prompting to create controller");
    }

    // Update imports
    let mut file_contents = read_to_string(&path).expect("Couldn't read get_page_by_slug controller");
    file_contents = add_or_update_import(&file_contents, "actix_web", "get");
    file_contents = add_or_update_import(&file_contents, "actix_web", "get");
    file_contents = add_or_update_import(&file_contents, "tera", "Tera");
    file_contents = add_or_update_import(&file_contents, "tera", "Context");
    file_contents = add_or_update_import(&file_contents, "models", "Page");
    file_contents = add_or_update_import(&file_contents, "actix_web", "HttpResponse");
    file_contents = add_or_update_import(&file_contents, "actix_identity", "Identity");
    file_contents = add_or_update_import(&file_contents, "actix_web", "web::Data");
    file_contents = add_or_update_import(&file_contents, "actix_web", "web::Path");


    // Add the new controller content to the file
    file_contents.push_str("\n\n");
    file_contents.push_str(&contents);

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&path)
        .unwrap();
    // Write the updated contents to the file
    writeln!(file, "{}", file_contents)?;

    file.flush()?;


    add_new_controller_to_main_rs(None, Some("page"), "get_page_by_slug")?;
    Ok(())
}

pub fn delete_page_controller() -> Result<(), Error> {
    let contents = r#"
#[get("/page/{id}/delete")]
pub async fn delete_page(tmpl: Data<Tera>, user: Option<Identity>, id: Path<i32>) -> HttpResponse {
    let result = Page::delete_page(id.into_inner()).await;
    match result {
        Ok(_) => {
            let mut context = Context::new();
            if let Some(_user) = user {
                context.insert("username", &_user.id().unwrap());
            }
            context.insert("title", "Create Page");
            context.insert("route_name", "create_page");
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

"#.to_string();


    let path = determine_controller_path(&"page");

    let mut file_contents = fs::read_to_string(&path).expect("Couldn't read delete_page controller");

    // Update imports
    file_contents = add_or_update_import(&file_contents, "actix_web", "get");
    file_contents = add_or_update_import(&file_contents, "tera", "Tera");
    file_contents = add_or_update_import(&file_contents, "tera", "Context");
    file_contents = add_or_update_import(&file_contents, "models", "Page");
    file_contents = add_or_update_import(&file_contents, "actix_web", "HttpResponse");
    file_contents = add_or_update_import(&file_contents, "actix_identity", "Identity");
    file_contents = add_or_update_import(&file_contents, "actix_web", "web::Data");
    file_contents = add_or_update_import(&file_contents, "actix_web", "web::Path");

    println!("import contents {}", file_contents.red());

    // Add the new controller content to the file
    file_contents.push_str("\n\n");
    file_contents.push_str(&contents);

    // Write to file
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&path)
        .unwrap();
    // Write the updated contents to the file
    writeln!(file, "{}", file_contents)?;

    file.flush()?;

    Ok(())
}


pub fn write_to_image_upload_controller() -> Result<(), Error> {
    let contents = r#"
    use actix_web::guard::Host;
use actix_web::web::Buf;
use actix_web::{post, web::Data, HttpRequest, HttpResponse};
use futures::StreamExt as _;
use futures::TryStreamExt;
use serde_json::json;
use std::{fs, io::Write, path::PathBuf};

use actix_multipart::Multipart;
use image::imageops::FilterType;
use mime::{Mime, IMAGE_GIF, IMAGE_JPEG, IMAGE_PNG};
#[post("/image")]
async fn upload_image(mut payload: Multipart, req: HttpRequest) -> HttpResponse {
    let legal_filetypes: [Mime; 3] = [IMAGE_PNG, IMAGE_JPEG, IMAGE_GIF];
    let image_directory = PathBuf::from("static/images");

    // ensure image directory exists
    if !image_directory.exists() {
        if let Err(_) = fs::create_dir_all(&image_directory) {
            return HttpResponse::InternalServerError().json({
                json!({
                    "error": "Failed to create image directory.",
                    "status": 500
                })
            });
        }
    }

    let mut files_info = Vec::new();

    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_type().clone();

        // ensure file is an image
        if content_type.is_none() || !legal_filetypes.contains(&content_type.unwrap()) {
            return HttpResponse::BadRequest().json({
                json!({
                    "error": "File must be an image.",
                    "status": 400
                })
            });
        }

        let filename = field
            .content_disposition()
            .get_filename()
            .unwrap_or_default()
            .to_string();

        // Replace spaces in the filename with underscores
        let sanitized_filename = sanitize_filename::sanitize(&filename);

        let filepath = image_directory.join(&sanitized_filename);

        // create a path to save the file
        let mut file = match fs::File::create(&filepath) {
            Ok(file) => file,
            Err(_) => return HttpResponse::InternalServerError().json({
                json!({
                    "error": "Failed to create file.",
                    "status": 500
                })
            })
        };

        // copy the content of file into the file variable
        while let Ok(Some(chunk)) = field.try_next().await {
            if let Err(_) = file.write_all(&chunk) {
                return HttpResponse::InternalServerError().json({
                    json!({
                        "error": "Failed to write file.",
                        "status": 500
                    })
                });
            }
        }

        // create a thumbnail of the image
        match image::open(&filepath) {
            Ok(image) => {
                let thumbnail = image.thumbnail(100, 100);
                if let Err(_) = thumbnail.save(filepath.with_extension("thumb.jpg")) {
                    return HttpResponse::InternalServerError().json({
                        json!({
                            "error": "Failed to save thumbnail.",
                            "status": 500
                        })
                    });
                }
            }
            Err(_) => return HttpResponse::InternalServerError().json({
                json!({
                    "error": "Failed to open image.",
                    "status": 500
                })
            }),
        };

        files_info.push(json!({
            "filename": sanitized_filename.as_str(),
            "thumbnail": format!("{}://{}/images/{}",
                req.connection_info().scheme(),
                req.connection_info().host(),
                sanitized_filename.as_str()
            ),
            "filepath": format!("{}://{}/images/{}",
                req.connection_info().scheme(),
                req.connection_info().host(),
                sanitized_filename.as_str()
            )
        }));
    }

    // Check if any files were uploaded
    if files_info.is_empty() {
        return HttpResponse::BadRequest().json({
            json!({
                "error": "No file uploaded.",
                "status": 400
            })
        });
    }

    // On successful upload of multiple files, return JSON response
    HttpResponse::Ok().json({
        json!({
            "status": 200,
            "message": "Files uploaded successfully.",
            "files": files_info
        })
    })
}
"#.to_string();

    // Determine the controller file path
    let path = "./src/controllers/image.rs";

    // create the file
    create_file(&path).expect("Error creating image controller.");

    // Read and update the file contents
    let mut file_contents = read_to_string(&path).expect("Couldn't read image controller");

    file_contents.push_str("\n\n");
    file_contents.push_str(&contents);
    // Write the updated contents to the file
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&path)
        .unwrap();
    // Write the updated contents to the file
    writeln!(file, "{}", file_contents)?;

    file.flush()?;

    add_module_declaration("image".to_string(), std::path::Path::new("./src/controllers/mod.rs"))?;

    add_new_controller_to_main_rs(None, Some("image"), "upload_image")?;

    Ok(())
}

pub fn write_to_all_page_controllers() -> Result<(), Error> {
    write_to_edit_page_get_controller()?;
    write_to_get_page_by_slug_controller()?;
    write_to_new_post_controller("page".to_string())?;
    write_to_new_update_controller("page".to_string())?;
    write_to_get_page_by_id()?;
    write_to_new_get_all_controller("page".to_string())?;
    write_to_get_page_details_controller()?;
    write_to_page_dashboard_get_controller()?;
    delete_page_controller()?;
    Ok(())
}
pub mod tests {
    use std::fs::{self, create_dir};
    use std::path::PathBuf;
    use std::io::{self};

    fn setup_test_environment() -> io::Result<(tempfile::TempDir, PathBuf)> {
        let temp_dir = tempfile::tempdir()?;
        let src_dir = temp_dir.path().join("src");
        create_dir(&src_dir)?;
        // create a main.rs in the src directory
        let main_path = src_dir.join("main.rs");
        fs::File::create(&main_path)?;
        // write to mainrs
        let  main_contents = r##"
        use actix_cors::Cors;
use actix_files::Files;
use actix_identity::IdentityMiddleware;
use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;
use actix_web::{
    web::{self},
    App, HttpServer,
};
use color_eyre::eyre::Result;
use rustyroad::database::Database;
use std::env;
use tera::Tera;
mod controllers;
mod models;

fn get_secret_key() -> Result<Key, Box<dyn std::error::Error>> {
    let secret_key_from_env = env::var("SECRET_KEY")?;
    if secret_key_from_env.len() < 32 {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Secret key must be at least 32 characters",
        )));
    }
    let key = Key::from(secret_key_from_env.as_bytes());
    Ok(key)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    let database = web::Data::new(Database::get_database_from_rustyroad_toml().unwrap());

    println!("Starting Actix web server...");

    HttpServer::new(move || {
        let cors = Cors::permissive();
        // Load tera views from the specified directory
        let tera = Tera::new("src/views/**/*").unwrap();
        println!("Initializing Actix web application...");

        let secret_key = get_secret_key().unwrap();

        let session_mw = SessionMiddleware::builder(CookieSessionStore::default(), secret_key)
            // disable secure cookie for local testing
            .cookie_secure(false)
            .build();

        App::new()
            .wrap(
                actix_web::middleware::Logger::default()
                    .exclude("/static")
                    .exclude("/favicon.ico"),
            )
            .wrap(cors)
            .wrap(IdentityMiddleware::default())
            .app_data(database.clone())
            .wrap(session_mw)
            .app_data(web::Data::new(tera.clone())) // Updated line
            .service(controllers::index::index)
            .service(controllers::dashboard::dashboard_controller)
            .service(controllers::login::login_controller)
            .service(controllers::login::login_function)
            .service(controllers::login::user_logout)
            .service(controllers::page::create_page)
            .service(controllers::page::update_page)
            .service(controllers::page::get_page_by_id)
            .service(Files::new("/", "./static")) // Add this line
    })
    .bind(("127.0.0.1", 80))
    .unwrap()
    .workers(2)
    .run()
    .await
}
        "##.to_string();
        fs::write(&main_path, main_contents)?;
        let page_controller_dir = src_dir.join("controllers");
        create_dir(&page_controller_dir)?;

        let page_controller_path = page_controller_dir.join("page.rs");
        fs::File::create(&page_controller_path)?;

        Ok((temp_dir, page_controller_path))
    }

    #[test]
    fn test_write_to_all_page_controllers() {
        let (temp_dir, page_controller_path) = setup_test_environment()
            .expect("Failed to set up test environment");

        // change the current working directory to the temp directory
        std::env::set_current_dir(&temp_dir.path())
            .expect("Failed to change current working directory");

        write_to_all_page_controllers()
            .expect("Error writing to all page controllers");

        let  page_controller_contents = fs::read_to_string(&page_controller_path)
            .expect("Error reading page controller contents");

        println!("page_controller_contents: {}", page_controller_contents.purple());

        assert!(page_controller_contents.contains("use actix_web::HttpResponse;"),
                "Page controller does not contain expected content");
    }
}
