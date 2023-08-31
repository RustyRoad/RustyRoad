use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use crate::writers::{add_new_controller_to_main_rs, write_to_controller_name_html, write_to_file, write_to_new_get_controller};
use crate::models::grapes_js::*;
use color_eyre::eyre::Result;
use eyre::Error;
use crate::database::{create_migration, find_migration_dir, MigrationDirection, run_migration};
use crate::generators::create_file;

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

        write_to_grapes_model().await.expect("Couldn't write to grapes model");

        write_to_edit_page_html();

        // run the migrations
        run_migration("grapes_js".to_string(), MigrationDirection::Up).await.expect("Couldn't run grapes_js migration");
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

    create_file("/views/pages/edit_page.html").unwrap_or_else(|_| panic!("Error: Could not create edit_page.html"));

    write_to_file("/views/pages/edit_page.html", contents.as_bytes())
        .unwrap_or_else(|why| panic!("Couldn't write to edit_page.html: {}", why.to_string()));
}


pub async fn write_to_grapes_model() -> Result<(), Error> {
    fs::read_to_string("rustyroad.toml")
        .unwrap_or_else(|_| panic!("Error: This is not a RustyRoad project. Please run `rustyroad new` to create a new project."));
    let grapes_model_file_location = "src/models/grapes_js.rs";

    // create the file
    create_file(grapes_model_file_location.clone()).unwrap_or_else(|_| panic!("Error: Could not create {}", grapes_model_file_location.clone()));

    let mut grapes_model_file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(grapes_model_file_location.clone())
        .unwrap_or_else(|_| panic!("Error: Could not open {}", grapes_model_file_location.clone()));



    let grapes_model_contents = format!(r#"

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct HtmlGrapesJs {{
        pub id: i32,
        pub html_content: String,
        pub created_at: DateTime<chrono::Utc>,
        pub updated_at: DateTime<chrono::Utc>,
        pub associated_user_id: i32,
        pub metadata: String,
    }}

    impl HtmlGrapesJs {{
        pub fn new() -> Self {{
            Self {{
                id: 0,
                html_content: "".to_string(),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
                associated_user_id: 0,
                metadata: "".to_string(),
            }}
        }}
    }}"#);

    grapes_model_file.write_all(grapes_model_contents.as_bytes()).unwrap_or_else(|_| panic!("Error: Could not write to {}", grapes_model_file_location.clone()));


    let grapes_js_migration_file_location = create_migration("grapes_js").await.unwrap_or_else(|_| panic!("Error: Could not create migration for grapes_js"));

    // convert to a string
    let grapes_js_migration_file_location = grapes_js_migration_file_location;

    let grapes_js_migration_contents = format!(r#"
    CREATE TABLE IF NOT EXISTS grapes_js (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        html_content TEXT NOT NULL,
        created_at DATETIME NOT NULL,
        updated_at DATETIME NOT NULL,
        associated_user_id INTEGER NOT NULL,
        metadata TEXT NOT NULL
    );
    "#);

    // get the migration directory
    let grapes_js_migration_directory = find_migration_dir("/config/database/migrations".to_string(), "grapes_js".to_string()).unwrap_or_else(|_| panic!("Error: Could not find migration directory for grapes_js"));

    // write to the up.sql file
write_to_file(format!("{}/up.sql", grapes_js_migration_directory).as_str(), grapes_js_migration_contents.as_bytes()).unwrap_or_else(|_| panic!("Error: Could not write to up.sql for grapes_js"));

    // write to the down.sql file
write_to_file(format!("{}/down.sql", grapes_js_migration_directory).as_str(), "DROP TABLE grapes_js;".as_bytes()).unwrap_or_else(|_| panic!("Error: Could not write to down.sql for grapes_js"));
    Ok(())
}
