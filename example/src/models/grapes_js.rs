use actix_web::web::to;
use chrono::{DateTime, Utc, NaiveDateTime, TimeZone};
use rustyroad::database::{Database, DatabaseType};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, FromRow, Row};

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
}

impl<'a> FromRow<'a, PgRow> for HtmlGrapesJs {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            html_content: row.try_get("html_content")?,
            created_at: TimeZone::from_utc_datetime(&Utc, &row.try_get::<NaiveDateTime, _>("created_at")?),
            updated_at: TimeZone::from_utc_datetime(&Utc, &row.try_get::<NaiveDateTime, _>("updated_at")?),
            associated_user_id: row.try_get("associated_user_id")?,
            metadata: row.try_get("metadata")?,
        })
    }
}

pub async fn get_db_pool() -> Result<sqlx::PgPool, sqlx::Error> {
    let database = Database::get_database_from_rustyroad_toml().unwrap();

    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        database.username, database.password, database.host, database.port, database.name
    );

    let db_pool = sqlx::PgPool::connect(&database_url).await?;
    Ok(db_pool)
}
