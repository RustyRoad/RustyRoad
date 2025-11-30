use crate::writers::write_to_file;
use eyre::Error;

/// # Name: create_grapesjs_component
/// ## Description:
/// - Creates the grapesjs.html.tera file in the src/views/components folder.
/// - This is needed for grapesjs to work.
/// ## Arguments: None
/// ## Returns:
/// - Result<(), Error>
/// ## Example:
/// ```
/// use rustyroad::writers::templates::components::features::grapesjs_components::create_grapesjs_component;
///
/// create_grapesjs_component();
/// ```
pub fn create_grapesjs_component() -> Result<(), Error> {
    // define the html template
    let html = r#"
   <div id="gjs" style="height: 100%; overflow: hidden; width: 100%;">
    <div style="margin:100px 100px 25px; padding:25px; font:caption">
        {% if html_content %}
        {{html_content|safe}}
        {% else %}
        Welcome to the editor grab a block from the right and drag it here
        {% endif %}
    </div>
</div>

<style>
    body,
    html {
        height: 80%;
        margin: 0;
    }

    .gjs-block {
        padding: 0 !important;
        width: 100% !important;
        min-height: auto !important;
    }

    .gjs-block svg {
        width: 100%;
    }

    .change-theme-button {
        width: 40px;
        height: 40px;
        border-radius: 50%;
        margin: 5px;
    }

    .change-theme-button:focus {
        /* background-color: yellow; */
        outline: none;
        box-shadow: 0 0 0 2pt #c5c5c575;
    }


    .gjs-pn-views-container {
        height: auto !important;
    }
</style>
    "#;

    //declare the file path.
    let path = "src/views/components/grapesjs.html.tera";

    // create the file.
    std::fs::File::create(path)?;

    // ensure the file exists.
    std::fs::metadata(path)?;

    //write the file.
    write_to_file(path, html.as_bytes())?;

    Ok(())
}

/// # Name: add_grapesjs_to_header
/// ## Description:
/// - Adds the grapesjs script to the header of the header.html.tera file.
/// - This is needed for grapesjs to work.
/// ## Arguments: None
/// ## Returns:
/// - Result<(), Error>
/// ## Example:
/// ```
/// use rustyroad::writers::templates::components::features::grapesjs_components::add_grapesjs_to_header;
///
/// add_grapesjs_to_header();
/// ```
pub fn add_grapesjs_to_header() -> Result<(), Error> {
    // define the html template
    let html = r#"

<link href="https://unpkg.com/grapesjs/dist/css/grapes.min.css" rel="stylesheet">
<script src="https://unpkg.com/grapesjs"></script>
<script src="https://unpkg.com/grapesjs-preset-webpage"></script>
<script src="https://unpkg.com/grapesjs-script-editor"></script>
<script src="https://unpkg.com/@rustyroad/editor"></script>
<script src="https://unpkg.com/idb@7.1.1/build/umd.js"></script>
    "#;

    //declare the file path.
    let path = "src/views/sections/header.html.tera";

    //read the file
    let mut contents = String::new();

    // push the html to the end of the file
    contents.push_str(html);

    //overwrite the file
    write_to_file(path, contents.as_bytes())?;

    Ok(())
}
