use crate::generators::create_file;
use crate::helpers::helpers::add_or_update_import;
use crate::writers::write_to_file;
use crate::Project;
use std::io::Error;

// Write to index controller
pub fn write_to_index_controller(project: &Project) -> Result<(), Error> {
    let contents = r#"use actix_web::{get, web, HttpResponse};
use tera::{Context, Tera};

#[get("/")]
async fn index(tmpl: web::Data<Tera>) -> HttpResponse {
    let mut context = Context::new();
    context.insert("foo", "123");
    let rendered = tmpl.render("pages/index.html.tera", &context).unwrap();
    HttpResponse::Ok().body(rendered)
}"#
    .to_string();

    write_to_file(&project.index_controller.to_string(), contents.as_bytes()).unwrap_or_else(
        |why| {
            println!("Couldn't write to {}: {}", project.index_controller, why);
        },
    );
    Ok(())
}

/// # Name: write_to_dashboard_controller
/// ### Description:
/// - Writes to the dashboard controller
/// - It creates the dashboard_controller method which is used to render the dashboard page
/// - It also adds the dashboard controller to the main.rs file
/// # Arguments:
/// * project: &Project
/// # Returns:
/// * Result<(), Error>
/// # Example:
/// ```
/// use eyre::Error;
/// use rustyroad::writers::write_to_dashboard_controller;
/// use rustyroad::Project;
/// use std::path::PathBuf;
/// use std::env;
///
/// let mut project = Project::new();
/// project.name = "test".to_string();
/// project.dashboard_controller = PathBuf::from(format!("{}/src/controllers/dashboard.rs", env::current_dir().unwrap().to_str().unwrap()));
///
/// write_to_dashboard_controller(&project);
/// ```
pub fn write_to_dashboard_controller(project: &Project) -> Result<(), Error> {
    // ensure the file exists
    create_file(&project.dashboard_controller.to_string()).unwrap_or_else(|why| {
        println!("Couldn't create file: {}", why);
    });

    let contents = r#"
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
"#
    .to_string();

    // Update imports
    let mut import_contents = add_or_update_import("", "actix_web", "get");
    import_contents = add_or_update_import(&import_contents, "actix_web", "get");
    import_contents = add_or_update_import(&import_contents, "actix_web", "HttpResponse");
    import_contents = add_or_update_import(&import_contents, "tera", "Context");
    import_contents = add_or_update_import(&import_contents, "tera", "Tera");
    import_contents = add_or_update_import(&import_contents, "actix_identity", "Identity");
    import_contents = add_or_update_import(&import_contents, "actix_web", "Error");
    import_contents = add_or_update_import(&import_contents, "actix_web", "http::header::LOCATION");
    import_contents = add_or_update_import(&import_contents, "actix_web", "web::Data");

    import_contents.push_str("\n\n");
    import_contents.push_str(&contents);

    // Write to file
    write_to_file(
        &project.dashboard_controller.to_string(),
        import_contents.as_bytes(),
    )
    .expect("Couldn't write to dashboard controller");

    println!(
        "Test 1 for dashboard controller: {}",
        project.dashboard_controller.as_str()
    );
    Ok(())
}

// Write to not_found controller
pub fn write_to_not_found_controller(project: &Project) -> Result<(), Error> {
    let contents = r#"
#[get("/not_found")]
async fn not_found(tmpl: Data<Tera>) -> HttpResponse {
    let mut context = tera::Context::new();
    context.insert("controller_name", "not_found");
    let rendered = tmpl
        .render("pages/404.html.tera", &context)
        .unwrap_or_else(|err| {
            eprintln!("Template rendering error: {}", err);
            String::from("Server error")
        });

    HttpResponse::NotFound().body(rendered)
}"#
    .to_string();

    // Update imports
    let mut import_contents = add_or_update_import("", "actix_web", "get");
    import_contents = add_or_update_import(&import_contents, "actix_web", "HttpResponse");
    import_contents = add_or_update_import(&import_contents, "tera", "Context");
    import_contents = add_or_update_import(&import_contents, "tera", "Tera");
    import_contents = add_or_update_import(&import_contents, "actix_web", "web::Data");

    // Add the new controller content to the file
    import_contents.push_str("\n\n");
    import_contents.push_str(&contents);

    // Write to file
    write_to_file(
        &project.not_found_controller.to_string(),
        import_contents.as_bytes(),
    )
    .expect("Couldn't write to not_found controller");
    Ok(())
}
