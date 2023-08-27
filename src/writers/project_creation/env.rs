use color_eyre::eyre::Result;
use eyre::Error;
use rand::{distributions::Alphanumeric, Rng};
use crate::Project;
use crate::writers::write_to_file;
pub fn set_env(project: Project) -> Result<String, Error> {
     // generate a random 32 bit string that is 50 characters long
    let random_string: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(50)
        .map(char::from)
        .collect();

    // create an env in root directory
    let contents = format!(
        "SECRET_KEY={};"
        random_string
    );

    let mut result = "success".to_string();


    write_to_file(&project.env, contents.as_bytes()).unwrap_or_else(|why| {
       result = "failed".to_string();
    });

    Ok(result)
}