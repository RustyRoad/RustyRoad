use crate::Project;
use std::fs::create_dir;
use std::io::Error;
use std::path::Path;

/// # Name: create_directory
/// # Description: Creates a directory
/// # Arguments:
/// * `name` - The name of the directory
pub fn create_directory(project: &Project) -> Result<(), Error> {
    let directories = vec![
        &project.name,
        &project.src_dir,
        &project.config,
        &project.config_env,
        &project.db,
        &project.routes,
        &project.controllers,
        &project.models,
        &project.migrations,
        &project.seeders,
        &project.tests,
        &project.config_initializers,
        &project.templates,
        &project.static_dir,
        &project.template_components,
        &project.template_layouts,
        &project.template_pages,
        &project.static_css,
        &project.static_js,
        &project.static_images,
        &project.user_controller_directory,
        &project.user_model_directory,
        &project.user_migration_directory,
    ];
    for directory in directories {
        create_dir(directory).unwrap_or_else(|why| {
            println!("!{:?} {:?}", &directory, why.kind());
        });
    }
    Ok(())
}
