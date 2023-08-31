
CREATE TABLE IF NOT EXISTS grapes_js (
    id INT PRIMARY KEY AUTO_INCREMENT,
    html_content TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW() ON UPDATE NOW(),
    associated_user_id INT NOT NULL,
    metadata TEXT NOT NULL
);
    