use crate::writers::general;

/// Name: write_to_controllers_mod
/// Description: Writes to the controllers/authenticated_page file
/// Calls the write_to_module function from the general module writer
pub fn write_to_controllers_mod(
    file_name: &String,
    controller_name: String,
) -> Result<(), std::io::Error> {
    let mut components = Vec::new();
    // the function  that is generating the index file also needs to call this function.
    // right now this is being called and run automatically.
    components.push(controller_name);
    //components.push("about".to_string());
    //components.push("contact".to_string());
    general::write_to_module(file_name, components)
}

/// Name: write_to_models_mod
/// Description: Writes to the models/authenticated_page file
/// Calls the write_to_module function from the general module writer
pub fn write_to_models_mod(file_name: &String, model_name: String) -> Result<(), std::io::Error> {
    let mut components = Vec::new();
    // the function  that is generating the index file also needs to call this function.
    // right now this is being called and run automatically.
    components.push(model_name);
    //components.push("about".to_string());
    //components.push("contact".to_string());
    general::write_to_module(file_name, components)
}
