use crate::writers::{add_new_controller_to_main_rs, write_to_controller_name_html, write_to_file, write_to_new_get_controller};
use crate::models::grapes_js::*;
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

        // create a new edit page route
        add_new_controller_to_main_rs("edit_page").expect("Couldn't add new controller to main.rs");

        write_to_new_get_controller("edit_page".to_string()).expect("Couldn't write to new get controller");

        write_to_controller_name_html("edit_page").expect("Couldn't write to edit_page.html.tera");


        // set grapesJs
        let Html = HtmlGrapesJs::new();
    }
}




pub fn write_to_edit_page_html() {
    let contents: String = r#"
    <div id="gjs" style="height:0px; overflow:hidden">
        <div style="margin:100px 100px 25px; padding:25px; font:caption">
            This is a demo content from _index.html. You can use this template file for
            development purpose. It won't be stored in your git repository
        </div>
    </div>

    <style>
        body,
        html {
            height: 100%;
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
    </style>

    <script>
        const escapeName = (name) => `${name}`.trim().replace(/([^a-z0-9\w-:/]+)/gi, '-');

        window.editor = grapesjs.init({
            height: '100%',
            container: '#gjs',
            showOffsets: true,
            fromElement: true,
            noticeOnUnload: false,
            storageManager: false,
            selectorManager: { escapeName },
            plugins: ['grapesjs-tailwind'],
            pluginsOpts: {
                'grapesjs-tailwind': { /* Test here your options  */ }
            }
        });

        editor.Panels.addButton('options', {
            id: 'update-theme',
            className: 'fa fa-adjust',
            command: 'open-update-theme',
            attributes: {
                title: 'Update Theme',
                'data-tooltip-pos': 'bottom',
            },
        });
    </script>
    "#.to_string();

    write_to_file("/views/pages/edit_page.html", contents.as_bytes())
        .unwrap_or_else(|why| panic!("Couldn't write to edit_page.html: {}", why.to_string()));
}