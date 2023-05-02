use std::fs;
use std::io::Error;
use std::path::PathBuf;


use crate::writers::write_to_file;

pub fn write_to_route_name_html(route_name: String) -> Result<(), Error> {
    let contents = format!(
        r#"{{% extends 'base' %}}
{{% block title %}}Index{{% endblock title %}}
{{% block head %}}
{{{{ super() }}}}
{{% endblock head %}}
{{% block content %}}
<div class='relative px-6 lg:px-8'>
<div class='mx-auto  max-w-2xl py-32 sm:py-48 lg:py-56' >
<h1 class='text-4xl sm:text-5xl lg:text-6xl font-extrabold leading-none mb-4'>Your Route's Name: {{{{route_name}}}}</h1>
<p class='text-xl sm:text-2xl lg:text-3xl font-medium mb-8'>This is a rustyroad project</p>
</div>
</div>
{{% endblock content %}}"#
    );

    // write to the file
    write_to_file(
        &format!("./templates/pages/{}.html.tera", route_name).to_string(),
        contents.as_bytes(),
    )
    .unwrap_or_else(|why| {
        println!(
            "Couldn't write to {}: {}",
            &format!("./templates/pages/{}.html.tera", route_name).to_string(),
            why.to_string()
        );
    });
    Ok(())
}


/// This function writes a new Actix Web route handler function to a Rust source file.
///
/// # Arguments
///
/// * `route_name` - The name of the route, which is used to name the file, the handler function, and the URL path of the route.
///
/// # Returns
///
/// * `Ok(())` if the content was successfully written to the file, or an Error if something went wrong.
pub fn write_to_route_name_rs(route_name: String) -> Result<(), Error> {
    // Define the contents to be written to the file
    // This includes importing necessary Actix Web and Tera modules, defining the route handler function,
    // and setting up the Tera template rendering
    let contents = format!(
        r#"use actix_web::{{get, web, HttpResponse}};
use tera::{{Context, Tera}};

#[get("/{}")]
async fn {}(tmpl: web::Data<Tera>) -> impl Responder {{
    let mut context = Context::new();
    context.insert("route_name", "{}");
    let rendered = tmpl.render("pages/{}.html.tera", &context).unwrap();
    HttpResponse::Ok().body(rendered)
}}"#,
        route_name, route_name, route_name, route_name
    );

    // Define the path to the file
    let path = format!("./src/routes/{}.rs", route_name);

    // Write the contents to the file
    // The write_to_file function is assumed to be a function that takes a path and a byte slice and writes the bytes to the file at the path
    // If the file doesn't exist, the function will create it, and if it does exist, the function will overwrite it
    match fs::write(PathBuf::from(&path), contents.as_bytes()) {
        Ok(()) => println!("Successfully written to {}.rs", route_name),
        Err(e) => println!("Failed to write to {}.rs: {:?}", route_name, e),
    }

    // Return Ok if everything succeeded
    Ok(())
}

pub fn write_to_initial_get_route_rs(route_name: String) -> Result<(), Error> {
    // trim the route_name to remove the text before the last slash and the text before the .rs
    let new_route_name = route_name
        .trim_start_matches("./src/routes/")
        .trim_end_matches(".rs");

    let route_file_name = std::path::Path::new(&route_name)
        .file_name()
        .and_then(std::ffi::OsStr::to_str)
        .unwrap_or("");

    let contents = format!(
        r#"use actix_web::{{get, web, HttpResponse, HttpRequest, Error}};
use tera::{{Context, Tera}};
use crate::models;
use models::user::UserLogin;

#[get("/{}")]
async fn {}_route(tmpl: web::Data<Tera>) -> HttpResponse {{
    let mut context = Context::new();
    context.insert("route_name", "{}");
    let rendered = tmpl.render("pages/{}.html.tera", &context).unwrap();
    HttpResponse::Ok().body(rendered)
}}"#,
        route_file_name.trim_end_matches(".rs"),
        route_file_name.trim_end_matches(".rs"),
        route_file_name.trim_end_matches(".rs"),
        route_file_name.trim_end_matches(".rs")
    );

    write_to_file(&route_name.to_string(), contents.as_bytes()).unwrap_or_else(|why| {
        println!("Failed to write to {}: {:?}", new_route_name, why.kind());
    });

    Ok(())
}

pub fn write_to_initial_post_route_rs(route_name: String) -> Result<(), Error> {
    // trim the route_name to remove the text before the last slash and the text before the .rs
    let new_route_name = route_name
        .trim_start_matches("./src/routes/")
        .trim_end_matches(".rs");

    let contents = r#"

 use actix_web::post;

#[post("/login")]
async fn login_function(
    form: web::Form<UserLogin>,
    tmpl: web::Data<Tera>, // Updated line
) -> Result<HttpResponse, actix_web::Error> {
    // get database data from rustyroad.toml

    let database = rustyroad::database::Database::get_database_from_rustyroad_toml().unwrap();

    form.user_login(tmpl, database).await
}


#[get("/logout")]
async fn user_logout(
    tmpl: web::Data<Tera>,
    req: HttpRequest, // Add the HttpRequest
) -> Result<HttpResponse, Error> {
 let database = rustyroad::database::Database::get_database_from_rustyroad_toml().unwrap();
    UserLogin::user_logout(tmpl, database, req).await
}
"#
    .to_string();

    write_to_file(&route_name.to_string(), contents.as_bytes()).unwrap_or_else(|why| {
        println!("Failed to write to {}: {:?}", new_route_name, why.kind());
    });
    Ok(())
}
