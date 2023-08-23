

pub struct GrapesJS();

impl GrapesJS {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn add_grapesjs(&mut self) {
        // move the contents of the grapesjs folder to the static folder
        let grapes_js_java_script = std::fs::read_to_string("grapesjs-tailwind/dist/grapesjs-tailwind.min.js").unwrap();

        let new_grapes_js_path = std::path::Path::new("static/js/grapesjs-tailwind.min.js");

        std::fs::write(new_grapes_js_path, grapes_js_java_script).unwrap();
    }
}



pub fn add_grapesjs_to_page_builder() {
todo!("Add grapesjs to page builder to the edit page view");
}