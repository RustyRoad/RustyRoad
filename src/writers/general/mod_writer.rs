use crate::writers::write_to_file;
use color_eyre::eyre::Result;

/// # Name: write_to_module
/// This function writes a new Rust module to a Rust source file.
/// # Arguments:
/// * module_path: &String
/// * module: Vec<String>
/// # Returns:
/// * Result<(), std::io::Error>
/// # Example:
/// ```
/// use std::vec;
/// use rustyroad::writers::write_to_module;
/// use crate::writers::general;
/// use std::io::Error;
///
///
///let name = "test";
///let mut components = Vec::new();
///
/// components.push(name.to_string());
///
/// let module_path = "src/test.rs".to_string();
///
///
/// write_to_module(&module_path, components);
/// ```
pub fn write_to_module(module_path: &String, module: Vec<String>) -> Result<(), std::io::Error> {
    let mut template = String::new();

    for component in &module {
        template.push_str(&format!("pub mod {};", component));
    }

    // Add an empty line
    template.push_str("\n");

    // use the modules after they are created

    for component in &module {
        template.push_str(&format!("pub use {}::*;", component));
    }

    // write the template to the file
    write_to_file(&module_path, template.as_bytes())
}
