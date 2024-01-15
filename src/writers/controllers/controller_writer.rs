use crate::helpers::helpers::*;
use crate::writers::{add_new_controller_to_main_rs, write_to_file, write_to_module};
use color_eyre;
use color_eyre::eyre::Result;
use eyre::*;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use color_eyre::owo_colors::OwoColorize;

/// # Name: write_to_controller_name_html
/// This function generates the html for a controller's view.
/// It is a standard template that can be used for any controller.
/// # Arguments:
/// * controller_name: &str
/// # Returns:
/// * Result<(), Error>
/// # Example:
/// ```
/// use rustyroad::writers::write_to_controller_name_html;
/// write_to_controller_name_html("test").expect("Error writing to controllerName.html.tera");
/// ```
pub fn write_to_controller_name_html(controller_name: &str) -> Result<(), Error> {
    let contents = format!(
        r#"{{% extends 'base.html.tera' %}}
{{% block title %}}Index{{% endblock title %}}
{{% block head %}}
{{{{ super() }}}}
{{% endblock head %}}
{{% block content %}}
<div class='relative px-6 lg:px-8'>
<div class='mx-auto  max-w-2xl py-32 sm:py-48 lg:py-56' >
<h1 class='text-4xl sm:text-5xl lg:text-6xl font-extrabold leading-none mb-4'>Your controller's Name: {{{{controller_name}}}}</h1>
<p class='text-xl sm:text-2xl lg:text-3xl font-medium mb-8'>This is a rustyroad project</p>
</div>
</div>
{{% endblock content %}}"#
    );

    // write to the file
    write_to_file(
        &format!("src/views/pages/{}.html.tera", controller_name).to_string(),
        contents.as_bytes(),
    )
    .unwrap_or_else(|why| {
        println!(
            "Couldn't write to {}: {}",
            &format!("./views/pages/{}.html.tera", controller_name).to_string(),
            why.to_string()
        );
    });
    Ok(())
}

/// # Name: write_to_controller_name_html_with_authorized_view
/// This function generates the html for a controller's view with an authorized view.
/// It is a standard template that can be used for any controller that requires authorization.
/// # Arguments:
/// * controller_name: &str
/// * folder_name: &str
/// # Returns:
/// * Result<(), Error>
/// # Example:
/// ```
/// use rustyroad::writers::write_to_controller_name_html_with_authorized_view;
/// write_to_controller_name_html_with_authorized_view("test", "authenticated_page").expect("Error writing to controllerName.html.tera");
/// ```
pub fn write_to_controller_name_html_with_authorized_view(
    controller_name: &str,
    folder_name: &str,
) -> Result<(), Error> {
    let contents = format!(
        r"{{% extends 'layouts/authenticated_page/{}.html.tera' %}}
            {{% block title %}}Index{{% endblock title %}}
            {{% block head %}}
            {{{{ super() }}}}
            {{% endblock head %}}
            {{% block content %}}
            <div class='relative px-6 lg:px-8'>
            <div class='mx-auto  max-w-2xl py-32 sm:py-48 lg:py-56' >
            <h1 class='text-4xl sm:text-5xl lg:text-6xl font-extrabold leading-none mb-4'>Your controller's Name: {{{{controller_name}}}}</h1>
            <p class='text-xl sm:text-2xl lg:text-3xl font-medium mb-8'>This is a rustyroad project</p>
            </div>
            </div>
            {{% endblock content %}}",
        folder_name
    );

    // write to the file
    write_to_file(
        &format!(
            "src/views/layouts/authenticated_page/{}/{}.html.tera",
            folder_name, controller_name
        )
        .to_string(),
        contents.as_bytes(),
    )
    .unwrap_or_else(|why| {
        println!(
            "Couldn't write to {}: {}",
            &format!(
                "./views/layouts/authenticated_page/{}/{}.html.tera",
                folder_name, controller_name
            )
            .to_string(),
            why.to_string()
        );
    });
    Ok(())
}

/// # Name: write_to_new_get_all_controller
/// This function writes a new GET controller that gets all of the items of a model to a Rust source file.
/// # Arguments:
/// * `model_name` - The name of the model that the controller will get all of the items of.
/// * `path` - The path to the file that the controller will be written to.
/// # Returns:
/// * `Ok(())` if the content was successfully written to the file, or an Error if something went wrong.
/// # Example:
/// ```
/// use rustyroad::writers::write_to_new_get_all_controller;
/// write_to_new_get_all_controller("user".to_string()).expect("Error writing to user.rs");
/// ```
pub fn write_to_new_get_all_controller(model_name: String) -> Result<(), Error> {
    // look for the model in the models folder
    let model_path = format!("./src/models/{}.rs", model_name);
    let model_file_path = PathBuf::from(&model_path);
    if !model_file_path.exists() {
        println!(
            "The model {} does not exist. Would you like to create it? (y/n)",
            model_name
        );
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        if input.trim().eq_ignore_ascii_case("y") {
            // Logic to create the model, for example `write_to_page_model` or similar
        } else {
            return Err(eyre!(
                "The model {} does not exist. Please create it and try again.",
                model_name
            ));
        }
    }

    let capitalize_model_name = capitalize_first(&model_name); // Assumed to be defined elsewhere

    let controller_contents = format!(
        r#"#[get("/{}/all")]
pub async fn get_all_{}s() -> HttpResponse {{
    let result = {}::get_all_{}s().await;
    match result {{
        Ok({}) => HttpResponse::Ok().json({}),
        Err(e) => HttpResponse::BadRequest().json(e.to_string()),
    }}
}}

"#,
        model_name,
        model_name,
        capitalize_model_name,
        model_name,
        capitalize_model_name,
        capitalize_model_name
    );

    // Determine the controller file path
    let path = determine_controller_path(&model_name);

    // Ensure the controller file exists, or create it
    let file_exists = PathBuf::from(&path).exists();
    if !file_exists {
        prompt_to_create_controller(&path).expect("Error prompting to create controller");
    }

    // Read and update the file contents
    let mut file_contents = fs::read_to_string(&path)?;

    // Update imports
    file_contents = add_or_update_import(&file_contents, "actix_web", "web");
    file_contents = add_or_update_import(&file_contents, "actix_web", "get");
    file_contents = add_or_update_import(&file_contents, "actix_web", "HttpResponse");
    file_contents = add_or_update_import(&file_contents, "crate", "models");
    file_contents = add_or_update_import(&file_contents, "models",&capitalize_model_name);
    file_contents = add_or_update_import(&file_contents, "actix_web", "web::Data");
    file_contents = add_or_update_import(&file_contents, "actix_web", "web::Json");

    // Add the new controller content to the file
    file_contents.push_str("\n\n");
    file_contents.push_str(&controller_contents);

    // Write the updated contents to the file
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&path)
        .unwrap();
    // Write the updated contents to the file
    writeln!(file, "{}", file_contents)?;

    file.flush()?;

    add_new_controller_to_main_rs(None,Some(&model_name), &format!("get_all_{}s", &model_name))?; // Assuming this function exists

    println!("Successfully written to {}.rs", model_name);
    Ok(())
}



/// This function writes a new Actix Web controller handler function to a Rust source file.
///
/// # Arguments
///
/// * `model_name` - The name of the model, which is used to name the file, the handler function, and the URL path of the controller.
///
/// # Returns
///
/// * `Ok(())` if the content was successfully written to the file, or an Error if something went wrong.
pub fn write_to_new_get_controller(model_name: String) -> Result<(), Error> {
    // look for the model in the models folder
    let model_path = format!("./src/models/{}.rs", model_name);
    let model_file_path = PathBuf::from(&model_path);
    if !model_file_path.exists() {
        println!(
            "The model {} does not exist. Would you like to create it? (y/n)",
            model_name
        );
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        if input.trim().eq_ignore_ascii_case("y") {
            // Logic to create the model, for example `write_to_page_model` or similar
        } else {
            return Err(eyre!(
                "The model {} does not exist. Please create it and try again.",
                model_name
            ));
        }
    }

    // Define the contents to be written to the file
    // This includes importing necessary Actix Web and Tera modules, defining the controller handler function,
    // and setting up the Tera template rendering
    let new_controller_content = format!(
        r#"  /// Alert: This is a generated controller.
        /// The controller is generated by the rustyroad CLI.
        /// It is a best guess at what the controller should look like.
        /// Please review the controller and make any necessary changes.
        #[get("/{}")]
        pub async fn get_{}(tmpl: web::Data<Tera>) -> HttpResponse {{
            let mut context = Context::new();
            context.insert("title", "{}");
            context.insert("controller_name", "{}");
            let rendered = tmpl.render("pages/{}.html.tera", &context).unwrap();
            HttpResponse::Ok().body(rendered)
        }}"#, &model_name, &model_name, &model_name, &model_name, &model_name
    );

    // Determine the controller file path
    let path = determine_controller_path(&model_name);

    // Ensure the controller file exists, or create it
    let file_exists = PathBuf::from(&path).exists();
    if !file_exists {
        prompt_to_create_controller(&path).expect("Error prompting to create controller");
    }

    // Read and update the file contents
    let mut file_contents = fs::read_to_string(&path)?;

    // Update imports
    file_contents = add_or_update_import(&file_contents, "actix_web", "get");
    file_contents = add_or_update_import(&file_contents, "actix_web", "web");
    file_contents = add_or_update_import(&file_contents, "actix_web", "HttpResponse");
    file_contents = add_or_update_import(&file_contents, "tera", "Context");
    file_contents = add_or_update_import(&file_contents, "tera", "Tera");

    // Add the new controller content to the file
    file_contents.push_str("\n\n");
    file_contents.push_str(&new_controller_content);
    file_contents.push_str("\n\n");

    // Write the updated contents to the file
    fs::write(PathBuf::from(&path), file_contents.as_bytes())?;

    add_new_controller_to_main_rs(None,Some(&model_name), &format!("get_{}", &model_name))?; // Assuming this function exists

    println!("Successfully written to {}", &path);

    Ok(())

}

/// # Name: write_to_initial_get_controller_authorized_view
/// This function writes a new GET controller that requires authentication to a Rust source file.
/// # Arguments:
/// * `controller_name` - The name of the controller, which is used to name the file, the handler function, and the URL path of the controller.
/// * `folder_name` - The name of the folder that the controller will be written to.
/// # Returns:
/// * `Ok(())` if the content was successfully written to the file, or an Error if something went wrong.
/// # Example:
/// ```
/// use rustyroad::writers::write_to_initial_get_controller_authorized_view;
/// write_to_initial_get_controller_authorized_view("user".to_string());
/// ```
pub fn write_to_initial_get_controller_authorized_view(model_name: String) -> Result<(), Error> {
    let capitalized_model_name = capitalize_first(&model_name);
    // look for the model in the models folder
    let model_path = format!("./src/models/{}.rs", model_name);
    let model_file_path = PathBuf::from(&model_path);
    if !model_file_path.exists() {
        println!(
            "The model {} does not exist. Would you like to create it? (y/n)",
            model_name
        );
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        if input.trim().eq_ignore_ascii_case("y") {
            // Logic to create the model, for example `write_to_page_model` or similar
        } else {
            return Err(eyre!(
                "The model {} does not exist. Please create it and try again.",
                model_name
            ));
        }
    }

    // Define the contents to be written to the file
    let new_controller_content = format!(
        r#"
#[get("/{}")]
pub async fn {}_controller_with_authorized_view(
    tmpl: web::Data<Tera>,
    user: Option<Identity>
) -> HttpResponse {{
    if let Some(_user) = user {{
        let mut context = Context::new();
        context.insert("username", &user.id().unwrap());
        context.insert("title", "{}");
        context.insert("controller_name", "{}");
        let rendered = tmpl.render("layouts/authenticated_page/{}/{}.html.tera", &context).unwrap();
        HttpResponse::Ok().body(rendered)
    }} else {{
        let mut context = Context::new();
        context.insert("title", "Login");
        context.insert("route_name", "login");
        context.insert("error", "You must be logged in to view this page.");
        HttpResponse::Found()
            .append_header((actix_web::http::header::LOCATION, "/login"))
            .finish()
    }}
}}"#,
        &model_name,
        &model_name,
        &model_name,
        &model_name,
        &model_name,
        &model_name
    );

    // Determine the controller file path
    let path = determine_controller_path(&model_name);

    // Ensure the controller file exists, or create it
    let file_exists = PathBuf::from(&path).exists();
    if !file_exists {
        prompt_to_create_controller(&path).expect("Error prompting to create controller");
    }

    // Read and update the file contents
    let mut file_contents = fs::read_to_string(&path)?;

    // Update imports
    file_contents = add_or_update_import(&file_contents, "actix_web", "get");
    file_contents = add_or_update_import(&file_contents, "actix_web", "web");
    file_contents = add_or_update_import(&file_contents, "actix_web", "HttpResponse");
    file_contents = add_or_update_import(&file_contents, "actix_web", "HttpRequest");
    file_contents = add_or_update_import(&file_contents, "actix_identity", "Identity");
    file_contents = add_or_update_import(&file_contents, "crate", "models");
    file_contents = add_or_update_import(&file_contents, "tera", "Context");
    file_contents = add_or_update_import(&file_contents, "tera", "Tera");
    file_contents = add_or_update_import(&file_contents, "models::user", "UserLogin");
    file_contents = add_or_update_import(&file_contents, format!("models::{}", &model_name).as_str(), &capitalized_model_name);

    // Add the new controller content to the file
    file_contents.push_str("\n\n");
    file_contents.push_str(&new_controller_content);
    file_contents.push_str("\n\n");
    // Write the updated contents to the file
    fs::write(PathBuf::from(&path), file_contents.as_bytes())?;

    add_new_controller_to_main_rs(None,Some(&model_name), &format!("{}_controller_with_authorized_view", &model_name))?; // Assuming this function exists

    println!("Successfully written to {}.rs", model_name);

    Ok(())
}

/// # Name: write_to_new_post_controller
/// This function writes a new Actix Web controller handler function to a Rust source file.
/// # Arguments:
/// * `controller_name` - The name of the controller, which is used to name the file, the handler function, and the URL path of the controller.
/// # Returns:
/// * `Ok(())` if the content was successfully written to the file, or an Error if something went wrong.
/// # Example:
/// ```
/// use rustyroad::writers::write_to_new_post_controller;
/// write_to_new_post_controller("login".to_string());
/// ```
pub fn write_to_new_post_controller(model_name: String) -> Result<(), Error> {
    let capitalized_model_name = capitalize_first(&model_name);
    let controller_signature = format!("#[post(\"/{}/{{}}\")]", model_name);

    // look for the model in the models folder
    //let model_path = format!("./src/models/{}.rs", model_name);
    // let model_file_path = PathBuf::from(&model_path);
    // if !model_file_path.exists() {
    //     println!(
    //         "The model {} does not exist. Would you like to create it? (y/n)",
    //         model_name
    //     );
    //     let mut input = String::new();
    //     std::io::stdin().read_line(&mut input)?;
    //     if input.trim().eq_ignore_ascii_case("y") {
    //         // Logic to create the model, for example `write_to_page_model` or similar
    //     } else {
    //         return Err(eyre!(
    //             "The model {} does not exist. Please create it and try again.",
    //             model_name
    //         ));
    //     }
    // }

    // Define the contents to be written to the file
    let controller_contents =
        format!(r#"
/// Alert: This is a generated controller.
/// The controller is generated by the rustyroad CLI.
/// It is a best guess at what the controller should look like.
/// Please review the controller and make any necessary changes.
#[post("/{}")]
pub async fn create_{}({}: Json<{}>,user: Option<Identity>) -> HttpResponse {{
    if let Some(_user) = user {{
        let result = {}::create_{}({}.into_inner()).await;
        match result {{
            Ok(page) => HttpResponse::Ok().json(page),
            Err(e) => {{
                eprintln!("Error creating page: {{:?}}", e); // Log the error
                HttpResponse::BadRequest().json(e.to_string())
            }}
        }}
    }} else {{
      // redirect to login page
      let mut context = Context::new();
        context.insert("title", "Login");
        context.insert("route_name", "login");
        context.insert("error", "You must be logged in to create a new {}.");
        HttpResponse::Found()
            .append_header((actix_web::http::header::LOCATION, "/login"))
            .finish()
    }}
   // before we allow the user to create a new {} we need to check if they are logged in
   // if they are not logged in, we need to redirect them to the login page
}}
"#,
                &model_name,
                &model_name,
                &model_name,
                &capitalized_model_name,
                &capitalized_model_name,
                &model_name,
                &model_name,
                &model_name,
                &model_name
        );

    // Determine the controller file path
    let path = determine_controller_path(&model_name);

    // Ensure the controller file exists, or create it
    let file_exists = PathBuf::from(&path).exists();
    if !file_exists {
        prompt_to_create_controller(&path).expect("Error prompting to create controller");
    }

    // Read and update the file contents
    let mut file_contents = fs::read_to_string(&path)?;
    if !file_contents.contains(&controller_signature) {
        // Update imports
        file_contents = add_or_update_import(&file_contents, "actix_web", "web");

        file_contents = add_or_update_import(&file_contents, "actix_web", "post");
        file_contents = add_or_update_import(&file_contents, "actix_web", "HttpResponse");
        file_contents = add_or_update_import(&file_contents, "tera", "Context");
        file_contents = add_or_update_import(&file_contents, "tera", "Tera");
        file_contents = add_or_update_import(&file_contents, "actix_identity", "Identity");
        file_contents = add_or_update_import(&file_contents, "crate", "models");
        file_contents = add_or_update_import(&file_contents, "models", &capitalized_model_name);
        file_contents = add_or_update_import(&file_contents, "actix_web", "web::Json");
        file_contents = add_or_update_import(&file_contents, "actix_web", "web::Data");

        // Add the new controller content to the file
        file_contents.push_str("\n\n");
        file_contents.push_str(&controller_contents);

        println!("File contents: {}", &file_contents.red());

        println!("Controller contents: {}", &controller_contents.green());


        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&path)
            .unwrap();
        // Write the updated contents to the file
        writeln!(file, "{}", file_contents)?;

        file.flush()?;

        add_new_controller_to_main_rs(None,Some(&model_name), &format!("create_{}", &model_name))?; // Assuming this function exists

        println!("Successfully written to {}.rs", model_name);
        Ok(())
    } else {
        println!("The controller already exists.");
        Ok(())
    }
}

/// Note: This is the best working example of a controller writer.
/// # Name: write_to_new_delete_controller
/// This function writes a new Actix Web controller handler function to a Rust source file.
/// # Arguments:
/// * `controller_name` - The name of the controller, which is used to name the file, the handler function, and the URL path of the controller.
/// # Returns:
/// * `Ok(())` if the content was successfully written to the file, or an Error if something went wrong.
/// ## Important:
/// * This takes an argument of a model name, the model name and the method of the CRUD operation together make up the controller name.
/// # Example:
/// ```
/// use rustyroad::writers::write_to_new_delete_controller;
/// write_to_new_delete_controller("user".to_string());
/// ```
pub fn write_to_new_delete_controller(model_name: String) -> Result<(), Error> {
    let capitalized_model_name = crate::helpers::helpers::capitalize_first(&model_name);
    // look for the model in the models folder
    let model_path = format!("./src/models/{}.rs", model_name);
    let model_file_path = PathBuf::from(&model_path);
    if !model_file_path.exists() {
        println!(
            "The model {} does not exist. Would you like to create it? (y/n)",
            model_name
        );
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        if input.trim().eq_ignore_ascii_case("y") {
            // Logic to create the model, for example `write_to_page_model` or similar
        } else {
            return Err(eyre!(
                "The model {} does not exist. Please create it and try again.",
                model_name
            ));
        }
    }

    // Define the contents to be written to the file


    let contents = format!(
        r#"#[delete("/{}/{{id}}")]
        pub async fn delete_{}(id: Path<i32>, user: Option<Identity>) -> HttpResponse {{
            if let Some(_user) = user {{
                let result = {}::delete_{}(id.into_inner()).await;
                match result {{
                    Ok(_) => HttpResponse::Ok().json("Successfully deleted."),
                    Err(e) => HttpResponse::BadRequest().json(e.to_string()),
                }}
            }} else {{
                HttpResponse::Unauthorized().json("You must be logged in to delete.")
            }}
        }}"#,
        &model_name,
        &model_name,
        &capitalized_model_name,
        &model_name
    );

    // Determine the controller file path
    let path = determine_controller_path(&model_name);

    // Ensure the controller file exists, or create it
    let file_exists = PathBuf::from(&path).exists();
    if !file_exists {
        prompt_to_create_controller(&path).expect("Error prompting to create controller");
    }

    // Read and update the file contents
    let mut file_contents = fs::read_to_string(&path)?;

    // Update imports
    file_contents = add_or_update_import(&file_contents, "actix_web", "delete");
    file_contents = add_or_update_import(&file_contents, "actix_web", "web");
    file_contents = add_or_update_import(&file_contents, "actix_web", "HttpResponse");
    file_contents = add_or_update_import(&file_contents, "tera", "Context");
    file_contents = add_or_update_import(&file_contents, "tera", "Tera");
    file_contents = add_or_update_import(&file_contents, "actix_identity", "Identity");
    file_contents = add_or_update_import(&file_contents, "crate", "models");
    file_contents = add_or_update_import(&file_contents, "models::user", "UserLogin");
    file_contents = add_or_update_import(&file_contents, format!("models::{}", &model_name).as_str(), &capitalized_model_name);

    // Add the new controller content to the file
    file_contents.push_str("\n\n");
    file_contents.push_str(&contents);

    // Write the updated contents to the file
    fs::write(PathBuf::from(&path), file_contents.as_bytes())?;

    add_new_controller_to_main_rs(None,Some(&model_name), &format!("delete_{}", &model_name))?; // Assuming this function exists

    println!("Successfully written to {}.rs", model_name);

    Ok(())
}


/// # Name: write_to_new_update_controller
/// This function writes to a new update controller.
/// # Arguments:
/// * `controller_name` - The name of the controller, which is used to name the file, the handler function, and the URL path of the controller.
/// # Returns:
/// * `Ok(())` if the content was successfully written to the file, or an Error if something went wrong.
/// # Example:
/// ```
/// use rustyroad::writers::write_to_new_update_controller;
/// write_to_new_update_controller("user".to_string());
/// ```
pub fn write_to_new_update_controller(model_name: String) -> Result<(), Error> {
    let capitalized_model_name = capitalize_first(&model_name);
    // look for the model in the models folder
    let model_path = format!("./src/models/{}.rs", model_name);
    let model_file_path = PathBuf::from(&model_path);
    if !model_file_path.exists() {
        println!(
            "The model {} does not exist. Would you like to create it? (y/n)",
            model_name
        );
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        if input.trim().eq_ignore_ascii_case("y") {
            // Logic to create the model, for example `write_to_page_model` or similar
        } else {
            return Err(eyre!(
                "The model {} does not exist. Please create it and try again.",
                model_name
            ));
        }
    }

    // Define the contents to be written to the file
    let new_controller_content = format!(
        r#"#[patch("/{}/{{id}}")]
        pub async fn update_{}(id: Path<i32>, {}: Json<{}>, user: Option<Identity>) -> HttpResponse {{
            if let Some(_user) = user {{
                let result = {}::update_{}(id.into_inner(), {}.into_inner()).await;
                match result {{
                    Ok(page) => HttpResponse::Ok().json(page),
                    Err(e) => HttpResponse::BadRequest().json(e.to_string()),
                }}
            }} else {{
                HttpResponse::Unauthorized().json("You must be logged in to update.")
            }}
        }}"#,
        &model_name,
        &model_name,
        &model_name,
        &capitalized_model_name,
        &capitalized_model_name,
        &model_name,
        &model_name
    );

    // Determine the controller file path
    let path = determine_controller_path(&model_name);

    // Ensure the controller file exists, or create it
    let file_exists = PathBuf::from(&path).exists();
    if !file_exists {
        prompt_to_create_controller(&path).expect("Error prompting to create controller");
    }

    // Read and update the file contents
    let mut file_contents = fs::read_to_string(&path)?;

    // Update imports
    file_contents = add_or_update_import(&file_contents, "actix_web", "patch");
    file_contents = add_or_update_import(&file_contents, "actix_web", "HttpResponse");
    file_contents = add_or_update_import(&file_contents, "tera", "Context");
    file_contents = add_or_update_import(&file_contents, "tera", "Tera");
    file_contents = add_or_update_import(&file_contents, "actix_identity", "Identity");
    file_contents = add_or_update_import(&file_contents, "crate", "models");
    file_contents = add_or_update_import(&file_contents, "models", &capitalized_model_name);
    file_contents = add_or_update_import(&file_contents, "actix_web", "web::Json");
    file_contents = add_or_update_import(&file_contents, "actix_web", "web::Path");


    // Add the new controller content to the file
    file_contents.push_str("\n\n");
    file_contents.push_str(&new_controller_content);
    file_contents.push_str("\n\n");
    // Write the updated contents to the file
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&path)
        .unwrap();
    // Write the updated contents to the file
    writeln!(file, "{}", file_contents)?;

    file.flush()?;

    add_new_controller_to_main_rs(None,Some(&model_name), &format!("update_{}", &model_name))?; // Assuming this function exists

    println!("Successfully written to {}.rs", model_name);

    Ok(())
}

/// # Name: write_to_new_get_controller_with_authorized_view
/// This function writes to a new get controller with an authorized view.
/// # Arguments:
/// * `controller_name` - The name of the controller, which is used to name the file, the handler function, and the URL path of the controller.
/// * `folder_name` - The name of the folder that the controller will be written to.
/// # Returns:
/// * `Ok(())` if the content was successfully written to the file, or an Error if something went wrong.
/// # Example:
/// ```
/// use rustyroad::writers::write_to_new_get_controller_with_authorized_view;
/// write_to_new_get_controller_with_authorized_view("user".to_string());
/// ```
pub fn write_to_new_get_controller_with_authorized_view(model_name: String) -> Result<(), Error> {
    let controller_signature = format!("#[put(\"/{}/{{id}}\")]", &model_name);
    let _capitalized_model_name = crate::helpers::helpers::capitalize_first(&model_name);
    // look for the model in the models folder
    let model_path = format!("./src/models/{}.rs", model_name);
    let model_file_path = PathBuf::from(&model_path);
    if !model_file_path.exists() {
        println!(
            "The model {} does not exist. Would you like to create it? (y/n)",
            model_name
        );
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        if input.trim().eq_ignore_ascii_case("y") {
            // Logic to create the model, for example `write_to_page_model` or similar
        } else {
            return Err(eyre!(
                "The model {} does not exist. Please create it and try again.",
                model_name
            ));
        }
    }


    // Define the contents to be written to the file
    // This includes importing necessary Actix Web and Tera modules, defining the controller handler function,
    // and setting up the Tera template rendering
    let new_controller_content = format!(
        r#"#[get("/{}")]
                async fn authenticated_view_for_{}(
                    tmpl: web::Data<Tera>,
                    user: Option<Identity>
                ) -> impl Responder {{
                    if let Some(_user) = user
                    {{
                          let mut context = Context::new();
                          context.insert("username", &user.id().unwrap());
                          context.insert("title", "{}");
                          context.insert("controller_name", "{}");
                          let rendered = tmpl.render("layouts/authenticated_page/{}/{}.html.tera", &context).unwrap();
                          HttpResponse::Ok().body(rendered)
                    }} else {{
                        let mut context = Context::new();
                        context.insert("title", "Login");
                        context.insert("route_name", "login");
                        context.insert("error", "You must be logged in to view this page.");
                        HttpResponse::Found()
                            .append_header((actix_web::http::header::LOCATION, "/login"))
                            .finish()
                        }}
                    }}"#,
        &model_name,
        &model_name,
        &model_name,
        &model_name,
        &model_name,
        &model_name
    );

    // Determine the controller file path
    let path = determine_controller_path(&model_name);

    // Ensure the controller file exists, or create it
    let file_exists = PathBuf::from(&path).exists();
    if !file_exists {
        prompt_to_create_controller(&path).expect("Error prompting to create controller");
    }

    // Read and update the file contents
    let mut file_contents = fs::read_to_string(&path)?;

    if !file_contents.contains(&controller_signature) {
    // Update imports
    file_contents = add_or_update_import(&file_contents, "actix_web", "get");
    file_contents = add_or_update_import(&file_contents, "actix_web", "web");
    file_contents = add_or_update_import(&file_contents, "actix_web", "HttpResponse");
    file_contents = add_or_update_import(&file_contents, "tera", "Context");
    file_contents = add_or_update_import(&file_contents, "tera", "Tera");
    file_contents = add_or_update_import(&file_contents, "actix_identity", "Identity");
    file_contents = add_or_update_import(&file_contents, "crate", "models");
    file_contents = add_or_update_import(&file_contents, "models::user", "UserLogin");

    // Add the new controller content to the file
    file_contents.push_str("\n\n");
    file_contents.push_str(&new_controller_content);
    file_contents.push_str("\n\n");
    // Write the updated contents to the file
    fs::write(PathBuf::from(&path), file_contents.as_bytes())?;

    add_new_controller_to_main_rs(None,Some(&model_name), &format!("authenticated_view_for_{}", &model_name))?; // Assuming this function exists

    println!("Successfully written to {}.rs", model_name);

    Ok(())
    } else {
        println!("The controller already exists.");
        Ok(())
    }
}

/// # Name: write_to_previous_get_controller
/// This function writes a GET controller to a controller that already exists.
/// # Arguments:
/// * `previous_controller_name` - The name of the controller that the new controller will be written to.
/// * `new_controller_name` - The name of the new controller.
/// # Returns:
/// * `Ok(())` if the content was successfully written to the file, or an Error if something went wrong.
/// # Example:
/// ```
/// use rustyroad::writers::write_to_previous_get_controller;
/// write_to_previous_get_controller("user".to_string(), "get_user".to_string());
/// ```
pub fn write_to_previous_get_controller(previous_controller_name: String, new_controller_name: String) -> Result<(), Error> {
    let controller_signature = format!("#[get(\"/{}/{}\")]", &previous_controller_name, &new_controller_name);
    // Define the contents to be written to the file
    // This includes importing necessary Actix Web and Tera modules, defining the controller handler function,
    // and setting up the Tera template rendering
    let contents = format!(
        r#"

        #[get("/{}/{}")]
        async fn {}(tmpl: web::Data<Tera>) -> impl Responder {{
            let mut context = Context::new();
            context.insert("controller_name", "{}");
            let rendered = tmpl.render("pages/{}.html.tera", &context).unwrap();
            HttpResponse::Ok().body(rendered)
        }}"#,
        previous_controller_name,
        new_controller_name,
        new_controller_name,
        new_controller_name,
        new_controller_name
    );

    // Define the path to the file
    let path = format!(
        "./src/controllers/{}/{}.rs",
        previous_controller_name, previous_controller_name
    );

    // instead of overwriting the file, we need to append to the file
    // lets get the contents of the file first
    let mut file_contents = fs::read_to_string(&path).unwrap();
    if !file_contents.contains(&controller_signature) {
    // and then append the new contents to the file
    file_contents.push_str(&contents);

    write_to_file(&path, file_contents.as_bytes()).unwrap_or_else(|why| {
        println!(
            "Failed to write to {}: {:?}",
            &path,
            why.kind()
        );
    });

    // Return Ok if everything succeeded
    Ok(())
    } else {
        println!("The controller already exists.");
        Ok(())
    }
}

/// # Name: write_to_previous_create_controller
/// This function writes a POST controller to a controller that already exists.
/// # Arguments:
/// * `previous_controller_name` - The name of the controller that the new controller will be written to.
/// * `new_controller_name` - The name of the new controller.
/// # Returns:
/// * `Ok(())` if the content was successfully written to the file, or an Error if something went wrong.
/// # Example:
/// ```
/// use rustyroad::writers::write_to_previous_create_controller;
/// write_to_previous_create_controller("user".to_string(), "create_user".to_string());
/// ```
pub fn write_to_previous_create_controller(previous_controller_name: String, new_controller_name: String) -> Result<(), Error> {
    let controller_signature = format!("#[post(\"/{}/{}\")]", previous_controller_name, new_controller_name);

    // Define the contents to be written to the file
    // This includes importing necessary Actix Web and Tera modules, defining the controller handler function,
    // and setting up the Tera template rendering
    let contents = format!(
        r#"

        use actix_identity::Identity;
        use actix_web::{{post, web, HttpResponse}};
        use crate::models::{};

        /// Alert: This is a generated controller.
        /// The controller is generated by the rustyroad CLI.
        /// It is a best guess at what the controller should look like.
        /// Please review the controller and make any necessary changes.
        #[post("/{}/{}")]
        pub async fn create_{}({}: web::Json<{}>,user: Option<Identity>) -> HttpResponse {{
            if let Some(_user) = user {{
                let result = {}::create_{}({}.into_inner()).await;
                match result {{
                    Ok(page) => HttpResponse::Ok().json(page),
                    Err(e) => HttpResponse::BadRequest().json(e.to_string()),
                }}
            }} else {{
              // redirect to login page
              let mut context = Context::new();
                context.insert("title", "Login");
                context.insert("route_name", "login");
                context.insert("error", "You must be logged in to create a new {}.");
                HttpResponse::Found()
                    .append_header((actix_web::http::header::LOCATION, "/login"))
                    .finish()
            }}
           // before we allow the user to create a new {} we need to check if they are logged in
           // if they are not logged in, we need to redirect them to the login page
        }}"#,
        &new_controller_name,
        &new_controller_name,
        &new_controller_name,
        &new_controller_name,
        &new_controller_name,
        &new_controller_name,
        &new_controller_name,
        &new_controller_name,
        &new_controller_name,
        &new_controller_name,
        &new_controller_name
    );

    // Define the path to the file
    let mut path = format!(
        "./src/controllers/{}/{}.rs",
        previous_controller_name, previous_controller_name
    );

    path = if std::path::Path::exists((&path).as_ref()) {
    path.to_string()
    } else {
        format!("./src/controllers/{}.rs", previous_controller_name)
    };

    // instead of overwriting the file, we need to append to the file
    // lets get the contents of the file first
    let mut file_contents = fs::read_to_string(&path).unwrap();
    if !file_contents.contains(&controller_signature) {
    // and then append the new contents to the file
    file_contents.push_str(&contents);

    write_to_file(&path, file_contents.as_bytes()).unwrap_or_else(|why| {
        println!(
            "Failed to write to {}: {:?}",
            &path,
            why.kind()
        );
    });

    // Return Ok if everything succeeded
    Ok(())
    } else {
        println!("The controller already exists.");
        Ok(())
    }
}

/// # Name: write_to_initial_get_controller
/// This function writes a new GET controller handler function to a Rust source file.
/// # Arguments:
/// * `controller_name` - The name of the controller, which is used to name the file, the handler function, and the URL path of the controller.
/// # Returns:
/// * `Ok(())` if the content was successfully written to the file, or an Error if something went wrong.
/// # Example:
/// ```
/// use rustyroad::writers::write_to_initial_get_controller;
/// write_to_initial_get_controller("user".to_string());
/// ```
pub fn write_to_initial_get_controller(controller_name: String) -> Result<(), Error> {
    // trim the controller_name to remove the text before the last slash and the text before the .rs
    let new_controller_name = controller_name
        .trim_start_matches("./src/controllers/")
        .trim_end_matches(".rs");

    let controller_file_name = std::path::Path::new(&controller_name)
        .file_name()
        .and_then(std::ffi::OsStr::to_str)
        .unwrap_or("");

    let contents = format!(
        r#"use actix_web::{{get, web, HttpResponse, HttpRequest, Error}};
use tera::{{Context, Tera}};
use crate::models;
use rustyroad::database::Database;
use models::user::UserLogin;

#[get("/{}")]
async fn {}_controller(tmpl: web::Data<Tera>) -> HttpResponse {{
    let mut context = Context::new();
    context.insert("controller_name", "{}");
    let rendered = tmpl.render("pages/{}.html.tera", &context).unwrap();
    HttpResponse::Ok().body(rendered)
}}"#,
        controller_file_name.trim_end_matches(".rs"),
        controller_file_name.trim_end_matches(".rs"),
        controller_file_name.trim_end_matches(".rs"),
        controller_file_name.trim_end_matches(".rs")
    );

    write_to_file(&controller_name.to_string(), contents.as_bytes()).unwrap_or_else(|why| {
        println!(
            "Failed to write to {}: {:?}",
            new_controller_name,
            why.kind()
        );
    });

    Ok(())
}

/// # Name: write_to_new_post_controller_authentication
/// This function writes a new Actix Web controller handler function to a Rust source file.
/// # Arguments:
/// * `controller_name` - The name of the controller, which is used to name the file, the handler function, and the URL path of the controller.
/// # Returns:
/// * `Ok(())` if the content was successfully written to the file, or an Error if something went wrong.
/// # Example:
/// ```
/// use rustyroad::writers::write_to_new_post_controller;
/// write_to_new_post_controller("login".to_string());
/// ```
pub fn write_to_initial_post_controller_authentication(controller_name: String) -> Result<(), Error> {
    // trim the controller_name to remove the text before the last slash and the text before the .rs
    let new_controller_name = controller_name
        .trim_start_matches("./src/controllers/")
        .trim_end_matches(".rs");

    let contents = r#"

 use actix_web::post;

#[post("/login")]
async fn login_function(
    form: web::Form<UserLogin>,
    tmpl: web::Data<Tera>, // Updated line
    db: web::Data<Database>,
    req: HttpRequest
) -> Result<HttpResponse, actix_web::Error> {
     form.user_login(req, tmpl, db.get_ref().clone()).await
}


#[get("/logout")]
async fn user_logout(
    tmpl: web::Data<Tera>,
    user: Option<actix_identity::Identity>,
) -> Result<HttpResponse, Error> {
    if let Some(user) = user {
        UserLogin::user_logout(tmpl, user).await
   } else {
         let mut context = Context::new();
         context.insert("controller_name", "login");
         context.insert("error", "You must be logged in to logout.");
         let rendered = tmpl.render("pages/login.html.tera", &context).unwrap();
         Ok(HttpResponse::Ok().body(rendered))
   }
}
"#
    .to_string();

    write_to_file(&controller_name.to_string(), contents.as_bytes()).unwrap_or_else(|why| {
        println!(
            "Failed to write to {}: {:?}",
            new_controller_name,
            why.kind()
        );
    });
    Ok(())
}

/// # Name: write_to_controller_file_no_folder
/// ### Description:
/// - This function updates an existing controller file in the `src/controllers` directory of an Actix Web project.
/// - It adds new imports and methods to the controller file without creating a new folder.
/// - The function reads the existing content of the file, updates it with new imports and methods, and then writes the updated content back to the file.
///
/// ### Arguments:
/// * `model_name` - The name of the model associated with the controller. This is used to name the file and identify the controller within the project.
/// * `import_contents` - A string containing import statements to be added to the controller file.
/// * `method_contents` - A string containing method definitions to be added to the controller file.
///
/// ### Returns:
/// * The function does not return a value but prints a success message upon successful completion or an error message in case of failure.
///
/// ### Example:
/// ```
/// use rustyroad::writers::write_to_controller_file_no_folder;
///
/// let model_name = "user";
/// let import_contents = "use actix_web::{web, HttpResponse};\nuse crate::models::User;";
/// let method_contents = "async fn get_user() -> HttpResponse {\n /* method implementation */ \n}";
///
/// write_to_controller_file_no_folder(model_name.to_string(), import_contents.to_string(), method_contents.to_string());
/// ```
pub fn write_to_controller_file_no_folder(model_name: String, import_contents: String, method_contents: String) {
    // Define the path to the file
    let path = format!("./src/controllers/{}.rs", model_name);

    // Read the contents of the file so we don't overwrite it
    let file_contents = match fs::read_to_string(&path) {
        std::result::Result::Ok(contents) => contents,
        Err(_) => String::new(), // If the file doesn't exist, start with an empty string
    };

    // Update file contents with the new imports and methods
    let updated_imports = import_contents
        .split('\n')
        .fold(file_contents, |acc, import| {
            if !import.trim().is_empty() {
                let parts: Vec<&str> = import.split("::").collect();
                if parts.len() > 1 {
                    let module = parts[0].trim();
                    let import_item = parts[1].trim_matches('{').trim_matches('}').trim();
                    add_or_update_import(&acc, module, import_item)
                } else {
                    acc // if the import line is not in the expected format, leave it unchanged
                }
            } else {
                acc
            }
        });

    // Combine updated imports with the method contents
    let combined_contents = format!("{}\n\n{}", updated_imports, method_contents);

    match fs::write(PathBuf::from(&path), combined_contents.as_bytes()) {
        std::result::Result::Ok(()) => {
            add_new_controller_to_main_rs(None,Some(&model_name), &format!("create_{}", &model_name))
                .unwrap_or_else(|why| {
                    println!(
                        "Couldn't add the create_{} controller to the main.rs file: {}",
                        &model_name,
                        why.to_string()
                    );
                });

            let mut components = Vec::new();

            components.push(format!("{}", &model_name));

            let module_path = format!("src/controllers/{}.rs", &model_name);

            write_to_module(&module_path, components)
                .expect("Error writing the module to the controllers module");
            println!("Successfully written to {}.rs", model_name)
        }
        Err(e) => println!("Failed to write to {}.rs: {:?}", model_name, e),
    }
}
/// # Name: write_to_page_dashboard_get_controller
/// ### Description:
/// - Creates the page_dashboard controller in the dashboard.rs file.
/// - This controller is used to render the page_dashboard.html.tera template.
/// - The page_dashboard.html.tera template is used to display a list of pages to an authorized user.
///
/// ### Example:
/// ```
/// use rustyroad::writers::write_to_page_dashboard_get_controller;
///
/// write_to_page_dashboard_get_controller();
/// ```
pub fn write_to_page_dashboard_get_controller() -> Result<(), Error> {
    let controller_signature = "#[get(\"/page_dashboard\")]";
    // Define the contents to be written to the file
    // This includes importing necessary Actix Web and Tera modules, defining the controller handler function,
    // and setting up the Tera template rendering
    // Define the new controller content
    let new_controller_content = r#"
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
"#;

    // Define the path to the file
    let path = format!("./src/controllers/dashboard.rs",);

    // Write the contents to the file
    // The write_to_file function is assumed to be a function that takes a path and a byte slice and writes the bytes to the file at the path
    // If the file doesn't exist, the function will create it, and if it does exist, the function will overwrite it

    // Read the contents of the file so we don't overwrite it
    let mut file_contents = fs::read_to_string(path.clone())?;
    if !file_contents.contains(controller_signature) {
    // Update imports in the file contents
    file_contents = add_or_update_import(&file_contents, "crate", "models");
    file_contents = add_or_update_import(&file_contents, "models", "Page");
    file_contents = add_or_update_import(&file_contents, "actix_web", "get");
    file_contents = add_or_update_import(&file_contents, "actix_web", "web");
    file_contents = add_or_update_import(&file_contents, "actix_web", "HttpResponse");
    file_contents = add_or_update_import(&file_contents, "actix_web", "http::header::LOCATION");
    file_contents = add_or_update_import(&file_contents, "tera", "Context");
    file_contents = add_or_update_import(&file_contents, "tera", "Tera");
    file_contents = add_or_update_import(&file_contents, "actix_identity", "Identity");
    file_contents = add_or_update_import(&file_contents, "serde_json", "json");




    // Add two new lines to the end of the file
    file_contents.push_str("\n\n");

    // Add the new controller content to the file
    file_contents.push_str(new_controller_content);

    // Write the updated contents to the file
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&path)
        .unwrap();
    // Write the updated contents to the file
    writeln!(file, "{}", file_contents)?;

    file.flush()?;

    println!("Successfully written to dashboard.rs");

    // add the page_dashboard controller to the main.rs file
    add_new_controller_to_main_rs(None,None, "page_dashboard")?;

    Ok(())
    } else {
        println!("The controller already exists.");
        Ok(())
    }
}

/// # Name: write_to_create_page_dashboard_get_controller
/// ### Description:
/// - Creates the create_page_dashboard controller in the dashboard.rs file.
/// - This controller is used to render the create_page_dashboard.html.tera template.
///
/// ### Example:
/// ```
/// use rustyroad::writers::write_to_create_page_dashboard_get_controller;
/// write_to_create_page_dashboard_get_controller();
/// ```
pub fn write_to_create_page_dashboard_get_controller() -> Result<(), Error> {
    let controller_signature = "#[get(\"/create_page\")]";
    // Define the contents to be written to the file
    // This includes importing necessary Actix Web and Tera modules, defining the controller handler function,
    // and setting up the Tera template rendering
    // Define the new controller content
    let new_controller_content = r#"
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
"#;

    let path = format!("./src/controllers/dashboard.rs",);

    let mut file_contents = fs::read_to_string(path.clone())?;
    if !file_contents.contains(controller_signature) {
    // Add two new lines to the end of the file
    file_contents.push_str("\n\n");

    // Add the new controller content to the file
    file_contents.push_str(new_controller_content);

    // Write the updated contents to the file
    fs::write(PathBuf::from(path), file_contents.as_bytes())?;

    println!("Successfully written to dashboard.rs");

    // add the create_page_dashboard controller to the main.rs file
    add_new_controller_to_main_rs(None,None, "create_page_dashboard")?;

    Ok(())
    } else {
        println!("The controller already exists.");
        Ok(())
    }
}


pub fn  write_to_edit_page_get_controller() -> Result<(), Error> {

    let controller_signature = "#[get(\"/page/{id}/edit\")]";
    // Define the contents to be written to the file
    // This includes importing necessary Actix Web and Tera modules, defining the controller handler function,
    // and setting up the Tera template rendering
    // Define the new controller content
    let new_controller_content = r#"
#[get("/page/{id}/edit")]
pub async fn edit_page(tmpl: Data<Tera>, id: Path<i32>, user: Option<Identity>) -> HttpResponse {
    if let Some(_user) = user {
        let result = Page::get_page_by_id(id.into_inner()).await;
        match result {
            Ok(page) => {
                let mut context = Context::new();
                context.insert("title", "Edit Page");
                context.insert("route_name", "edit_page");
                context.insert("html_content", &page.html_content);
                context.insert("page", &page);
                context.insert("page_id", &page.id);
                context.insert("username", &_user.id().unwrap());
                let s = tmpl.render("layouts/authenticated_page/page/edit_page.html.tera", &context).unwrap();
                HttpResponse::Ok().body(s)
            }
            Err(e) => {
                let mut context = Context::new();
                context.insert("error", &e.to_string());
                let s = tmpl.render("layouts/authenticated_page/page/edit_page.html.tera", &context).unwrap();
                HttpResponse::Ok().body(s)
            }
        }
    } else {
        HttpResponse::Unauthorized().json("You must be logged in to edit.")
    }
}
    "#;

    let path = format!("./src/controllers/page.rs");

    let mut file_contents = fs::read_to_string(path.clone())?;

    if !file_contents.contains(controller_signature) {
        file_contents = add_or_update_import(&file_contents, "crate", "models");
        file_contents = add_or_update_import(&file_contents, "models", "Page");
        file_contents = add_or_update_import(&file_contents, "tera", "Tera");
        file_contents = add_or_update_import(&file_contents, "tera", "Context");
        file_contents = add_or_update_import(&file_contents, "actix_web", "get");
        file_contents = add_or_update_import(&file_contents, "actix_web", "HttpResponse");
        file_contents = add_or_update_import(&file_contents, "actix_web", "web::Path");
        file_contents = add_or_update_import(&file_contents, "actix_web", "web::Data");
        file_contents = add_or_update_import(&file_contents, "actix_identity", "Identity");

        // Add two new lines to the end of the file
        file_contents.push_str("\n\n");

        // Add the new controller content to the file
        file_contents.push_str(new_controller_content);



        // Write the updated contents to the file
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&path)
            .unwrap();
        // Write the updated contents to the file
        writeln!(file, "{}", file_contents)?;

        file.flush()?;

        println!("Successfully written to dashboard.rs");

        // add the create_page_dashboard controller to the main.rs file
        add_new_controller_to_main_rs(None, Some(&"page"), "edit_page")?;

        Ok(())
    } else {
        println!("The edit_page controller already exists.");
        Ok(())
    }
}


pub fn write_to_new_dashboard_get_controller_to_test(file_path: &Path) -> Result<(), Error> {
    // Define the contents to be written to the file
    // This includes importing necessary Actix Web and Tera modules, defining the controller handler function,
    // and setting up the Tera template rendering
    // Define the new controller content
    let new_controller_content = r#"
#[get("/page_dashboard")]
async fn page_dashboard(tmpl: web::Data<Tera>, user: Option<Identity>) -> impl Responder {

    if let Some(_user) = user {

        let mut context = Context::new();
        let (pages, error) = match pages_result {
            Ok(pages) if pages.is_empty() => (Vec::new(), "No pages found".to_string()),
            Ok(pages) => (pages, "".to_string()),
            Err(e) => (Vec::new(), e.to_string()),
        };

        context.insert("pages", &pages);
        context.insert("error", &error);


        context.insert("username", &user.id().unwrap());
        context.insert("title", "Dashboard");
        context.insert("controller_name", "page_dashboard");

        let rendered = tmpl.render("layouts/authenticated_page/page/page_dashboard.html.tera", &context).unwrap();
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
}"#;
    // Write the contents to the file
    // The write_to_file function is assumed to be a function that takes a path and a byte slice and writes the bytes to the file at the path
    // If the file doesn't exist, the function will create it, and if it does exist, the function will overwrite it
    // Read the contents of the file so we don't overwrite it
    let mut file_contents = fs::read_to_string(file_path)?;

    // Update imports in the file contents
    file_contents = add_or_update_import(&file_contents, "actix_web", "get");
    file_contents = add_or_update_import(&file_contents, "actix_web", "web");
    file_contents = add_or_update_import(&file_contents, "actix_web", "HttpResponse");
    file_contents = add_or_update_import(&file_contents, "actix_web", "Responder");
    file_contents = add_or_update_import(&file_contents, "tera", "Tera");
    file_contents = add_or_update_import(&file_contents, "tera", "Context");

    // Add two new lines to the end of the file
    file_contents.push_str("\n\n");

    // Add the new controller content to the file
    file_contents.push_str(new_controller_content);

    // Write the updated contents to the file
    fs::write(PathBuf::from(file_path), file_contents.as_bytes())?;

    println!("Successfully written to dashboard.rs");
    Ok(())
}





#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::NamedTempFile;

    #[test]
    fn test_write_to_new_dashboard_get_controller() {
        // Create a temporary file
        let temp_file = NamedTempFile::new().unwrap();
        let file_path = temp_file.path().to_path_buf();

        // Write initial content to the temp file
        let initial_content = "use actix_web::{HttpResponse, Responder};\n";
        fs::write(&file_path, initial_content).unwrap();

        // Call the function with the path to the temp file
        write_to_new_dashboard_get_controller_to_test(&file_path).unwrap();

        // Read the updated content from the temp file
        let updated_content = fs::read_to_string(file_path).unwrap();

        // Perform your assertions here
        // Check if the new controller content is added and imports are correctly updated
        assert!(updated_content.contains("#[get(\"/page_dashboard\")]"));
        assert!(updated_content.contains("async fn page_dashboard"));
        // ... other necessary assertions
    }
}
