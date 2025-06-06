use chrono::{NaiveDateTime, TimeZone, DateTime, Utc};
use rustyroad::database::{Database, PoolConnection};
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
        pub struct Page {
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
        }

        impl Page {
            pub fn new() -> Self {
                Self {
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
                }
            }

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
        ) -> Result<serde_json::Value, sqlx::Error> {
            let sql = r#"INSERT INTO page (
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
        ) RETURNING *;
        "#;

            let database = Database::get_database_from_rustyroad_toml().unwrap();

            
               let pool = Database::get_db_pool(database).await.unwrap();

                let pool_connection = match pool {
                    PoolConnection::Pg(pool) => pool,

                    _ => panic!("Error getting pg pool"),
                };
                

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

            Ok(serde_json::json!({
                "status": "success",
                "message": "Page saved successfully",
                "data": new_page
            }))
        }


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
        pub async fn get_page_by_id(id: i32) -> Result<Page, sqlx::Error> {
            let sql = r#"SELECT * FROM page WHERE id = $1"#;
            let database = Database::get_database_from_rustyroad_toml().unwrap();
            
               let pool = Database::get_db_pool(database).await.unwrap();

                let pool_connection = match pool {
                    PoolConnection::Pg(pool) => pool,

                    _ => panic!("Error getting pg pool"),
                };
                
            let page: Page = sqlx::query_as(&sql).bind(id).fetch_one(&pool_connection).await?;
            Ok(page)
    }


    /// # Name: update_page
    /// ### Description: Updates a page
    /// ### Parameters: id: i32, page: Page
    /// ### Returns: Result<serde_json::Value, sqlx::Error>
    /// ### Example:
    /// ```
    /// use rustyroad::models::page::Page;
    /// let id = 1;
    /// let page = Page::new();
    /// let result = Page::update_page(id, new_html);
    /// ```
    pub async fn update_page(
        id: i32,
        page: Page,
    ) -> Result<serde_json::Value, sqlx::Error> {
        let sql = r#"
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
        RETURNING *;
              "#;
        let database = Database::get_database_from_rustyroad_toml().unwrap();
        
               let pool = Database::get_db_pool(database).await.unwrap();

                let pool_connection = match pool {
                    PoolConnection::Pg(pool) => pool,

                    _ => panic!("Error getting pg pool"),
                };
                
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

        Ok(serde_json::json!({
            "status": "success",
            "message": "Page updated successfully",
            "data": updated_page
        }))

    }


    /// # Name: get_all_pages
    /// ### Description: Gets all pages
    /// ### Returns: Result<serde_json::Value, sqlx::Error>
    /// ### Example:
    /// ```
    /// use rustyroad::models::page::Page;
    /// let result = Page::get_all_pages();
    /// ```
    pub async fn get_all_pages() -> Result<serde_json::Value, sqlx::Error> {
        let sql = "SELECT * FROM page";
        let database = Database::get_database_from_rustyroad_toml().unwrap();
        
               let pool = Database::get_db_pool(database).await.unwrap();

                let pool_connection = match pool {
                    PoolConnection::Pg(pool) => pool,

                    _ => panic!("Error getting pg pool"),
                };
                
        let pages: Vec<Page> = sqlx::query_as(&sql).fetch_all(&pool_connection).await?;
        Ok(serde_json::json!({
            "status": "success",
            "message": "Pages retrieved successfully",
            "data": pages
        }))
    }


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
    pub async fn get_page_by_slug(slug: String) -> Result<Page, sqlx::Error> {
        let sql = r#"SELECT * FROM page WHERE slug = $1"#;
        let database = Database::get_database_from_rustyroad_toml().unwrap();


        
               let pool = Database::get_db_pool(database).await.unwrap();

                let pool_connection = match pool {
                    PoolConnection::Pg(pool) => pool,

                    _ => panic!("Error getting pg pool"),
                };
                

         let page: Page = sqlx::query_as(&sql)
            .bind(slug)
            .fetch_one(&pool_connection)
            .await?;
        Ok(page)
    }



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
    pub async fn delete_page(id: i32) -> Result<serde_json::Value, sqlx::Error> {
        let sql = "DELETE FROM page WHERE id = $1";
        let database = Database::get_database_from_rustyroad_toml().unwrap();
        
               let pool = Database::get_db_pool(database).await.unwrap();

                let pool_connection = match pool {
                    PoolConnection::Pg(pool) => pool,

                    _ => panic!("Error getting pg pool"),
                };
                
        sqlx::query(&sql).bind(id).execute(&pool_connection).await?;
        Ok(serde_json::json!({
            "status": "success",
            "message": "Page deleted successfully",
        }))
    }
}


fn deserialize_unix_timestamp<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let timestamp = i64::deserialize(deserializer)?;
    Ok(chrono::Utc.timestamp_opt(timestamp, 0).single().unwrap().naive_utc())
}
