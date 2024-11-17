
use eyre::Error;
use crate::{CRUDType};
use crate::writers::{create_create_controller_in_existing_folder, create_create_controller_in_new_folder, create_read_controller_in_existing_folder, create_read_controller_in_new_folder, create_update_controller_in_existing_folder, create_update_model};
/// # Name: create_new_controller
/// ### Description:
/// - Creates a new controller
/// ### Parameters:
/// - model_name: String
/// - controller_type: CRUDType
/// ### Returns:
/// - Result<(), Error>
/// ### Example:
/// ```
/// use sqlparser::ast::Statement::Assert;
/// use rustyroad::writers::create_new_controller;
/// use rustyroad::CRUDType;
///
/// let model_name = "page".to_string();
/// let controller_type = CRUDType::Read;
///
/// let result = create_new_controller(model_name, controller_type);
/// ```
pub async fn create_new_controller(model_name: String, controller_type: CRUDType) -> Result<(), Error> {
    // the controller will need to check the current directory to see if it is a rustyroad project
    // if it is not, it will return an error and ask the user to run the command in a rustyroad project
    // if it is a rustyroad project, it will create a new directory with the controllerName
    // it will create a new file with the controllerName.rs
    // ask the user if a controller folder already exists
    // if it does, ask the user if they want to add this controller to the file for that controller
    // if it does not, continue with the rest of the code
    println!("Does a controller folder already exist (y/n): ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();



    // if the user enters y, ask the user if they want to add this controller to the file for that controller
    if input == "y" {
        match controller_type {
            CRUDType::Read => {
                // create the controller
                create_read_controller_in_existing_folder(model_name.clone()).unwrap_or_else(
                    |why| {
                        println!("Failed to create read controller: {:}", why.to_string());
                    },
                );
                Ok(())
            }
            CRUDType::Create => {
                // create the controller
               create_create_controller_in_existing_folder(model_name.clone()).unwrap_or_else(
                    |why| {
                        println!("Failed to create create controller: {:}", why.to_string());
                    },
                );
                Ok(())
            }
            CRUDType::Update => {
                // create the controller
                create_update_controller_in_existing_folder(&model_name).unwrap_or_else(
                    |why| {
                        println!("Failed to create update controller: {:}", why.to_string());
                    },
                );

                // Create the update model
                create_update_model(&model_name).await.expect("Failed to create update model");
                Ok(())
            }
            CRUDType::Delete => {
                // create the controller
                todo!("Delete controller")
            }
        }
    } else {
        match controller_type {
            // if the user enters n, continue with the rest of the code and create a new controller that will be added to the controllers/authenticated_page file
            CRUDType::Read => {
               create_read_controller_in_new_folder(model_name.clone()).unwrap_or_else(
                    |why| {
                        println!("Failed to create read controller: {:}", why.to_string());
                    },
                );
                Ok(())
            }
            CRUDType::Create => {
                // create the controller
                create_create_controller_in_new_folder(model_name.clone()).unwrap_or_else(
                    |why| {
                        println!("Failed to create create controller: {:}", why.to_string());
                    },
                );
                Ok(())
            }
            CRUDType::Update => {
                // create the controller
                create_update_controller_in_existing_folder(&model_name).unwrap_or_else(
                    |why| {
                        println!("Failed to create update controller: {:}", why.to_string());
                    },
                );
                Ok(())
            }
            CRUDType::Delete => {
                // create the controller
                todo!("Delete controller")
            }
        }
    }
}