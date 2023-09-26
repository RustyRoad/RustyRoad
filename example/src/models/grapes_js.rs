use actix_web::web::to;
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use rustyroad::database::{Database, DatabaseType};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, FromRow, Row};

/// # Name: HtmlGrapesJs
/// ### Description: A struct that represents a page created with GrapesJS
/// ### Example:
/// ```
/// use rustyroad::models::grapes_js::HtmlGrapesJs;
///
/// let new_html = HtmlGrapesJs::new();
/// ```
/// ### Fields:
/// * id: Option<i32>
/// * html_content: String
/// * created_at: DateTime<chrono::Utc>
/// * updated_at: DateTime<chrono::Utc>
/// * associated_user_id: i32
/// * metadata: String
/// ### Methods:
/// * create_new_database_page(new_html: HtmlGrapesJs) -> Result<serde_json::Value, sqlx::Error>
/// * get_page_by_id(id: i32) -> Result<HtmlGrapesJs, sqlx::Error>
/// * get_db_pool() -> Result<sqlx::PgPool, sqlx::Error>
/// ### Example:
/// ```
/// use rustyroad::models::grapes_js::HtmlGrapesJs;
///
/// let new_html = HtmlGrapesJs::new();
/// let result = HtmlGrapesJs::create_new_database_page(new_html);
/// ```
/// ### Example:
/// ```
/// use rustyroad::models::grapes_js::HtmlGrapesJs;
///
/// let id = 1;
///
/// let result = HtmlGrapesJs::get_page_by_id(id);
/// ```
///
/// ### Example:
/// ```
/// use rustyroad::models::grapes_js::HtmlGrapesJs;
///
/// let result = HtmlGrapesJs::get_db_pool();
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HtmlGrapesJs {
    pub id: Option<i32>,
    pub html_content: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<chrono::Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: DateTime<chrono::Utc>,
    pub associated_user_id: i32,
    pub metadata: String,
}

impl HtmlGrapesJs {
    pub fn new() -> Self {
        Self {
            id: None,
            html_content: "".to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            associated_user_id: 0,
            metadata: "".to_string(),
        }
    }

    /// # Name: create_new_database_page
    /// ### Description: Creates a new database page
    /// ### Parameters: new_html: HtmlGrapesJs
    /// ### Returns: Result<serde_json::Value, sqlx::Error>
    /// ### Example:
    /// ```
    /// use rustyroad::models::grapes_js::HtmlGrapesJs;
    ///
    /// let new_html = HtmlGrapesJs::new();
    /// let result = HtmlGrapesJs::create_new_database_page(new_html);
    /// ```
    pub async fn create_new_database_page(
        new_html: HtmlGrapesJs,
    ) -> Result<serde_json::Value, sqlx::Error> {
        let sql = r#"
        INSERT INTO grapes_js (html_content, created_at, updated_at, associated_user_id, metadata)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *"#;

        let pool = get_db_pool().await.unwrap();

        let new_grapes_js_page: HtmlGrapesJs = sqlx::query_as(&sql)
            .bind(new_html.html_content)
            .bind(new_html.created_at.naive_utc())
            .bind(new_html.updated_at.naive_utc())
            .bind(new_html.associated_user_id)
            .bind(new_html.metadata)
            .fetch_one(&pool)
            .await?;

        Ok(serde_json::json!({
            "status": "success",
            "message": "Page saved successfully",
            "data": new_grapes_js_page
        }))
    }

    /// # Name: get_page_by_id
    /// ### Description: Gets a page by id
    /// ### Parameters: id: i32
    /// ### Returns: Result<serde_json::Value, sqlx::Error>
    /// ### Example:
    /// ```
    /// use rustyroad::models::grapes_js::HtmlGrapesJs;
    ///
    /// let id = 1;
    /// let result = HtmlGrapesJs::get_page_by_id(id);
    /// ```
    pub async fn get_page_by_id(id: i32) -> Result<HtmlGrapesJs, sqlx::Error> {
        let sql = r#"
        SELECT * FROM grapes_js WHERE id = $1"#;

        let pool = get_db_pool().await.unwrap();

        let grapes_js_page: HtmlGrapesJs = sqlx::query_as(&sql).bind(id).fetch_one(&pool).await?;

        Ok(grapes_js_page)
    }
}

impl<'a> FromRow<'a, PgRow> for HtmlGrapesJs {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            html_content: row.try_get("html_content")?,
            created_at: TimeZone::from_utc_datetime(
                &Utc,
                &row.try_get::<NaiveDateTime, _>("created_at")?,
            ),
            updated_at: TimeZone::from_utc_datetime(
                &Utc,
                &row.try_get::<NaiveDateTime, _>("updated_at")?,
            ),
            associated_user_id: row.try_get("associated_user_id")?,
            metadata: row.try_get("metadata")?,
        })
    }
}

/// # Name: get_db_pool
///
/// ### Description: Gets the database pool
/// ### Returns: Result<sqlx::PgPool, sqlx::Error>
/// ### Example:
/// ```
/// use rustyroad::models::grapes_js::HtmlGrapesJs;
///
/// let result = HtmlGrapesJs::get_db_pool();
/// ```
pub async fn get_db_pool() -> Result<sqlx::PgPool, sqlx::Error> {
    let database = Database::get_database_from_rustyroad_toml().unwrap();

    match database.database_type {
        DatabaseType::Postgres => {
            let database_url = format!(
                "postgres://{}:{}@{}:{}/{}",
                database.username,
                database.password,
                database.host,
                database.port,
                database.database_name
            );
            let db_pool = sqlx::PgPool::connect(&database_url).await?;
            Ok(db_pool)
        }
        DatabaseType::Sqlite => {
            let database_url = format!(
                "sqlite://{}",
                database.database_name
            );
            let db_pool = sqlx::PgPool::connect(&database_url).await?;
            Ok(db_pool)
        }
        DatabaseType::Mysql => {
            let database_url = format!(
                "mysql://{}:{}@{}:{}/{}",
                database.username,
                database.password,
                database.host,
                database.port,
                database.database_name
            );
            let db_pool = sqlx::PgPool::connect(&database_url).await?;
            Ok(db_pool)
        }
        _ => {
            println!("Database type not supported");
        }
    }

    let db_pool = sqlx::PgPool::connect(&database_url).await?;
    Ok(db_pool)
}
