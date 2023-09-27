
                CREATE TABLE IF NOT EXISTS page (
    id SERIAL PRIMARY KEY,
    html_content TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    associated_user_id INTEGER NOT NULL,
    metadata TEXT NOT NULL
);
    