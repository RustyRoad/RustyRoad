use crate::writers::{controller_writer, write_to_controllers_mod, write_to_file};
use color_eyre::eyre::Result;
use crate::Project;
use std::io::Error;

pub fn write_to_authenticated_layout(project: Project) -> Result<(), Error> {
    let contents = r#"{% extends 'layouts/authenticated/authenticated.html.tera' %}
{% block title %}{{title | default(value="Dashboard", boolean=true)}}{% endblock title %}

{% block authenticated_content %}
<!-- Custom main content for this page -->
{% endblock authenticated_content %}


{% endblock main_content %}"#.to_string();

    write_to_file(&project.authenticated_layout, contents.as_bytes()).unwrap_or_else(|why| {
        panic!(
            "Couldn't write to {}: {}",
            &project.authenticated_layout, why
        )
    });

    Ok(())
}