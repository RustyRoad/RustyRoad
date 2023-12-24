use crate::writers::{ write_to_controllers_mod, write_to_dashboard_controller, write_to_file};
use crate::Project;
use std::io::Error;

pub fn write_to_dashboard(project: Project) -> Result<(), Error> {
    let contents = r#"{% extends 'layouts/authenticated_page/authenticated_page.html.tera' %}
{% block title %}{{title | default(value="Dashboard", boolean=true)}}{% endblock title %}

{% block authenticated_content %}
<div class='relative px-6 lg:px-8'>
       <div class='mx-auto  max-w-2xl py-32 sm:py-48 lg:py-56'>
                <h1 class='text-4xl sm:text-5xl lg:text-6xl font-extrabold leading-none mb-4'>Hello, {{ username }}!</h1>

              <p class='text-xl sm:text-2xl lg:text-3xl font-medium mb-8'>You're Logged in!</p>
       </div>
</div>
{% endblock authenticated_content %}"#.to_string();

    write_to_file(&project.dashboard_page_html, contents.as_bytes()).unwrap_or_else(|why| {
        panic!(
            "Couldn't write to {}: {}",
            &project.dashboard_page_html, why
        )
    });

    // write to dashboard controller
    write_to_dashboard_controller(&project).unwrap_or_else(|why| {
        panic!(
            "Couldn't write to the: {}: {}",
            &project.dashboard_controller, why
        )
    });

    // print the current directory
    println!("Current directory: {:?}", std::env::current_dir().unwrap());

    // ensure the current directory is the project directory
    // if it is not, change the current directory to the project directory
    if std::env::current_dir().unwrap().to_str().unwrap() != project.name {
        let path_to_set = std::env::current_dir().unwrap().join(&project.name);
        std::env::set_current_dir(path_to_set).unwrap();
    }

    // print the current directory
    println!("Current directory: {:?}", std::env::current_dir().unwrap());

    // define the new controllers mod with the current directory
    let controllers_mod = format!("{}/src/controllers/mod.rs", std::env::current_dir().unwrap().to_str().unwrap());

    write_to_controllers_mod(&controllers_mod, "dashboard".to_string()).unwrap_or_else(
        |why| {
            panic!(
                "Couldn't write to the: {}: {}",
                &project.dashboard_controller, why
            )
        },
    );
    Ok(())
}
