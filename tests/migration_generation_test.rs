use rustyroad::database::migrations::create_migration;
use std::fs;
use tempfile::TempDir;

#[tokio::test]
async fn test_add_column_migration_generation() {
    let temp_dir = TempDir::new().unwrap();
    let original_dir = std::env::current_dir().unwrap();
    
    // Change to temp directory
    std::env::set_current_dir(temp_dir.path()).unwrap();
    
    // Create rustyroad.toml
    let toml_content = r#"
[rustyroad_project]
name = "test_project"
[database]
database_name = "test_db"
database_user = "test_user"
database_password = "test_password"
database_host = "localhost"
database_port = "5432"
database_type = "postgres"
"#;
    fs::write("rustyroad.toml", toml_content).unwrap();
    
    // Create migration directory
    fs::create_dir_all("config/database/migrations").unwrap();
    
    // Test add column migration
    let result = create_migration("add_page_id_to_funnel_steps", vec!["page_id:integer".to_string()]).await;
    assert!(result.is_ok());
    
    // Find the generated migration directory
    let migration_dirs = fs::read_dir("config/database/migrations").unwrap();
    let migration_dir = migration_dirs
        .filter_map(|entry| entry.ok())
        .find(|entry| entry.file_name().to_string_lossy().contains("add_page_id_to_funnel_steps"))
        .unwrap();
    
    // Check up.sql
    let up_sql = fs::read_to_string(migration_dir.path().join("up.sql")).unwrap();
    assert!(up_sql.contains("ALTER TABLE funnel_steps"));
    assert!(up_sql.contains("ADD COLUMN page_id INTEGER"));
    assert!(up_sql.contains("ADD CONSTRAINT fk_page FOREIGN KEY (page_id) REFERENCES pages(id)"));
    
    // Check down.sql
    let down_sql = fs::read_to_string(migration_dir.path().join("down.sql")).unwrap();
    assert!(down_sql.contains("ALTER TABLE funnel_steps"));
    assert!(down_sql.contains("DROP COLUMN page_id"));
    
    // Restore original directory
    std::env::set_current_dir(original_dir).unwrap();
}

#[tokio::test]
async fn test_create_table_migration_generation() {
    let temp_dir = TempDir::new().unwrap();
    let original_dir = std::env::current_dir().unwrap();
    
    // Change to temp directory
    std::env::set_current_dir(temp_dir.path()).unwrap();
    
    // Create rustyroad.toml
    let toml_content = r#"
[rustyroad_project]
name = "test_project"
[database]
database_name = "test_db"
database_user = "test_user"
database_password = "test_password"
database_host = "localhost"
database_port = "5432"
database_type = "postgres"
"#;
    fs::write("rustyroad.toml", toml_content).unwrap();
    
    // Create migration directory
    fs::create_dir_all("config/database/migrations").unwrap();
    
    // Test create table migration
    let result = create_migration("create_users_table", vec!["name:string".to_string(), "email:string".to_string()]).await;
    assert!(result.is_ok());
    
    // Find the generated migration directory
    let migration_dirs = fs::read_dir("config/database/migrations").unwrap();
    let migration_dir = migration_dirs
        .filter_map(|entry| entry.ok())
        .find(|entry| entry.file_name().to_string_lossy().contains("create_users_table"))
        .unwrap();
    
    // Check up.sql
    let up_sql = fs::read_to_string(migration_dir.path().join("up.sql")).unwrap();
    assert!(up_sql.contains("CREATE TABLE IF NOT EXISTS create_users_table"));
    assert!(up_sql.contains("name VARCHAR(255)"));
    assert!(up_sql.contains("email VARCHAR(255)"));
    
    // Check down.sql
    let down_sql = fs::read_to_string(migration_dir.path().join("down.sql")).unwrap();
    assert!(down_sql.contains("DROP TABLE IF EXISTS create_users_table"));
    
    // Restore original directory
    std::env::set_current_dir(original_dir).unwrap();
}