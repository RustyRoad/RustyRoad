use crate::writers::write_to_new_update_controller;
use crate::writers::{
    write_to_file, write_to_module, write_to_new_post_controller,
};
use chrono::Local;
use std::env;
use std::fs;
use std::fs::{create_dir, OpenOptions};
use std::io::Write;

use crate::database::{run_migration, Database, DatabaseType, MigrationDirection};
use crate::features::write_to_get_page_by_id;
use crate::generators::create_file;
use color_eyre::eyre::Result;
use eyre::Error;

pub struct GrapesJs();

impl GrapesJs {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn add_page(&mut self) -> Result<(), Error> {
        // move the contents of the page folder to the static folder
        // create the edit page directory if it doesn't exist
        if let Ok(current_dir) = env::current_dir() {
            // create the page directory
            let page_directory = format!("{}/src/views/layouts/authenticated/page", current_dir.display());
            // check if the page directory exists
            let path_path = std::path::Path::new(&page_directory);
            if !path_path.exists() {
                // create the page directory
                create_dir(page_directory.clone()).expect("Couldn't create page directory");
            }

            println!("Writing to post controller");
            write_to_new_post_controller("page".to_string())
                .expect("Couldn't write to new post controller");

            write_to_new_update_controller("page".to_string())
                .expect("Couldn't write to new update controller");

            match write_to_edit_page_html() {
                Ok(_) => {
                    println!("Successfully wrote to edit_page.html");
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }

            match write_to_create_page_html() {
                Ok(_) => {
                    println!("Successfully wrote to create_page.html");
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }

            // shifting to the get pages now
            println!("Writing to get_page_by_id.rs");
            write_to_get_page_by_id()
                .await
                .expect("Couldn't write to get_page_by_id.rs");

            write_to_page_model()
                .await
                .expect("Couldn't write to page model");

            // run the migrations
            run_migration("page".to_string(), MigrationDirection::Up)
                .await
                .expect("Couldn't run page migration");

                let readme_path = format!("{}/README.md", current_dir.display());

                let readme_text = r#"
                ### Page Builder
To access the page builder, go to the /page/{pageId} URL. For example, if you are running the server locally, log in and go to localhost/page/1 to access the page builder for the page with id 1.
"#;
        

                // write to the readme
                write_to_file(&readme_path, readme_text.as_bytes())
                    .expect("Couldn't write to readme");
    
            
            } else {
                println!("Couldn't get current directory");
            }
        Ok(())
    }
    
}

pub fn write_to_edit_page_html() -> Result<(), Error> {
    let contents: String = r#"
    {% extends 'layouts/authenticated/authenticated.html.tera' %}
    {% block title %}{{title | default(value="Dashboard", boolean=true)}}{% endblock title %}
    
    {% block authenticated_content %}
    {{ super() }}
    
    

    <body id='app' class='h-full'>
        {% include 'components/navbar.html.tera'%}
        <div id="gjs" style="height: 100%; width: 100%;">
            <div style="margin:100px 100px 25px; padding:25px; font:caption">
                This is a demo content from _index.html. You can use this template file for development purpose. It
                won't be stored in your git repository
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


            .gjs-pn-views-container {
                height: auto !important;
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

        let isSaved = false;

        const saveHtml = (HtmlGrapesJs) => {
            if (!isSaved) {
                // save html to database
                fetch('/page/{{page_id}}', {
                    method: 'PATCH',
                    headers: {
                        'Content-Type': 'application/json',
                    },
                    body: JSON.stringify(
                        HtmlGrapesJs
                    ),
                })
                    .then(response => response.json())
                    .then(data => {
                        console.log('Success:', data);
                        sender.set('active', 1); // turn on the button
                    })
                    .catch((error) => {
                        console.error('Error:', error);
                        sender.set('active', 1); // turn on the button
                    });
                isSaved = true;
            }
        };

        editor.Commands.add('savePage', {
            run(editor, sender) {
                sender.set('active', 0); // turn off the button
                // get html from editor
                var html = editor.getHtml();
                // create object to save to database
                const now = Date.now();  // milliseconds since 1970-01-01T00:00:00Z
                const HtmlGrapesJs = {
                    html_content: html,
                    created_at: Math.floor(now / 1000),  // convert to seconds
                    updated_at: Math.floor(now / 1000),  // convert to seconds
                    associated_user_id: 1,
                    metadata: JSON.stringify({
                        title: 'test',
                        description: 'test',
                        keywords: 'test',
                    }),
                };
                saveHtml(HtmlGrapesJs);
            }
        });

        editor.Panels.addButton('options', {
            id: 'savePage',
            className: 'fa fa-save',
            command: 'savePage',
            attributes: {
                title: 'Save HTML',
                'data-tooltip-pos': 'bottom',
            },
        });
        </script>
        <div id='footer'>
            {% block footer %}
            {% include 'sections/footer' ignore missing %}
            {% endblock footer %}
        </div>
        </body>
</html>
    "#
    .to_string();

    create_file("src/views/layouts/authenticated/page/create_page.html.tera")
        .unwrap_or_else(|_| panic!("Error: Could not create create_page.html.tera"));

    write_to_file(
        "src/views/layouts/authenticated/page/create_page.html.tera",
        contents.as_bytes(),
    )
    .unwrap_or_else(|_| panic!("Error: Could not write to create_page.html"));
    Ok(())
}

/// # Name: write_to_page_model
/// ### Description: Writes to the page model
/// ### Returns: Result<(), Error>
/// ### Example:
/// ```
/// use rustyroad::features::grapesjs::implementation::write_to_page_model;
/// let result = write_to_page_model();
/// ```
pub async fn write_to_page_model() -> Result<(), Error> {
    fs::read_to_string("./rustyroad.toml")
        .unwrap_or_else(|_| panic!("Error: This is not a RustyRoad project. Please run `rustyroad new` to create a new project."));
    let page_model_file_location = "src/models/page.rs";

    // create the file
    create_file(page_model_file_location.clone()).unwrap_or_else(|_| {
        panic!(
            "Error: Could not create {}",
            page_model_file_location.clone()
        )
    });

    let mut page_model_file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(page_model_file_location.clone())
        .unwrap_or_else(|_| panic!("Error: Could not open {}", page_model_file_location.clone()));

    let create_page_sql = format!(
        "r#\"\
        INSERT INTO page (html_content, created_at, updated_at, associated_user_id, metadata)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *;\
        \"#;"
    );

    let update_page_sql = format!("r#\"UPDATE page SET html_content = $1, updated_at = $2, metadata = $3 WHERE id = $4 RETURNING *;\"#;");

    let get_page_page_html = format!("r#\"SELECT * FROM page WHERE id = $1\"#;");

    let database = Database::get_database_from_rustyroad_toml().unwrap();

    let pool_connection_code = match database.database_type {
        DatabaseType::Postgres => {
            format!(
                r#"
               let pool = Database::get_db_pool(database).await.unwrap();

                let pool_connection = match pool {{
                    PoolConnection::Pg(pool) => pool,

                    _ => panic!("Error getting pg pool"),
                }};
                "#
            )
        }
        DatabaseType::Mysql => {
            format!(
                r#"
                let pool = Database::get_db_pool(database).await.unwrap();

                let pool_connection = match pool {{
                    PoolConnection::MySql(pool) => pool,

                    _ => panic!("Error getting mysql pool"),
                }};
                "#
            )
        }
        DatabaseType::Sqlite => {
            format!(
                r#"
                let pool = Database::get_db_pool(database).await.unwrap();

                let pool_connection = match pool {{
                    PoolConnection::Sqlite(pool) => pool,

                    _ => panic!("Error getting sqlite pool"),
                }};
                "#
            )
        }
        DatabaseType::Mongo => {
            todo!("Implement MongoDatabaseType.get_database_types")
        }
    };

    let page_model_contents = format!(
        r#"use chrono::{{NaiveDateTime, TimeZone}};
use rustyroad::database::{{Database, PoolConnection}};
use serde::{{Deserialize, Deserializer}};
use sqlx::FromRow;
        

    /// # Name: Page
    /// ### Description: A struct that represents a page created with page
    /// ### Example:
    /// ```
    /// use rustyroad::models::page::Page;
    ///
    /// let new_html = Page::new();
    /// ```
    /// ### Fields:
    /// * id: Option<i32>
    /// * html_content: String
    /// * created_at: DateTime<chrono::Utc>
    /// * updated_at: DateTime<chrono::Utc>
    /// * associated_user_id: i32
    /// * metadata: String
    /// ### Methods:
    /// * create_new_database_page(new_html: Page) -> Result<serde_json::Value, sqlx::Error>
    /// * get_page_by_id(id: i32) -> Result<Page, sqlx::Error>
    /// * get_db_pool() -> Result<sqlx::PgPool, sqlx::Error>
    /// ### Example:
    /// ```
    /// use rustyroad::models::page::Page;
    ///
    /// let new_html = Page::new();
    /// let result = Page::create_new_database_page(new_html);
    /// ```
    /// ### Example:
    /// ```
    /// use rustyroad::models::page::Page;
    ///
    /// let id = 1;
    ///
    /// let result = Page::get_page_by_id(id);
    /// ```
    ///
    /// ### Example:
    /// ```
    /// use rustyroad::models::page::Page;
    ///
    /// let result = Page::get_db_pool();
    /// ```
        #[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize, FromRow)]
        pub struct Page {{
        pub id: Option<i32>,
        pub html_content: String,
        pub created_at: NaiveDateTime,
        pub updated_at: NaiveDateTime,
        pub associated_user_id: i32,
        pub metadata: String,
        }}

        impl Page {{
            pub fn new() -> Self {{
                Self {{
                    id: None,
                    html_content: "".to_string(),
                    created_at: chrono::Utc::now().naive_utc(),
                    updated_at: chrono::Utc::now().naive_local(),
                    associated_user_id: 0,
                    metadata: "".to_string(),
                }}
            }}

        /// # Name: create_page
        /// ### Description: Creates a new database page
        /// ### Parameters: new_html: Page
        /// ### Returns: Result<serde_json::Value, sqlx::Error>
        /// ### Example:
        /// ```
        /// use rustyroad::models::page::Page;
        ///
        /// let new_html = Page::new();
        /// let result = Page::create_new_database_page(new_html);
        /// ```
        pub async fn create_page(
            new_html: Page,
        ) -> Result<serde_json::Value, sqlx::Error> {{
            let sql = {create_page_sql}

            let database = Database::get_database_from_rustyroad_toml().unwrap();

            {pool_connection_code}

            let new_page:  Page = sqlx::query_as(&sql)
                .bind(new_html.html_content)
                .bind(new_html.created_at)
                .bind(new_html.updated_at)
                .bind(new_html.associated_user_id)
                .bind(new_html.metadata)
                .fetch_one(&pool_connection)
                .await?;

            Ok(serde_json::json!({{
                "status": "success",
                "message": "Page saved successfully",
                "data": new_page
            }}))
        }}


    /// # Name: get_page_by_id
    /// ### Description: Gets a page by id
    /// ### Parameters: id: i32
    /// ### Returns: Result<serde_json::Value, sqlx::Error>
    /// ### Example:
    /// ```
    /// use rustyroad::models::page::Page;
    ///
    /// let id = 1;
    /// let result = Page::get_page_by_id(id);
    /// ```
        pub async fn get_page_by_id(id: i32) -> Result<Page, sqlx::Error> {{
            let sql = {get_page_page_html};
            let database = Database::get_database_from_rustyroad_toml().unwrap();
            {pool_connection_code}
            let page: Page = sqlx::query_as(&sql).bind(id).fetch_one(&pool_connection).await?;
            Ok(page)
    }}


    /// # Name: update_page
    /// ### Description: Updates a page
    /// ### Parameters: id: i32, new_html: Page
    /// ### Returns: Result<serde_json::Value, sqlx::Error>
    /// ### Example:
    /// ```
    /// use rustyroad::models::page::Page;
    /// let id = 1;
    /// let new_html = Page::new();
    /// let result = Page::update_page(id, new_html);
    /// ```
    pub async fn update_page(
        id: i32,
        new_html: Page,
    ) -> Result<serde_json::Value, sqlx::Error> {{
        let sql = {update_page_sql};
        let database = Database::get_database_from_rustyroad_toml().unwrap();
        {pool_connection_code}
        let updated_page: Page = sqlx::query_as(&sql)
            .bind(new_html.html_content)
            .bind(new_html.updated_at)
            .bind(new_html.metadata)
            .bind(id)
            .fetch_one(&pool_connection)
            .await?;

        Ok(serde_json::json!({{
            "status": "success",
            "message": "Page updated successfully",
            "data": updated_page
        }}))

    }}
}}



fn deserialize_unix_timestamp<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: serde::Deserializer<'de>,
{{
    let timestamp = i64::deserialize(deserializer)?;
    Ok(chrono::Utc.timestamp_opt(timestamp, 0).single().unwrap().naive_utc())
}}
"#,
        create_page_sql = create_page_sql,
        get_page_page_html = get_page_page_html
    );

    page_model_file
        .write_all(page_model_contents.as_bytes())
        .unwrap_or_else(|_| {
            panic!(
                "Error: Could not write to {}",
                page_model_file_location.clone()
            )
        });

    // add model to models/mod.rs
    let mut components = Vec::new();

    components.push("page".to_string());

    write_to_module(&"src/models/mod.rs".to_string(), components)
        .expect("Error writing to models/mod.rs");

    let database = Database::get_database_from_rustyroad_toml()
        .expect("Failed to get database from rustyroad.toml");

    let database_type = database.database_type;

    let page_migration_contents = match database_type {
        crate::database::DatabaseType::Postgres => {
            format!(
                r#"
                CREATE TABLE IF NOT EXISTS page (
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
CREATE TABLE IF NOT EXISTS page (
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
                CREATE TABLE IF NOT EXISTS page (
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
        "page"
    );

    // create the migration directory
    create_dir(folder_name.clone())
        .unwrap_or_else(|_| panic!("Error: Could not create migration directory for page"));
    // get the migration directory
    let page_migration_directory = folder_name.clone();

    create_file(format!("{}/up.sql", page_migration_directory).as_str())
        .unwrap_or_else(|_| panic!("Error: Could not create up.sql for page"));

    create_file(format!("{}/down.sql", page_migration_directory).as_str())
        .unwrap_or_else(|_| panic!("Error: Could not create down.sql for page"));

    // write to the up.sql file
    write_to_file(
        format!("{}/up.sql", page_migration_directory).as_str(),
        page_migration_contents.as_bytes(),
    )
    .unwrap_or_else(|_| panic!("Error: Could not write to up.sql for page"));

    // write to the down.sql file
    write_to_file(
        format!("{}/down.sql", page_migration_directory).as_str(),
        "DROP TABLE page;".as_bytes(),
    )
    .unwrap_or_else(|_| panic!("Error: Could not write to down.sql for page"));
    Ok(())
}

pub fn append_graped_js_to_header() -> Result<(), Error> {
    let contents: String = r#"
<link href="https://unpkg.com/grapesjs/dist/css/grapes.min.css" rel="stylesheet">
<script src="https://unpkg.com/grapesjs"></script>
<script src="https://unpkg.com/@rustyroad/editor@0.0.1/dist/grapesjs-tailwind.min.js"></script>

    "#
    .to_string();

    let header_file_location = "src/views/sections/header.html.tera";

    let mut header_file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(header_file_location.clone())
        .unwrap_or_else(|_| panic!("Error: Could not open {}", header_file_location.clone()));

    header_file
        .write_all(contents.as_bytes())
        .unwrap_or_else(|_| panic!("Error: Could not write to {}", header_file_location.clone()));

    Ok(())
}

// need to add method to save the html to the database
// this will need a special connection pool
// we need to determine the database type from the rustyroad.toml


pub fn write_to_create_page_html() -> Result<(), Error> {
    let contents: String = r#"
    {% extends 'layouts/authenticated/authenticated.html.tera' %}
{% block title %}{{title | default(value="Dashboard", boolean=true)}}{% endblock title %}

{% block authenticated_content %}
{{ super() }}


        <div id="gjs" class="h-full" style="height: 100%; width: 100%;">
            <div style="margin:100px 100px 25px; padding:25px; font:caption">
                This is a demo content from _index.html. You can use this template file for development purpose. It
                won't be stored in your git repository
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


            .gjs-pn-views-container {
                height: auto !important;
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

        let isSaved = false;

        const saveHtml = (HtmlGrapesJs) => {
            if (!isSaved) {
                // save html to database
                fetch('/page', {
                    method: 'PATCH',
                    headers: {
                        'Content-Type': 'application/json',
                    },
                    body: JSON.stringify(
                        HtmlGrapesJs
                    ),
                })
                    .then(response => response.json())
                    .then(data => {
                        console.log('Success:', data);
                        sender.set('active', 1); // turn on the button
                    })
                    .catch((error) => {
                        console.error('Error:', error);
                        sender.set('active', 1); // turn on the button
                    });
                isSaved = true;
            }
        };

        editor.Commands.add('savePage', {
            run(editor, sender) {
                sender.set('active', 0); // turn off the button
                // get html from editor
                var html = editor.getHtml();
                // create object to save to database
                const now = Date.now();  // milliseconds since 1970-01-01T00:00:00Z
                const HtmlGrapesJs = {
                    html_content: html,
                    created_at: Math.floor(now / 1000),  // convert to seconds
                    updated_at: Math.floor(now / 1000),  // convert to seconds
                    associated_user_id: 1,
                    metadata: JSON.stringify({
                        title: 'test',
                        description: 'test',
                        keywords: 'test',
                    }),
                };
                saveHtml(HtmlGrapesJs);
            }
        });

        editor.Panels.addButton('options', {
            id: 'savePage',
            className: 'fa fa-save',
            command: 'savePage',
            attributes: {
                title: 'Save HTML',
                'data-tooltip-pos': 'bottom',
            },
        });
        </script>
        {% endblock authenticated_content %}
    "#
    .to_string();

    let path = std::path::Path::new("src/views/layouts/authenticated/page/create_page.html.tera");
    if !path.exists() {
        println!("Creating the create_page.html.tera file...");
        create_file("src/views/layouts/authenticated/page/create_page.html.tera")
            .expect("Error creating the create_page.html.tera file");
    }
    write_to_file(
        "src/views/layouts/authenticated/page/create_page.html.tera",
        contents.as_bytes(),
    )
    .unwrap_or_else(|_| panic!("Error: Could not write to create_page.html"));
    Ok(())
}       