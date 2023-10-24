use eyre::Error;

use crate::{Project, writers::write_to_file};

pub fn write_to_layout(project: Project) -> Result<(), Error> {
    let contents = r#"{% extends 'base.html.tera' %}
{% block title %}{{title | default(value="Dashboard", boolean=true)}}{% endblock title %}
{% block head %}
{{ super() }}
{% endblock head %}
{% block content %}


<!--
    This example requires updating your template:

    ```
    <html class="h-full bg-white">
    <body class="h-full">
    ```
  -->
{% block sidebar %}
<!-- Sidebar content here -->
{% endblock sidebar %}


<main class="lg:pl-72">
  <div id='content'>
    {% block main_content %}
    <!-- Main content here -->
    {% endblock main_content %}
  </div>
</main>
{% endblock content %}"#.to_string();

    write_to_file(&project.layout_template, contents.as_bytes()).unwrap_or_else(|why| {
        panic!(
            "Couldn't write to {}: {}",
            &project.layout_template, why
        )
    });

    Ok(())
}