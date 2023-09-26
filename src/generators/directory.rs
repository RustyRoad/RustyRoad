use crate::Project;
use std::fs::create_dir;
use std::io::Error;

/// # Name: create_directories_for_new_project
/// ## Description
/// Creates the directories for a new project
/// ## Arguments
/// * `project` - A reference to a Project struct
/// ## Returns
/// * `Ok(())` if the directories were successfully created, or an Error if something went wrong.
/// ## Example
/// ```
/// use rustyroad::Project;
///
/// let project = Project::new();
/// rustyroad::generators::create_directories_for_new_project(&project);
/// ```
pub fn create_directories_for_new_project(project: &Project) -> Result<(), Error> {
    let directories = vec![
        &project.name,
        &project.src_dir,
        &project.config,
        &project.config_env,
        &project.db,
        &project.controllers,
        &project.controllers,
        &project.models,
        &project.migrations,
        &project.seeders,
        &project.tests,
        &project.config_initializers,
        &project.templates,
        &project.static_dir,
        &project.template_components,
        &project.template_sections,
        &project.template_layouts,
        &project.template_pages,
        &project.static_css,
        &project.static_js,
        &project.static_images,
        &project.user_controller_directory,
        &project.initial_migration_directory,
        &project.template_layouts,
        &project.auth_template_layouts,
    ];
    for directory in directories {
        create_dir(directory).unwrap_or_else(|why| {
            println!("!{:?} {:?}", &directory, why.kind());
        });
    }
    Ok(())
}
