use crate::writers::{route_writer, write_to_file, write_to_routes_mod};
use crate::Project;
use std::io::Error;

pub fn write_to_dashboard(project: Project) -> Result<(), Error> {
    let contents = r#"{% extends 'base' %}
{% block title %}Index{% endblock title %}
{% block head %}
{{ super() }}
{% endblock head %}
{% block content %}
<div class='relative px-6 lg:px-8'>
<div class='mx-auto  max-w-2xl py-32 sm:py-48 lg:py-56' >
<h1 class='text-4xl sm:text-5xl lg:text-6xl font-extrabold leading-none mb-4'>Your Route's Name: {{route_name}}</h1>
<p class='text-xl sm:text-2xl lg:text-3xl font-medium mb-8'>This is a rustyroad project</p>
</div>
</div>
       {% endblock content %}"#.to_string();

    write_to_file(&project.dashboard_page_html, contents.as_bytes()).unwrap_or_else(|why| {
        panic!(
            "Couldn't write to {}: {}",
            &project.dashboard_page_html, why
        )
    });

    route_writer::write_to_initial_route_rs(project.dashboard_route.clone()).unwrap_or_else(
        |why| {
            panic!(
                "Couldn't write to the: {}: {}",
                &project.dashboard_route, why
            )
        },
    );

    write_to_routes_mod(&project.routes_module, "dashboard".to_string()).unwrap_or_else(|why| {
        panic!(
            "Couldn't write to the: {}: {}",
            &project.dashboard_route, why
        )
    });
    Ok(())
}
