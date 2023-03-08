use crate::writers::write_to_file;
use std::io::Error;

pub fn write_to_header(name: &String) -> Result<(), Error> {
    let mut contents: String = r#"<link rel="stylesheet" href="styles.css" />
<script src="https://cdn.tailwindcss.com?plugins=forms,typography,aspect-ratio,line-clamp"></script>
<title>{% block title %}{% endblock title %} - #Title</title>"#
        .to_string();
    // Append page title to the title tag
    contents = contents.replace("#Title", &name.as_str());
    write_to_file(&name, contents.as_bytes())
        .unwrap_or_else(|why| panic!("Couldn't write to {}: {}", &name, why.to_string()));
    Ok(())
}
