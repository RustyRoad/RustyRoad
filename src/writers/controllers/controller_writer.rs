use crate::generators::create_file;
use crate::writers::{add_new_controller_to_main_rs, write_to_file, write_to_module};
use color_eyre::eyre::Result;
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

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
/// write_to_controller_name_html("test");
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
/// write_to_controller_name_html_with_authorized_view("test", "authenticated_page");
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
    // Define the contents to be written to the file
    // This includes importing necessary Actix Web and Tera modules, defining the controller handler function,
    // and setting up the Tera template rendering
    let contents = format!(
        r#"\n
        \n
        use actix_web::{{get, web, HttpResponse, Responder}};
use tera::{{Context, Tera}};

#[get("/{}")]
async fn {}(tmpl: web::Data<Tera>) -> impl Responder {{
    let mut context = Context::new();
    context.insert("controller_name", "{}");
    let rendered = tmpl.render("pages/{}.html.tera", &context).unwrap();
    HttpResponse::Ok().body(rendered)
}}"#,
        model_name, model_name, model_name, model_name
    );

    // Define the path to the file
    let path = format!(
        "./src/controllers/{}/{}.rs",
        model_name, model_name
    );

    // Write the contents to the file
    // The write_to_file function is assumed to be a function that takes a path and a byte slice and writes the bytes to the file at the path
    // If the file doesn't exist, the function will create it, and if it does exist, the function will overwrite it

    // read the contents of the file so we don't overwrite it
    let mut file_contents = fs::read_to_string(&path).unwrap();

    // add two new lines to the end of the file
    file_contents.push_str("\n\n");

    // add the contents to the file
    file_contents.push_str(&contents);

    match fs::write(PathBuf::from(&path), file_contents.as_bytes()) {
        Ok(()) => println!("Successfully written to {}.rs", model_name),
        Err(e) => println!("Failed to write to {}.rs: {:?}", model_name, e),
    }

    // Return Ok if everything succeeded
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
    // look up the name of the model in the models folder

    let capitalized_model_name = crate::helpers::helpers::capitalize_first(&model_name);

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
        #[post("/{}")]
        pub async fn create_{}({}: web::Json<{}>,user: Option<Identity>) -> HttpResponse {{
            if let Some(user) = user {{
                let result = {}::create_{}({}.into_inner()).await;
                match result {{
                    Ok(page) => HttpResponse::Ok().json(page),
                    Err(e) => HttpResponse::BadRequest().json(e.to_string()),
                }}
            }} else {{
              // redirect to login page
              let mut context = tera::Context::new();
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
        &capitalized_model_name,
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

    // Determine if a folder exists for the controller
    if !PathBuf::from(&format!("./src/controllers/{}", model_name)).exists() {
        // If it doesn't exist, ask the user if they want to create it or add the controller to the controllers/authenticated_page file
        println!(
            "The folder ./src/controllers/{} does not exist. Would you like to create it? (y/n)",
            model_name
        );
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        // If the user wants to create the folder, create it
        if input.trim() == "y" {
            fs::create_dir(PathBuf::from(&format!(
                "./src/controllers/{}",
                model_name
            )))
            .unwrap();
            // define the path to the file
            let path = format!(
                "./src/controllers/{}/{}.rs",
                model_name, model_name
            );
            // Create the file
            std::fs::File::create(PathBuf::from(&path)).unwrap();
            // Add the controller to the controllers/authenticated_page file
            
            // read the contents of the file so we don't overwrite it
            let mut file_contents = fs::read_to_string(&path).unwrap();

            // add two new lines to the end of the file
            file_contents.push_str("\n\n");

            // add the contents to the file
            file_contents.push_str(&contents);

            match fs::write(PathBuf::from(&path), file_contents.as_bytes()) {
                Ok(()) => {
                    add_new_controller_to_main_rs(
                        Some(&model_name),
                        &format!("create_{}", &model_name),
                    )
                    .unwrap_or_else(|why| {
                        println!(
                            "Couldn't add the create_{} controller to the main.rs file: {}",
                            &model_name,
                            why.to_string()
                        );
                    });

                    let mut components = Vec::new();

                    components.push(format!("{}", &model_name));

                    let module_path = format!("src/controllers/{}/authenticated_page", &model_name);

                    // create the edit page module file
                    create_file(&module_path).expect("Couldn't create edit_page authenticated_page file");

                    println!("module_path: {}", &module_path);
                    write_to_module(&module_path, components)
                        .expect("Error writing the module to the controllers module");

                    println!("Successfully written to {}.rs", model_name)
                }
                Err(e) => println!("Failed to write to {}.rs: {:?}", model_name, e),
            }
        } else {
            // If the user doesn't want to create the folder, add the controller to the controllers/authenticated_page file and create the file
            // define the path to the file
            let path = format!("./src/controllers/{}.rs", model_name);
            // create the file

            // read the contents of the file so we don't overwrite it
            let mut file_contents = fs::read_to_string(&path).unwrap();

            // add two new lines to the end of the file
            file_contents.push_str("\n\n");

            // add the contents to the file
            file_contents.push_str(&contents);

            match fs::write(PathBuf::from(&path), file_contents.as_bytes()) {
                Ok(()) => {
                    add_new_controller_to_main_rs(
                        Some(&model_name),
                        &format!("create_{}", &model_name),
                    )
                    .unwrap_or_else(|why| {
                        println!(
                            "Couldn't add the create_{} controller to the main.rs file: {}",
                            &model_name,
                            why.to_string()
                        );
                    });


                    let mut components = Vec::new();

                    components.push(format!("{}", &model_name));

                    let module_path = format!("src/controllers/authenticated_page");

                    write_to_module(&module_path, components)
                        .expect("Error writing the module to the controllers module");
                    println!("Successfully written to {}.rs", model_name)
                }
                Err(e) => println!("Failed to write to {}.rs: {:?}", model_name, e),
            }
        }
    } else {
        // If the folder does exist, write the file to the folder
        // Define the path to the file
        let path = format!(
            "./src/controllers/{}/{}.rs",
            model_name, model_name
        );

        // Create the file
        std::fs::File::create(PathBuf::from(&path)).unwrap();
        // Add the controller to the controllers/authenticated_page file
        

        // Write the contents to the file
        // The write_to_file function is assumed to be a function that takes a path and a byte slice and writes the bytes to the file at the path
        // If the file doesn't exist, the function will create it, and if it does exist, the function will overwrite it

        // read the contents of the file so we don't overwrite it
        let mut file_contents = fs::read_to_string(&path).unwrap();

        // add two new lines to the end of the file
        file_contents.push_str("\n\n");

        // add the contents to the file
        file_contents.push_str(&contents);

        match fs::write(PathBuf::from(&path), file_contents.as_bytes()) {
            Ok(()) => {
                add_new_controller_to_main_rs(
                    Some(&model_name),
                    &format!("create_{}", &model_name),
                )
                .unwrap_or_else(|why| {
                    println!(
                        "Couldn't add the create_{} controller to the main.rs file: {}",
                        &model_name,
                        why.to_string()
                    );
                });

                let mut components = Vec::new();

                components.push(format!("{}", &model_name));

                let module_file = format!("src/controllers/{}/authenticated_page", &model_name);
                let module_file_path = std::path::Path::new(&module_file);

                if !module_file_path.exists() {
                    create_file(&module_file).expect("Couldn't create edit_page authenticated_page file");
                }

                write_to_module(&module_file, components)
                    .expect("Error writing the module to the controllers module");

                println!("Successfully written to {}.rs", model_name)
            }
            Err(e) => println!("Failed to write to {}.rs: {:?}", model_name, e),
        }
    }
    // we need to manually add the "create "

    // Return Ok if everything succeeded
    Ok(())
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
    let model_name = model_name

        .trim_start_matches("./src/models/")
        .trim_end_matches(".rs");

    if model_name != model_name {
        println!("model_name not found");
    }

    let capitalized_model_name = crate::helpers::helpers::capitalize_first(&model_name);

    let contents = format!(
        r#"
        use actix_identity::Identity;
        use actix_web::{{delete, web, HttpResponse}};
        use crate::models::{};

        #[delete("/{}/{{id}}")]
        pub async fn delete_{}(id: web::Path<i32>, user: Option<Identity>) -> HttpResponse {{
            if let Some(user) = user {{
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
        &model_name,
        &capitalized_model_name,
        &model_name,
    );

    let path = format!("./src/controllers/{}.rs", model_name);
    let folder_path = format!("./src/controllers/{}", model_name);

    if !PathBuf::from(&folder_path).exists() {
        println!(
            "The folder {} does not exist. Would you like to create it? (y/n)",
            folder_path
        );
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "y" {
            fs::create_dir(PathBuf::from(&folder_path)).unwrap();
        } else {
            return Err(Error::new(
                ErrorKind::Other,
                "Controller folder creation aborted by user",
            ));
        }
    }

    // read the contents of the file so we don't overwrite it
    let mut file_contents = fs::read_to_string(&path).unwrap();

    // add two new lines to the end of the file
    file_contents.push_str("\n\n");

    // add the contents to the file
    file_contents.push_str(&contents);

    match fs::write(PathBuf::from(&path), file_contents.as_bytes()) {
        Ok(()) => {
            add_new_controller_to_main_rs(Some(&model_name), &format!("delete_{}", &model_name)).unwrap();

            let mut components = Vec::new();
            components.push(format!("{}", &model_name));

            let module_path = format!("src/controllers/{}/authenticated_page", &model_name);
            write_to_module(&module_path, components).expect("Error writing the module to the controllers module");

            println!("Successfully written to {}.rs", model_name);
            Ok(())
        }
        Err(e) => Err(e),
    }
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
    let capitalized_model_name = crate::helpers::helpers::capitalize_first(&model_name);

    let contents = format!(
        r#"/// Alert: This is a generated controller.
        /// The controller is generated by the rustyroad CLI.
        /// It is a best guess at what the controller should look like.
        /// Please review the controller and make any necessary changes.
        #[actix_web::patch("/{}")]
        pub async fn update_{}(id: web::Path<i32>, {}: web::Json<{}>, user: Option<Identity>) -> HttpResponse {{
            if let Some(user) = user {{
                let result = {}::update_{}(*id, {}.into_inner()).await;
                match result {{
                    Ok(page) => HttpResponse::Ok().json(page),
                    Err(e) => HttpResponse::BadRequest().json(e.to_string()),
                }}
            }} else {{
                let mut context = tera::Context::new();
                context.insert("title", "Login");
                context.insert("route_name", "login");
                context.insert("error", "You must be logged in to update a {}.");
                HttpResponse::Found()
                    .append_header((actix_web::http::header::LOCATION, "/login"))
                    .finish()
            }}
        }}"#,
        &model_name,
        &model_name,
        &model_name,
        &capitalized_model_name,
        &capitalized_model_name,
        &model_name,
        &model_name,
        &model_name
    );

    // From this point onwards, the process is similar to the 'create' controller,
    // such as checking if the directory exists, creating directories or files as needed,
    // and writing the contents to the file.
    // be sure to update the path on the other CRUD write methods too or they will all write outside the folder.
    let path = format!("./src/controllers/{}/{}.rs", model_name, model_name);
    let folder_path = format!("./src/controllers/{}", model_name);

    if !PathBuf::from(&folder_path).exists() {
        println!(
            "The folder {} does not exist. Would you like to create it? (y/n)",
            folder_path
        );
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "y" {
            fs::create_dir(PathBuf::from(&folder_path)).unwrap();
        } else {
            // If the user does not want to create the directory, you can return an error or handle it in another way suitable to your application
            return Err(Error::new(
                ErrorKind::Other,
                "Controller folder creation aborted by user",
            ));
        }
    }


    // read the contents of the file so we don't overwrite it
    let mut file_contents = fs::read_to_string(&path).unwrap();

    // add two new lines to the end of the file
    file_contents.push_str("\n\n");

    // add the contents to the file
    file_contents.push_str(&contents);
    println!("The path is: {}", &path);
    match fs::write(PathBuf::from(&path), file_contents.as_bytes()) {
        Ok(()) => {
            // Update the main.rs and authenticated_page as needed, similar to the create controller
            add_new_controller_to_main_rs(Some(&model_name), &format!("update_{}", &model_name)).unwrap();


            println!("Successfully written to {}.rs", model_name);
            Ok(())
        }
        Err(e) => Err(e),
    }
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
/// write_to_new_get_controller_with_authorized_view("user".to_string(), "authenticated_page".to_string());
/// ```
pub fn write_to_new_get_controller_with_authorized_view(
    controller_name: String,
    folder_name: String,
) -> Result<(), Error> {
    // Define the contents to be written to the file
    // This includes importing necessary Actix Web and Tera modules, defining the controller handler function,
    // and setting up the Tera template rendering
    let contents = format!(
        r#"use actix_identity::Identity;
                use actix_web::{{get, web, HttpResponse, Responder}};
                use tera::{{Context, Tera}};

                #[get("/{}")]
                async fn {}(
                    tmpl: web::Data<Tera>,
                    user: Option<Identity>
                ) -> impl Responder {{
                    if let Some(user) = user
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
        controller_name,
        controller_name,
        controller_name,
        controller_name,
        folder_name,
        controller_name
    );

    // Define the path to the file
    let path = format!(
        "./src/controllers/{}/{}.rs",
        controller_name, controller_name
    );

    // Write the contents to the file
    // The write_to_file function is assumed to be a function that takes a path and a byte slice and writes the bytes to the file at the path
    // If the file doesn't exist, the function will create it, and if it does exist, the function will overwrite it
    match fs::write(PathBuf::from(&path), contents.as_bytes()) {
        Ok(()) => println!("Successfully written to {}.rs", controller_name),
        Err(e) => println!("Failed to write to {}.rs: {:?}", controller_name, e),
    }

    // Return Ok if everything succeeded
    Ok(())
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
pub fn write_to_previous_get_controller(previous_controller_name: String, new_controller_name: String ) -> Result<(), Error> {
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
    println!("file_contents: {}", file_contents);
    // and then append the new contents to the file
    file_contents.push_str(&contents);

    match fs::write(PathBuf::from(&path), file_contents.as_bytes()) {
        Ok(()) => println!("Successfully written to {}.rs", previous_controller_name),
        Err(e) => println!(
            "Failed to write to {}.rs: {:?}",
            previous_controller_name, e
        ),
    }

    // Return Ok if everything succeeded
    Ok(())
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
pub fn write_to_previous_create_controller(
    previous_controller_name: String,
    new_controller_name: String,
) -> Result<(), Error> {
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
            if let Some(user) = user {{
                let result = {}::create_{}({}.into_inner()).await;
                match result {{
                    Ok(page) => HttpResponse::Ok().json(page),
                    Err(e) => HttpResponse::BadRequest().json(e.to_string()),
                }}
            }} else {{
              // redirect to login page
              let mut context = tera::Context::new();
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
    let path = format!(
        "./src/controllers/{}/{}.rs",
        previous_controller_name, previous_controller_name
    );

    // instead of overwriting the file, we need to append to the file
    // lets get the contents of the file first
    let mut file_contents = fs::read_to_string(&path).unwrap();

    // and then append the new contents to the file
    file_contents.push_str(&contents);

    match fs::write(PathBuf::from(&path), file_contents.as_bytes()) {
        Ok(()) => println!("Successfully written to {}.rs", previous_controller_name),
        Err(e) => println!(
            "Failed to write to {}.rs: {:?}",
            previous_controller_name, e
        ),
    }

    // Return Ok if everything succeeded
    Ok(())
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
pub fn write_to_initial_post_controller_authentication(
    controller_name: String,
) -> Result<(), Error> {
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
/// write_to_initial_get_controller_authorized_view("user".to_string(), "authenticated_page".to_string());
/// ```
pub fn write_to_initial_get_controller_authorized_view(
    controller_name: String,
    folder_name: String,
) -> Result<(), Error> {
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
use models::user::UserLogin;

#[get("/{}")]
pub async fn {}_controller_with_authorized_view(
    tmpl: web::Data<Tera>,
    user: Option<Identity>
) -> impl Responder {{
  if let Some(user) = user {{
        let mut context = Context::new();
        context.insert("username", &user.id().unwrap());
        context.insert("title", "{}");
        context.insert("controller_name", "{}");
        let rendered = tmpl.render("layouts/authenticated_page/{}/{}.html.tera", &context).unwrap();
        HttpResponse::Ok().body(rendered)
}}
    }} else {{
        let mut context = Context::new();
        context.insert("title", "Login");
        context.insert("route_name", "login");
        context.insert("error", "You must be logged in to view this page.");
        Ok(HttpResponse::Found()
            .append_header((actix_web::http::header::LOCATION, "/login"))
            .finish())
    }}
}}"#,
        controller_file_name.trim_end_matches(".rs"),
        controller_file_name.trim_end_matches(".rs"),
        controller_file_name.trim_end_matches(".rs"),
        controller_file_name.trim_end_matches(".rs"),
        folder_name,
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
/// write_to_new_get_all_controller("user".to_string());
/// ```
pub fn write_to_new_get_all_controller(model_name: String) -> Result<(), Error> {

    // look for the model in the models folder
    let model_path = format!("./src/models/{}.rs", model_name);
    let model_file_path = std::path::Path::new(&model_path);
    if !model_file_path.exists() {
        println!("The model {} does not exist. Would you like to create it? (y/n)", model_name);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if input.trim() == "y" {
          println!("Do it yourself for now");
        } else {
            return Err(Error::new(
                ErrorKind::Other,
                "Model creation aborted by user",
            ));
        }
    }

    let capitalize_model_name = crate::helpers::helpers::capitalize_first(&model_name);


    let controller_contents = format!(
        r#"
        use actix_web::{{get, web, HttpResponse}};
        use crate::models::{};

        #[get("/{}/all")]
        pub async fn get_all_{}s() -> HttpResponse {{
            let result = {}::get_all_{}().await;
            match result {{
                Ok({}) => HttpResponse::Ok().json({}),
                Err(e) => HttpResponse::BadRequest().json(e.to_string()),
            }}
        }}"#,
        &model_name,
        &model_name,
        &model_name,
        &capitalize_model_name,
        &model_name,
        &capitalize_model_name,
        &capitalize_model_name
    );

    // search the controllers folder for the model name
    let path = if PathBuf::from(format!("./src/controllers/{}.rs", &model_name)).exists() {
        format!("./src/controllers/{}.rs", &model_name)
    } else {
        format!("./src/controllers/{}/{}.rs", &model_name, &model_name)
    };

    // we know one of the paths will exist
    // ensure path exists
    let result = PathBuf::from(&path).exists();

    if !result {
        println!("The controller {} does not exist. Would you like to create it? (y/n)", &model_name);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if input.trim() == "y" {
            fs::create_dir(PathBuf::from(&path)).unwrap();
        } else {
            return Err(Error::new(
                ErrorKind::Other,
                "Controller creation aborted by user",
            ));
        }
    }

    // read the contents of the file so we don't overwrite it
    let mut file_contents = fs::read_to_string(&path).unwrap();

    // add two new lines to the end of the file
    file_contents.push_str("\n\n");

    // add the contents to the file
    file_contents.push_str(&controller_contents);

    match fs::write(PathBuf::from(&path), file_contents.as_bytes()) {
        Ok(()) => {
            add_new_controller_to_main_rs(Some(&model_name), &format!("get_all_{}", &model_name)).unwrap();
            println!("Successfully written to {}.rs", model_name);
            Ok(())
        }
        Err(e) => Err(e),
    }
}