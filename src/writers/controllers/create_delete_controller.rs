use std::fs;
use std::fs::create_dir;
use crate::generators::create_file;
use crate::writers::{add_new_controller_to_existing_module_in_main_rs, add_new_controller_to_main_rs, write_to_controller_name_html, write_to_controllers_mod, write_to_new_get_controller, write_to_new_delete_controller, write_to_previous_create_controller};
use eyre::Error;
use crate::writers;

/// # Name: create_create_controller_in_existing_folder
/// This function creates a new create controller in an existing folder.
/// # Arguments:
/// * controller_name: String
/// # Returns:
/// * Result<(), Error>
/// # Example:
///
/// ```
/// use rustyroad::writers::create_create_controller_in_existing_folder;
/// use rustyroad::CRUDType;
/// use eyre::Error;
///
/// let controller_name = "test".to_string();
///
/// create_create_controller_in_existing_folder(controller_name);
/// ```
pub fn create_delete_controller_in_existing_folder(controller_name: String) -> Result<(), Error> {
    println!("Do you want to add this controller to the file for that controller? (y/n): ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    // if the user enters y, add the controller to the file for that controller
    if input == "y" {
        // ask the user the name of the controller
        println!("What is the name of the model you want to create a controller for?: ");
        println!("In order to work out of the box, ensure the model already exists.");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        // find the folder in the controllers directory with the name of the controller
        // if the folder does not exist, let the user know and ask them if they want to create
        // a new controller with that name
        // if the folder does exist, add the controller to the file for that controller
        let current_dir = fs::read_dir("./src/controllers").unwrap();
        let mut has_controller = false;

        for entry in current_dir {
            let entry = entry.unwrap();
            let file_name = entry.file_name();
            let file_name = file_name.to_str().unwrap();
            if file_name == input {
                has_controller = true;
            }
        }

        if !has_controller {
            println!("There is no controller with that name. Do you want to create a new controller with that name? (y/n): ");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();

            // if the user enters y, create a new controller with that name
            if input == "y" {
                // create the controller
                // add the controller to the file for that controller
                write_to_new_delete_controller(controller_name.clone()).unwrap_or_else(|why| {
                    println!("Failed to write to controller: {:?}", why.to_string());
                });
                // end the function
                return Ok(());
            } else {
                println!("Please run the command again and enter a valid controller name.");
                // end the function
                std::process::exit(0);
            }
        } else {
            // if the folder does exist, add the controller to the file for that controller
            write_to_previous_create_controller(input.to_string(), controller_name.clone())
                .unwrap_or_else(|why| {
                    println!("Failed to write to controller: {:?}", why.to_string());
                });
            // Create a new file with the controllerName.html.tera
            create_file(&format!("./views/pages/{}.html.tera", controller_name)).unwrap_or_else(
                |why| {
                    println!("Failed to create file: {:?}", why.to_string());
                },
            );
            // Write to controllerName.html.tera file
            write_to_controller_name_html(controller_name.clone().as_str()).expect("Failed to write to controllerName.html.tera");
            // update main.rs file
            add_new_controller_to_existing_module_in_main_rs(
                input,
                controller_name.clone().as_str(),
            )
                .unwrap_or_else(|why| {
                    println!("Failed to add to controller in main.rs: {:?}", why.kind());
                });
            // end the function
            return Ok(());
        }
    } else {
        // if the user enters n, continue with the rest of the code
        // add the controller to the file for that controller
        write_to_new_get_controller(controller_name.clone()).unwrap_or_else(|why| {
            println!("Failed to write to controller: {:?}", why.to_string());
        });
        // end the function
        return Ok(());
    }
}

/// # Name: create_create_controller_in_new_folder
/// This function creates a new create controller in a new folder.
/// # Arguments:
/// * controller_name: String
/// # Returns:
/// * Result<(), Error>
/// # Example:
///
/// ```
/// use rustyroad::writers::create_create_controller_in_new_folder;
/// use eyre::Error;
///
/// let controller_name = "test".to_string();
///
/// create_create_controller_in_new_folder(controller_name);
/// ```
pub fn create_delete_controller_in_new_folder(controller_name: String) -> Result<(), Error> {
    // create the controller
    // check if the current directory is a rustyroad project
    let current_dir = fs::read_dir(".").unwrap();
    let mut has_rustyroad_toml = false;

    // check if the current directory has a rustyroad.toml file
    for entry in current_dir {
        let entry = entry.unwrap();
        let file_name = entry.file_name();
        let file_name = file_name.to_str().unwrap();
        if file_name == "rustyroad.toml" {
            has_rustyroad_toml = true;
        }
    }
    // rustyroad.toml file will be used to store the project name and other project information
    // if the current directory does not have a rustyroad.toml file, it will return an error
    if !has_rustyroad_toml {
        println!(
            "This is not a rustyroad project. Please run this command in a rustyroad project."
        );
        // end the function
        return Ok(());
    }

    // Create a new directory with the controllerName
    create_dir(format!("./src/controllers/{}", &controller_name)).unwrap_or_else(
        |why| {
            println!("Failed to create directory: {:?}", why.to_string());
        },
    );
    // Create a new controller using the controllerName
    // Update the controllers/page file
    let full_file_name = format!("./src/controllers/page");
    write_to_controllers_mod(&full_file_name, controller_name.clone())
        .unwrap_or_else(|why| {
            println!("Failed to write to controllers/mod: {:?}", why.to_string());
        });

    // create the controllers/page file
    create_file(&format!("./src/controllers/{}/page", controller_name))
        .unwrap_or_else(|why| {
            println!("Failed to create file: {:?}", why.to_string());
        });

    let mut components = Vec::new();
    // Create a vector and push the controllerName to the vector
    components.push(controller_name.clone().to_string());

    // Write to page file
    writers::write_to_module(
        &format!("./src/controllers/{}/page", &controller_name),
        components,
    )
        .unwrap_or_else(|why| {
            println!("Failed to write to page: {:?}", why.to_string());
        });

    // Create a new file with the controllerName.rs
    create_file(&format!(
        "./src/controllers/{}/{}.rs",
        controller_name, controller_name
    ))
        .unwrap_or_else(|why| {
            println!("Failed to create file: {:?}", why.to_string());
        });
    // Write to controllerName.rs file
    write_to_new_delete_controller(controller_name.clone()).unwrap_or_else(|why| {
        println!(
            "Failed to write to controllerName.rs: {:?}",
            why.to_string()
        );
    });

    // Create a new file with the controllerName.html.tera
    create_file(&format!("./views/pages/{}.html.tera", controller_name))
        .unwrap_or_else(|why| {
            println!("Failed to create file: {:?}", why.to_string());
        });
    // Write to controllerName.html.tera file
    write_to_controller_name_html(controller_name.clone().as_str()).expect("Failed to write to controllerName.html.tera");

    // update main.rs file
    add_new_controller_to_main_rs(None,None, controller_name.clone().as_str())
        .expect("Failed to add to controller in main.rs");

    // end the function
    return Ok(());
}