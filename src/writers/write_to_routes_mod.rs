use crate::writers::general;

/// Name: write_to_routes_mod
/// Description: Writes to the routes/mod.rs file
/// Calls the write_to_module function from the general module writer
pub fn write_to_routes_mod(file_name: &String, route_name: String) -> Result<(), std::io::Error> {
    let mut components = Vec::new();
    // the function  that is generating the index file also needs to call this function.
    // right now this is being called and run automatically.
    components.push(route_name);
    //components.push("about".to_string());
    //components.push("contact".to_string());
    general::write_to_module(file_name, components)
}


/// Name: write_to_models_mod
/// Description: Writes to the models/mod.rs file
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