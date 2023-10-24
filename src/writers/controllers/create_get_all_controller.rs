use std::fmt::format;
use crate::generators::create_file;
use crate::writers;
use crate::writers::{add_new_controller_to_main_rs, write_to_controller_name_html, write_to_controllers_mod, write_to_new_get_all_controller, write_to_new_post_controller, write_to_new_update_controller};
use eyre::Error;
use std::fs;
use std::fs::create_dir;
use std::path::Path;


pub fn create_get_all_controller(model_name: String) -> Result<(), Error> {
    // find the model in the models directory
    // if the model does not exist, let the user know and tell them to create a model with that name
    // we will later  use the migration to create the model
    let model_path = format!("./src/models/{}.rs", model_name);
    let model_path = Path::new(&model_path);
    if !model_path.exists() {
        println!("There is no model with that name. Please create a model with that name.");
        // end the function
        return Ok(());
    }
    // find the folder in the controllers directory with the name of the controller
    // first check if there is a folder with the name of the model
    // if not check if there is a file in the controllers directory with the name of the model
    let controller_with_folder_dir = format!("./src/controllers/{}/{}.rs", model_name, model_name);
    let mut has_controller = false;
    let mut is_file = false;
    let controller_path = Path::new(&controller_with_folder_dir);

    if controller_path.exists() {
        has_controller = true;
    }

    let current_dir = format!("./src/controllers/{}.rs", model_name);
    let controller_path = Path::new(&current_dir);
    if controller_path.exists() {
        has_controller = true;
        is_file = true;
    }

    if !has_controller {
        //create a new controller file
        create_file(&format!("./src/controllers/{}.rs", model_name)).unwrap_or_else(|why| {
            println!("Failed to create file: {:?}", why.to_string());
        });
    }

    if !is_file {
        // add the controller to the file for that controller
        let file_path = format!("./src/controllers/{}/{}.rs", model_name, model_name);

        // add the controller to the file
        write_to_new_get_all_controller(model_name.clone().to_string(), file_path).unwrap_or_else(|why| {
            println!("Failed to write to controller: {:?}", why.to_string());
        });
    } else {
        // add the controller to the file for that controller
        let file_path = format!("./src/controllers/{}.rs", model_name);

        // add the controller to the file
        write_to_new_get_all_controller(model_name.clone().to_string(), file_path).unwrap_or_else(|why| {
            println!("Failed to write to controller: {:?}", why.to_string());
        });
    }

    Ok(())

}