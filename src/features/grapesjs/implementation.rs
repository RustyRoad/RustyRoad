use crate::writers::{
    add_grapesjs_to_header, add_module_declaration, create_grapesjs_component,
    write_to_create_page_dashboard_get_controller, write_to_create_page_html,
    write_to_edit_page_html, write_to_page_dashboard_html, write_to_page_details_html,
};
use crate::writers::{
     write_to_file,
    write_to_page_dashboard_get_controller,
};
use chrono::Local;
use std::env;
use std::fs;
use std::fs::{create_dir, OpenOptions};
use std::io::Write;
use std::path::Path;

use crate::database::{run_migration, Database, DatabaseType, MigrationDirection};
use crate::features::{update_cargo_toml_for_grapesjs, update_index_controller};
use crate::generators::create_file;
use color_eyre::eyre::Result;
use eyre::Error;
use crate::features::grapesjs::grapesjs_page_controllers::{write_to_all_page_controllers, write_to_image_upload_controller};

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
            let page_template_directory = format!(
                "{}/src/views/layouts/authenticated_page/page",
                current_dir.display()
            );
            // check if the page directory exists
            let path_path = Path::new(&page_template_directory);
            if !path_path.exists() {
                // create the page directory
                create_dir(page_template_directory.clone())
                    .expect("Couldn't create page directory");
            }

            write_to_page_model()
                .await
                .expect("Couldn't write to page model");

            // Create a page controller file at the root controllers directory
            let page_controller_file_location = "src/controllers/page.rs";
            // add the new page controller to the main controller module
            let controller_name = "page".to_string();
            let controller_module = Path::new("./src/controllers/mod.rs");

            // create the file
            create_file(page_controller_file_location.clone()).unwrap_or_else(|_| {
                panic!(
                    "Error: Could not create {}",
                    page_controller_file_location.clone()
                )
            });

            // create the page details template
            write_to_page_details_html().expect("Couldn't write to page details html");


            println!("Writing to edit page html");

            write_to_edit_page_html().expect("Couldn't write to edit page html");

            println!("Writing to create page html");
            write_to_create_page_html().expect("Couldn't write to create page html");
            // run the migrations
            run_migration("page".to_string(), MigrationDirection::Up)
                .await
                .expect("Couldn't run page migration");

            write_to_all_page_controllers().expect("Couldn't write to all page controllers");

            let readme_path = format!("{}/README.md", current_dir.display());

            let readme_text = r#"
                ### Page Builder
To access the page builder, go to the /page/{pageId} URL. For example, if you are running the server locally, log in and go to localhost/page/1 to access the page builder for the page with id 1.
"#;

            // write to the readme
            write_to_file(&readme_path, readme_text.as_bytes()).expect("Couldn't write to readme");

            add_module_declaration(controller_name, &controller_module)
                .expect("Couldn't add page controller to main controller module");

            // add the model to the model module
            let model_name = "page".to_string();
            let model_path = Path::new("./src/models/mod.rs");
            add_module_declaration(model_name, &model_path)
                .expect("Couldn't add page model to main model module");

            // add the new View/Template: PageDashboard.html.tera to the views/pages directory
            write_to_page_dashboard_html().expect("Couldn't write to page dashboard html");
            write_to_page_dashboard_get_controller()
                .expect("Couldn't write to page dashboard get controller");
            // write to page_create controller
            write_to_create_page_dashboard_get_controller()
                .expect("Couldn't write to page create controller");

            // create grapesjs component
            create_grapesjs_component().expect("Couldn't create grapesjs component");

            // add grapesjs scripts to header
            add_grapesjs_to_header().expect("Couldn't add grapesjs scripts to header");

            update_index_controller()
                .await
                .expect("Couldn't update index controller");

            write_to_image_upload_controller()
                .expect("Couldn't write to image upload controller");

            update_cargo_toml_for_grapesjs()
                .expect("Couldn't update cargo toml for grapesjs");

        } else {
            println!("Couldn't get current directory");
        }
        Ok(())
    }
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

    // Original SQL queries with format! macro
    let create_page_sql = format!(
        "r#\"\
    INSERT INTO page (
        title,
            html_content,
            associated_user_id,
            summary,
            author,
            excerpt,
            slug,
            page_status,
            author_image,
            author_thumbnail,
            author_url,
            featured_image,
            featured_image_thumbnail,
            seo_title,
            seo_description,
            seo_keywords,
            seo_focus_keyphrase,
            seo_canonical_url,
            seo_no_index,
            seo_no_follow,
            seo_og_title,
            seo_og_locale,
            seo_og_type,
            seo_og_description,
            seo_og_image,
            seo_og_image_width,
            seo_og_image_height,
            seo_twitter_title,
            seo_twitter_description,
            seo_twitter_image,
            seo_twitter_image_alt,
            seo_twitter_card,
            schema_type,
            schema_page_type,
            schema_article_type,
            schema_description,
            schema_author,
            schema_publisher,
            schema_image,
            schema_url,
            schema_name,
            schema_headline,
            schema_date_published,
            schema_date_modified
        )
        VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
            $11, $12, $13, $14, $15, $16, $17, $18, $19, $20,
            $21, $22, $23, $24, $25, $26, $27, $28, $29, $30,
            $31, $32, $33, $34, $35, $36, $37, $38, $39, $40,
            $41, $42, $43, $44
        )  RETURNING *;\
    \"#"
    );

    let update_page_sql = format!(
        "r#\"\
    UPDATE page
        SET
            title = $1,
            html_content = $2,
            created_at = $3,
            associated_user_id = $4,
            summary = $5,
            author = $6,
            excerpt = $7,
            slug = $8,
            page_status = $9,
            author_image = $10,
            author_thumbnail = $11,
            author_url = $12,
            featured_image = $13,
            featured_image_thumbnail = $14,
            seo_title = $15,
            seo_description = $16,
            seo_keywords = $17,
            seo_focus_keyphrase = $18,
            seo_canonical_url = $19,
            seo_no_index = $20,
            seo_no_follow = $21,
            seo_og_title = $22,
            seo_og_locale = $23,
            seo_og_type = $24,
            seo_og_description = $25,
            seo_og_image = $26,
            seo_og_image_width = $27,
            seo_og_image_height = $28,
            seo_twitter_title = $29,
            seo_twitter_description = $30,
            seo_twitter_image = $31,
            seo_twitter_image_alt = $32,
            seo_twitter_card = $33,
            schema_type = $34,
            schema_page_type = $35,
            schema_article_type = $36,
            schema_description = $37,
            schema_author = $38,
            schema_publisher = $39,
            schema_image = $40,
            schema_url = $41,
            schema_name = $42,
            schema_headline = $43,
            schema_date_published = $44,
            schema_date_modified = $45
        WHERE id = $46
    RETURNING *;\
    \"#"
    );

    let get_page_by_id_sql = format!("r#\"SELECT * FROM page WHERE id = $1\"#;");

    let get_page_by_slug_sql = format!("r#\"SELECT * FROM page WHERE slug = $1\"#;");

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
        r#"use chrono::{{NaiveDateTime, TimeZone, DateTime, Utc}};
use rustyroad::database::{{Database, PoolConnection}};
use serde::Deserialize;
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
            pub title: String,
            pub html_content: String,
            pub created_at: Option<DateTime<Utc>>,
            pub updated_at: Option<DateTime<Utc>>,
            pub associated_user_id: i32,
            // New fields
            pub summary: Option<String>,
            pub author: Option<String>,
            pub excerpt: Option<String>,
            pub slug: Option<String>,
            pub page_status: Option<String>,
            pub author_image: Option<String>,
            pub author_thumbnail: Option<String>,
            pub author_url: Option<String>,
            pub featured_image: Option<String>,
            pub featured_image_thumbnail: Option<String>,
            pub seo_title: Option<String>,
            pub seo_description: Option<String>,
            pub seo_keywords: Option<String>,
            pub seo_focus_keyphrase: Option<String>,
            pub seo_canonical_url: Option<String>,
            pub seo_no_index: Option<bool>,
            pub seo_no_follow: Option<bool>,
            pub seo_og_title: Option<String>,
            pub seo_og_locale: Option<String>,
            pub seo_og_type: Option<String>,
            pub seo_og_description: Option<String>,
            pub seo_og_image: Option<String>,
            pub seo_og_image_width: Option<i32>,
            pub seo_og_image_height: Option<i32>,
            pub seo_twitter_title: Option<String>,
            pub seo_twitter_description: Option<String>,
            pub seo_twitter_image: Option<String>,
            pub seo_twitter_image_alt: Option<String>,
            pub seo_twitter_card: Option<String>,
            pub schema_type: Option<String>,
            pub schema_page_type: Option<String>,
            pub schema_article_type: Option<String>,
            pub schema_description: Option<String>,
            pub schema_author: Option<String>,
            pub schema_publisher: Option<String>,
            pub schema_image: Option<String>,
            pub schema_url: Option<String>,
            pub schema_name: Option<String>,
            pub schema_headline: Option<String>,
            pub schema_date_published: Option<NaiveDateTime>,
            pub schema_date_modified: Option<NaiveDateTime>,
        }}

        impl Page {{
            pub fn new() -> Self {{
                Self {{
                    id: None,
                    title: "".to_string(),
                    html_content: "".to_string(),
                    created_at: Some(Utc::now()),
                    updated_at: Some(Utc::now()),
                    associated_user_id: 0,
                    summary: None,
                    author: None,
                    excerpt: None,
                    slug: None,
                    page_status: None,
                    author_image: None,
                    author_thumbnail: None,
                    author_url: None,
                    featured_image: None,
                    featured_image_thumbnail: None,
                    seo_title: None,
                    seo_description: None,
                    seo_keywords: None,
                    seo_focus_keyphrase: None,
                    seo_canonical_url: None,
                    seo_no_index: None,
                    seo_no_follow: None,
                    seo_og_title: None,
                    seo_og_locale: None,
                    seo_og_type: None,
                    seo_og_description: None,
                    seo_og_image: None,
                    seo_og_image_width: None,
                    seo_og_image_height: None,
                    seo_twitter_title: None,
                    seo_twitter_description: None,
                    seo_twitter_image: None,
                    seo_twitter_image_alt: None,
                    seo_twitter_card: None,
                    schema_type: None,
                    schema_page_type: None,
                    schema_article_type: None,
                    schema_description: None,
                    schema_author: None,
                    schema_publisher: None,
                    schema_image: None,
                    schema_url: None,
                    schema_name: None,
                    schema_headline: None,
                    schema_date_published: None,
                    schema_date_modified: None,
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
            page: Page,
        ) -> Result<serde_json::Value, sqlx::Error> {{
            let sql = {create_page_sql};

            let database = Database::get_database_from_rustyroad_toml().unwrap();

            {pool_connection_code}

            let new_page:  Page = sqlx::query_as(&sql)
                .bind(page.title)
                .bind(page.html_content)
                .bind(page.associated_user_id)
                .bind(page.summary)
                .bind(page.author)
                .bind(page.excerpt)
                .bind(page.slug)
                .bind(page.page_status)
                .bind(page.author_image)
                .bind(page.author_thumbnail)
                .bind(page.author_url)
                .bind(page.featured_image)
                .bind(page.featured_image_thumbnail)
                .bind(page.seo_title)
                .bind(page.seo_description)
                .bind(page.seo_keywords)
                .bind(page.seo_focus_keyphrase)
                .bind(page.seo_canonical_url)
                .bind(page.seo_no_index)
                .bind(page.seo_no_follow)
                .bind(page.seo_og_title)
                .bind(page.seo_og_locale)
                .bind(page.seo_og_type)
                .bind(page.seo_og_description)
                .bind(page.seo_og_image)
                .bind(page.seo_og_image_width)
                .bind(page.seo_og_image_height)
                .bind(page.seo_twitter_title)
                .bind(page.seo_twitter_description)
                .bind(page.seo_twitter_image)
                .bind(page.seo_twitter_image_alt)
                .bind(page.seo_twitter_card)
                .bind(page.schema_type)
                .bind(page.schema_page_type)
                .bind(page.schema_article_type)
                .bind(page.schema_description)
                .bind(page.schema_author)
                .bind(page.schema_publisher)
                .bind(page.schema_image)
                .bind(page.schema_url)
                .bind(page.schema_name)
                .bind(page.schema_headline)
                .bind(page.schema_date_published)
                .bind(page.schema_date_modified)
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
            let sql = {get_page_page_html}
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
        page: Page,
    ) -> Result<serde_json::Value, sqlx::Error> {{
        let sql = {update_page_sql};
        let database = Database::get_database_from_rustyroad_toml().unwrap();
        {pool_connection_code}
        let updated_page: Page = sqlx::query_as(&sql)
            .bind(page.title)
            .bind(page.html_content)
            .bind(page.created_at)
            .bind(page.associated_user_id)
            .bind(page.summary)
            .bind(page.author)
            .bind(page.excerpt)
            .bind(page.slug)
            .bind(page.page_status)
            .bind(page.author_image)
            .bind(page.author_thumbnail)
            .bind(page.author_url)
            .bind(page.featured_image)
            .bind(page.featured_image_thumbnail)
            .bind(page.seo_title)
            .bind(page.seo_description)
            .bind(page.seo_keywords)
            .bind(page.seo_focus_keyphrase)
            .bind(page.seo_canonical_url)
            .bind(page.seo_no_index)
            .bind(page.seo_no_follow)
            .bind(page.seo_og_title)
            .bind(page.seo_og_locale)
            .bind(page.seo_og_type)
            .bind(page.seo_og_description)
            .bind(page.seo_og_image)
            .bind(page.seo_og_image_width)
            .bind(page.seo_og_image_height)
            .bind(page.seo_twitter_title)
            .bind(page.seo_twitter_description)
            .bind(page.seo_twitter_image)
            .bind(page.seo_twitter_image_alt)
            .bind(page.seo_twitter_card)
            .bind(page.schema_type)
            .bind(page.schema_page_type)
            .bind(page.schema_article_type)
            .bind(page.schema_description)
            .bind(page.schema_author)
            .bind(page.schema_publisher)
            .bind(page.schema_image)
            .bind(page.schema_url)
            .bind(page.schema_name)
            .bind(page.schema_headline)
            .bind(page.schema_date_published)
            .bind(page.schema_date_modified)
            .bind(id)
            .fetch_one(&pool_connection)
            .await?;

        Ok(serde_json::json!({{
            "status": "success",
            "message": "Page updated successfully",
            "data": updated_page
        }}))

    }}


    /// # Name: get_all_pages
    /// ### Description: Gets all pages
    /// ### Returns: Result<serde_json::Value, sqlx::Error>
    /// ### Example:
    /// ```
    /// use rustyroad::models::page::Page;
    /// let result = Page::get_all_pages();
    /// ```
    pub async fn get_all_pages() -> Result<serde_json::Value, sqlx::Error> {{
        let sql = "SELECT * FROM page";
        let database = Database::get_database_from_rustyroad_toml().unwrap();
        {pool_connection_code}
        let pages: Vec<Page> = sqlx::query_as(&sql).fetch_all(&pool_connection).await?;
        Ok(serde_json::json!({{
            "status": "success",
            "message": "Pages retrieved successfully",
            "data": pages
        }}))
    }}


    /// # Name: get_page_by_slug
    /// ### Description: Gets a page by slug
    /// ### Parameters: slug: String
    /// ### Returns: Result<serde_json::Value, sqlx::Error>
    /// ### Example:
    /// ```
    /// use rustyroad::features::Page;
    /// let slug = "index";
    /// let result = Page::get_page_by_slug(slug);
    /// ```
    pub async fn get_page_by_slug(slug: String) -> Result<Page, sqlx::Error> {{
        let sql = {get_page_by_slug_sql}
        let database = Database::get_database_from_rustyroad_toml().unwrap();


        {pool_connection_code}

         let page: Page = sqlx::query_as(&sql)
            .bind(slug)
            .fetch_one(&pool_connection)
            .await?;
        Ok(page)
    }}



    /// # Name: delete_page
    /// ### Description: Deletes a page
    /// ### Parameters: id: i32
    /// ### Returns: Result<serde_json::Value, sqlx::Error>
    /// ### Example:
    /// ```
    /// use rustyroad::models::page::Page;
    /// let id = 1;
    /// let result = Page::delete_page(id);
    /// ```
    pub async fn delete_page(id: i32) -> Result<serde_json::Value, sqlx::Error> {{
        let sql = "DELETE FROM page WHERE id = $1";
        let database = Database::get_database_from_rustyroad_toml().unwrap();
        {pool_connection_code}
        sqlx::query(&sql).bind(id).execute(&pool_connection).await?;
        Ok(serde_json::json!({{
            "status": "success",
            "message": "Page deleted successfully",
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
        update_page_sql = update_page_sql,
        get_page_page_html = get_page_by_id_sql,
        pool_connection_code = pool_connection_code
    );

    page_model_file
        .write_all(page_model_contents.as_bytes())
        .unwrap_or_else(|_| {
            panic!(
                "Error: Could not write to {}",
                page_model_file_location.clone()
            )
        });

    let database = Database::get_database_from_rustyroad_toml()
        .expect("Failed to get database from rustyroad.toml");

    let database_type = database.database_type;

    let page_migration_contents = match database_type {
        DatabaseType::Postgres => {
            format!(
                r#"
CREATE TABLE IF NOT EXISTS page (
    id SERIAL PRIMARY KEY,
    title TEXT,
    html_content TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    associated_user_id INTEGER NOT NULL,
    summary TEXT,
    author TEXT,
    excerpt TEXT,
    slug TEXT,
    page_status TEXT,
    author_image TEXT,
    author_thumbnail TEXT,
    author_url TEXT,
    featured_image TEXT,
    featured_image_thumbnail TEXT,
    seo_title TEXT,
    seo_description TEXT,
    seo_keywords TEXT,
    seo_focus_keyphrase TEXT,
    seo_canonical_url TEXT,
    seo_no_index BOOLEAN,
    seo_no_follow BOOLEAN,
    seo_og_title TEXT,
    seo_og_locale TEXT,
    seo_og_type TEXT,
    seo_og_description TEXT,
    seo_og_image TEXT,
    seo_og_image_width INTEGER,
    seo_og_image_height INTEGER,
    seo_twitter_title TEXT,
    seo_twitter_description TEXT,
    seo_twitter_image TEXT,
    seo_twitter_image_alt TEXT,
    seo_twitter_card TEXT,
    schema_type TEXT,
    schema_page_type TEXT,
    schema_article_type TEXT,
    schema_description TEXT,
    schema_author TEXT,
    schema_publisher TEXT,
    schema_image TEXT,
    schema_url TEXT,
    schema_name TEXT,
    schema_headline TEXT,
    schema_date_published TIMESTAMP,
    schema_date_modified TIMESTAMP
);
    "#
            )
        }
        DatabaseType::Mysql => {
            format!(
                r#"
CREATE TABLE IF NOT EXISTS page (
    id INT AUTO_INCREMENT PRIMARY KEY,
    title TEXT,
    html_content TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    associated_user_id INT NOT NULL,
    summary TEXT,
    author TEXT,
    excerpt TEXT,
    slug TEXT,
    page_status TEXT,
    author_image TEXT,
    author_thumbnail TEXT,
    author_url TEXT,
    featured_image TEXT,
    featured_image_thumbnail TEXT,
    seo_title TEXT,
    seo_description TEXT,
    seo_keywords TEXT,
    seo_focus_keyphrase TEXT,
    seo_canonical_url TEXT,
    seo_no_index BOOLEAN,
    seo_no_follow BOOLEAN,
    seo_og_title TEXT,
    seo_og_locale TEXT,
    seo_og_type TEXT,
    seo_og_description TEXT,
    seo_og_image TEXT,
    seo_og_image_width INT,
    seo_og_image_height INT,
    seo_twitter_title TEXT,
    seo_twitter_description TEXT,
    seo_twitter_image TEXT,
    seo_twitter_image_alt TEXT,
    seo_twitter_card TEXT,
    schema_type TEXT,
    schema_page_type TEXT,
    schema_article_type TEXT,
    schema_description TEXT,
    schema_author TEXT,
    schema_publisher TEXT,
    schema_image TEXT,
    schema_url TEXT,
    schema_name TEXT,
    schema_headline TEXT,
    schema_date_published TIMESTAMP,
    schema_date_modified TIMESTAMP
);
    "#
            )
        }
        DatabaseType::Sqlite => {
            format!(
                r#"
                    CREATE TABLE IF NOT EXISTS page (
                        id INTEGER PRIMARY KEY AUTOINCREMENT,
                        title TEXT,
                        html_content TEXT NOT NULL,
                        created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
                        updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
                        associated_user_id INTEGER NOT NULL,
                        summary TEXT,
                        author TEXT,
                        excerpt TEXT,
                        slug TEXT,
                        page_status TEXT,
                        author_image TEXT,
                        author_thumbnail TEXT,
                        author_url TEXT,
                        featured_image TEXT,
                        featured_image_thumbnail TEXT,
                        seo_title TEXT,
                        seo_description TEXT,
                        seo_keywords TEXT,
                        seo_focus_keyphrase TEXT,
                        seo_canonical_url TEXT,
                        seo_no_index BOOLEAN,
                        seo_no_follow BOOLEAN,
                        seo_og_title TEXT,
                        seo_og_locale TEXT,
                        seo_og_type TEXT,
                        seo_og_description TEXT,
                        seo_og_image TEXT,
                        seo_og_image_width INTEGER,
                        seo_og_image_height INTEGER,
                        seo_twitter_title TEXT,
                        seo_twitter_description TEXT,
                        seo_twitter_image TEXT,
                        seo_twitter_image_alt TEXT,
                        seo_twitter_card TEXT,
                        schema_type TEXT,
                        schema_page_type TEXT,
                        schema_article_type TEXT,
                        schema_description TEXT,
                        schema_author TEXT,
                        schema_publisher TEXT,
                        schema_image TEXT,
                        schema_url TEXT,
                        schema_name TEXT,
                        schema_headline TEXT,
                        schema_date_published DATETIME,
                        schema_date_modified DATETIME
                    );
                "#
            )
        }
        DatabaseType::Mongo => {
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

// next step, create the controller to render the page that loads all the pages.
/// Name: create_page_list_page
/// Description: Creates the page that loads all the pages
/// Returns: Result<(), Error>
/// Example:
/// ```
/// use rustyroad::features::grapesjs::implementation::create_page_list_page;
/// let result = create_page_list_page();
/// ```
/// ToDo: Add the page_list.html.tera to the page_list.rs controller
/// ToDo: Finish this method
pub fn create_page_list_page() -> Result<(), Error> {
    let contents: String = r#"
    {% extends 'layouts/authenticated_page/authenticated_page.html.tera' %}
{% block title %}{{title | default(value="Dashboard", boolean=true)}}{% endblock title %}

{% block authenticated_content %}
{{ super() }}


<div style="height: 92vh; width: 100%;">
{% include 'components/grapesjs.html.tera' ignore missing %}

"#
    .to_string();

    let path = Path::new("src/views/layouts/authenticated_page/page/page_list.html.tera");
    if !path.exists() {
        println!("Creating the page_list.html.tera file...");
        create_file("src/views/layouts/authenticated_page/page/page_list.html.tera")
            .expect("Error creating the page_list.html.tera file");
    }
    write_to_file(
        "src/views/layouts/authenticated_page/page/page_list.html.tera",
        contents.as_bytes(),
    )
    .unwrap_or_else(|_| panic!("Error: Could not write to page_list.html"));
    Ok(())
}

// also need a controller that will load the page by the slug
// need to update the page model to include the slug

// also create the template for the page that loads all the pages
// this will be a table with the page title, description, and a link to the page builder
