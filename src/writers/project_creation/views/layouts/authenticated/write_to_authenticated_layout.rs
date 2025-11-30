use crate::writers::write_to_file;
use crate::Project;
use color_eyre::eyre::Result;
use std::io::Error;

pub fn write_to_authenticated_layout(project: Project) -> Result<(), Error> {
    let contents = r#"
    {% extends 'layouts/layout.html.tera' %}
    {% block title %}{{title | default(value="Dashboard", boolean=true)}}{% endblock title %}
    
    
    {% block main_content %}
    
    <nav id="side-bar" class="z-">
        {% include 'components/sidebar.html.tera'%}
    </nav>
    {% block authenticated_content %}
    {% endblock authenticated_content %}
    <!-- Custom main content for this page -->
    {% endblock main_content %}
    "#
    .to_string();

    write_to_file(&project.authenticated_layout, contents.as_bytes()).unwrap_or_else(|why| {
        panic!(
            "Couldn't write to {}: {}",
            &project.authenticated_layout, why
        )
    });

    Ok(())
}
