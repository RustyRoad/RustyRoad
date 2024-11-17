use std::fs::File;
use std::io::Error;

/// # Name: create_file
/// # Description: Creates a file
/// # Arguments:
/// * `name` - The name of the file
pub fn create_file(name: &str) -> Result<(), Error> {
    let result = match File::create(name) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    };
    match result {
        Ok(_) => Ok(()),
        Err(e) => {
            println!("Error creating file: {:?}", e);
            Err(e)
        }
    }
}
