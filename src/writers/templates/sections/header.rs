use crate::writers::write_to_file;
use std::io::Error;

pub fn write_to_header(name: &String) -> Result<(), Error> {
    let mut contents: String = r#"<link rel="stylesheet" href="styles.css" />
<script src="https://cdn.tailwindcss.com?plugins=forms,typography,aspect-ratio,line-clamp"></script>
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>{% block title %}{% endblock title %} - #Title</title>"#
        .to_string();
    // Append page title to the title tag
    contents = contents.replace("#Title", &name.as_str());
    write_to_file(&name, contents.as_bytes())
        .unwrap_or_else(|why| panic!("Couldn't write to {}: {}", &name, why.to_string()));
    Ok(())
}

pub fn write_to_header_with_grapesjs(name: &String) -> Result<(), Error> {
    let mut contents: String = r#"<link href="https://unpkg.com/grapesjs/dist/css/grapes.min.css" rel="stylesheet">
<script src="https://unpkg.com/grapesjs"></script>
    <script src="/js/grapesjs-tailwind.min.js"></script>"#
        .to_string();
    
    
 // append to the end of the header
    let header = std::fs::read_to_string("/views/sections/header.html").unwrap();
    
    contents = contents + &header;
    
    write_to_file(&name, contents.as_bytes())
        .unwrap_or_else(|why| panic!("Couldn't write to {}: {}", &name, why.to_string()));
    Ok(())
}