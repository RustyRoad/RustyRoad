use std::io::Error;

use crate::writers::write_to_file;

pub fn write_to_route_name_html(route_name: String) -> Result<(), Error> {
    let contents = format!(
        r#"{{% extends 'base' %}}
{{% block title %}}Index{{% endblock title %}}
{{% block head %}}
{{{{ super() }}}}
{{% endblock head %}}
{{% block content %}}
<div class='relative px-6 lg:px-8'>
<div class='mx-auto  max-w-2xl py-32 sm:py-48 lg:py-56' >
<h1 class='text-4xl sm:text-5xl lg:text-6xl font-extrabold leading-none mb-4'>Your Route's Name: {{{{route_name}}}}</h1>
<p class='text-xl sm:text-2xl lg:text-3xl font-medium mb-8'>This is a rustyroad project</p>
</div>
</div>
{{% endblock content %}}"#
    );

    // write to the file
    write_to_file(
        &format!("./templates/pages/{}.html.tera", route_name).to_string(),
        contents.as_bytes(),
    )
    .unwrap_or_else(|why| {
        println!(
            "Couldn't write to {}: {}",
            &format!("./templates/pages/{}.html.tera", route_name).to_string(),
            why.to_string()
        );
    });
    Ok(())
}

pub fn write_to_route_name_rs(route_name: String) -> Result<(), Error> {
    let contents = format!(
        r#"use rocket::fs::{{relative, FileServer}};
use rocket_dyn_templates::{{context, Template}};

#[get("/")]
pub fn index() -> Template {{
    Template::render(
        "pages/{}",
        context! {{
            route_name: {},
        }},
    )
}}"#,
        route_name, route_name.to_ascii_uppercase()
    );

    write_to_file(
        &format!("./src/routes/{}/{}.rs", route_name, route_name),
        contents.as_bytes(),
    )
    .unwrap_or_else(|why| {
        println!("Failed to write to routeName.rs: {:?}", why.kind());
    });
    Ok(())
}
