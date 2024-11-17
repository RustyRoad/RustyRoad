use crate::writers::write_to_file;

// write to base html tera template
pub fn write_to_base_html(name: &String) -> Result<(), std::io::Error> {
    let contents = r#"<!DOCTYPE html>
<html class='bg-gray-50 h-full' lang='en'>

    <head>
        {% block head %}
        {% include 'sections/header.html.tera' %}
        {% endblock head %}
    </head>

    <body id='app' class='h-full flex flex-col'>
        {% include 'components/navbar.html.tera'%}
        <div class="h-full" id='content'>{% block content %}{% endblock content %}</div>
        <div id='footer'>
            {% block footer %}
            {% include 'sections/footer' ignore missing %}
            {% endblock footer %}
        </div>
    </body>
    <script src='/static/js/index.js'></script>

</html>"#;
    // write to file and use the result
    write_to_file(name, contents.as_bytes())
        .unwrap_or_else(|why| panic!("Couldn't write to {}: {}", &name, why));

    Ok(())
}
