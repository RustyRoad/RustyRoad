use crate::writers::{controller_writer, write_to_controllers_mod, write_to_file};
use color_eyre::eyre::Result;
use crate::Project;
use std::io::Error;

pub fn write_to_authenticated_layout(project: Project) -> Result<(), Error> {
    let contents = r#"{% extends 'layouts/layout.html.tera' %}
    {% block title %}{{title | default(value="Dashboard", boolean=true)}}{% endblock title %}
    
    
    {% block main_content %}
    
    <nav id="side-bar">
        {% include 'components/sidebar.html.tera'%}
    </nav>
    {% block authenticated_content %}
    {% endblock authenticated_content %}
    <script>
    
      document.addEventListener("DOMContentLoaded", function() {
        console.log("DOM fully loaded and parsed");
        
        console.log("Current Pathname is: ", window.location.pathname); // Debugging line
        
        const pathnameParts = window.location.pathname.split('/');
        const isPagePath = window.location.pathname === '/page' || (window.location.pathname.startsWith('/page/') && window.location.pathname.endsWith('/edit'));
    
        if (isPagePath) {
            const sidebar = document.getElementById('side-bar');
            
            if (sidebar) {
                console.log("Sidebar found. Hiding it now."); // Debugging line
                sidebar.style.display = 'none';
            } else {
                console.log("Sidebar element with ID 'side-bar' not found"); // Debugging line
            }
        } else {
            console.log("This is not the /page URL. Sidebar will not be hidden."); // Debugging line
        }
    });
        
    </script>
    <!-- Custom main content for this page -->
    {% endblock main_content %}"#.to_string();

    write_to_file(&project.authenticated_layout, contents.as_bytes()).unwrap_or_else(|why| {
        panic!(
            "Couldn't write to {}: {}",
            &project.authenticated_layout, why
        )
    });

    Ok(())
}