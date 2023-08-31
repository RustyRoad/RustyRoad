use crate::writers::{add_new_controller_to_main_rs, write_to_controller_name_html, write_to_controllers_mod, write_to_file, write_to_module, write_to_new_get_controller};
use chrono::Local;
use std::fs;
use std::fs::{create_dir, OpenOptions};
use std::io::Write;

use crate::database::{run_migration, MigrationDirection, Database};
use crate::generators::create_file;
use color_eyre::eyre::Result;
use eyre::Error;

pub struct GrapesJS();

impl GrapesJS {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn add_grapesjs(&mut self) -> Result<(), Error> {
        // move the contents of the grapesjs folder to the static folder
        let grapes_js_java_script: &'static [u8] =
            include_bytes!("../../grapesjs-tailwind/dist/grapesjs-tailwind.min.js");

        let new_grapes_js_path = std::path::Path::new("static/js/grapesjs-tailwind.min.js");

        // Create the directory structure if it doesn't exist
        fs::create_dir_all(new_grapes_js_path.parent().unwrap()).unwrap();

        // Write the contents of the byte array to a new file
        fs::write(new_grapes_js_path, grapes_js_java_script).unwrap();



        // create the edit page directory
        create_dir("src/controllers/edit_page")
            .expect("Couldn't create edit_page directory");



        // create the edit page controller
        create_file("src/controllers/edit_page/edit_page.rs")
            .expect("Couldn't create edit_page.rs controller file");

        // create the edit page module file
        create_file("src/controllers/edit_page/mod.rs")
            .expect("Couldn't create edit_page mod.rs file");

        let mut component = Vec::new();

        component.push("edit_page".to_string());

        append_graped_js_to_header().expect("Couldn't append grapesjs to header");

        add_new_controller_to_main_rs("edit_page").expect("Couldn't add new controller to main.rs");

        write_to_module(&"src/controllers/edit_page/mod.rs".to_string(), component)
            .expect("Couldn't write to edit_page mod.rs");

        write_to_controllers_mod(&"src/controllers/mod.rs".to_string(), "edit_page".to_string())
            .expect("Couldn't write to controllers/mod.rs");

        write_to_new_get_controller("edit_page".to_string())
            .expect("Couldn't write to new get controller");

        write_to_controller_name_html("edit_page").expect("Couldn't write to edit_page.html.tera");

        match write_to_edit_page_html() {
            Ok(_) => {
                println!("Successfully wrote to edit_page.html");
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }

        write_to_grapes_model()
            .await
            .expect("Couldn't write to grapes model");

        // run the migrations
        run_migration("grapes_js".to_string(), MigrationDirection::Up)
            .await
            .expect("Couldn't run grapes_js migration");

        Ok(())
    }
}

pub fn write_to_edit_page_html() -> Result<(), Error> {
    let contents: String = r#"
    {% extends 'base.html.tera' %}
{% block title %}Edit Page{% endblock title %}
{% block head %}
{{ super() }}
{% endblock head %}
{% block content %}
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


    <script src="/static/js/grapesjs-tailwind.min.js"></script>

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
{% endblock content %}
    "#
    .to_string();

    create_file("src/views/pages/edit_page.html.tera")
        .unwrap_or_else(|_| panic!("Error: Could not create edit_page.html.tera"));

    write_to_file("src/views/pages/edit_page.html.tera", contents.as_bytes())
        .unwrap_or_else(|_| panic!("Error: Could not write to edit_page.html"));
    Ok(())
}

pub async fn write_to_grapes_model() -> Result<(), Error> {
    fs::read_to_string("rustyroad.toml")
        .unwrap_or_else(|_| panic!("Error: This is not a RustyRoad project. Please run `rustyroad new` to create a new project."));
    let grapes_model_file_location = "src/models/grapes_js.rs";

    // create the file
    create_file(grapes_model_file_location.clone()).unwrap_or_else(|_| {
        panic!(
            "Error: Could not create {}",
            grapes_model_file_location.clone()
        )
    });

    let mut grapes_model_file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(grapes_model_file_location.clone())
        .unwrap_or_else(|_| {
            panic!(
                "Error: Could not open {}",
                grapes_model_file_location.clone()
            )
        });

    let grapes_model_contents = format!(
        r#"

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
    }}"#
    );

    grapes_model_file
        .write_all(grapes_model_contents.as_bytes())
        .unwrap_or_else(|_| {
            panic!(
                "Error: Could not write to {}",
                grapes_model_file_location.clone()
            )
        });

  let database =   Database::get_database_from_rustyroad_toml()
        .expect("Failed to get database from rustyroad.toml");

    let database_type = database.database_type;

  let grapes_js_migration_contents =  match database_type {
        crate::database::DatabaseType::Postgres => {
            format!(
                r#"
                CREATE TABLE IF NOT EXISTS grapes_js (
    id SERIAL PRIMARY KEY,
    html_content TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    associated_user_id INTEGER NOT NULL,
    metadata TEXT NOT NULL
);
    "#
            )
        }
        crate::database::DatabaseType::Mysql => {
            format!(
                r#"
CREATE TABLE IF NOT EXISTS grapes_js (
    id INT PRIMARY KEY AUTO_INCREMENT,
    html_content TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW() ON UPDATE NOW(),
    associated_user_id INT NOT NULL,
    metadata TEXT NOT NULL
);
    "#
            )

        }
        crate::database::DatabaseType::Sqlite => {
            format!(
                r#"
                CREATE TABLE IF NOT EXISTS grapes_js (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    html_content TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    associated_user_id INTEGER NOT NULL,
    metadata TEXT NOT NULL
);
    "#
            )

        }
        crate::database::DatabaseType::Mongo => {
            todo!("Implement MongoDatabaseType.get_database_types")
        }
    }
    .to_string();



    // Create directory with timestamp and name of migration
    // Then create up and down files
    let folder_name = format!(
        "config/database/migrations/{}-{}",
        Local::now().format("%Y%m%d%H%M%S"),
        "grapes_js"
    );

    // create the migration directory
    create_dir(folder_name.clone())
        .unwrap_or_else(|_| panic!("Error: Could not create migration directory for grapes_js"));
    // get the migration directory
    let grapes_js_migration_directory = folder_name.clone();

    create_file(format!("{}/up.sql", grapes_js_migration_directory).as_str())
        .unwrap_or_else(|_| panic!("Error: Could not create up.sql for grapes_js"));

    create_file(format!("{}/down.sql", grapes_js_migration_directory).as_str()).unwrap_or_else(|_| panic!("Error: Could not create down.sql for grapes_js"));

    // write to the up.sql file
    write_to_file(
        format!("{}/up.sql", grapes_js_migration_directory).as_str(),
        grapes_js_migration_contents.as_bytes(),
    )
    .unwrap_or_else(|_| panic!("Error: Could not write to up.sql for grapes_js"));

    // write to the down.sql file
    write_to_file(
        format!("{}/down.sql", grapes_js_migration_directory).as_str(),
        "DROP TABLE grapes_js;".as_bytes(),
    )
    .unwrap_or_else(|_| panic!("Error: Could not write to down.sql for grapes_js"));
    Ok(())
}



pub fn append_graped_js_to_header() -> Result<(), Error> {
    let contents: String = r#"
    <link href="https://unpkg.com/grapesjs/dist/css/grapes.min.css" rel="stylesheet"/>
<script src="https://unpkg.com/grapesjs"></script>
    <script src="/static/js/grapesjs-tailwind.min.js"></script>
    "#
    .to_string();

    let header_file_location = "src/views/sections/header.html.tera";

    let mut header_file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(header_file_location.clone())
        .unwrap_or_else(|_| {
            panic!(
                "Error: Could not open {}",
                header_file_location.clone()
            )
        });

    header_file
        .write_all(contents.as_bytes())
        .unwrap_or_else(|_| {
            panic!(
                "Error: Could not write to {}",
                header_file_location.clone()
            )
        });

    Ok(())
}