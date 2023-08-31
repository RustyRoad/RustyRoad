

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct HtmlGrapesJs {
        pub id: i32,
        pub html_content: String,
        pub created_at: DateTime<chrono::Utc>,
        pub updated_at: DateTime<chrono::Utc>,
        pub associated_user_id: i32,
        pub metadata: String,
    }

    impl HtmlGrapesJs {
        pub fn new() -> Self {
            Self {
                id: 0,
                html_content: "".to_string(),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
                associated_user_id: 0,
                metadata: "".to_string(),
            }
        }
    }