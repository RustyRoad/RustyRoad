//! SQL Migration Converter
//!
//! This module provides a smart checker that detects raw SQL migration files
//! created outside of RustyRoad's conventions (e.g., in a `./migrations/` directory)
//! and converts them to the proper RustyRoad format.
//!
//! ## Problem
//! AI agents and developers often create migration directories like `./migrations/*.sql`
//! instead of using RustyRoad's expected structure:
//! `./config/database/migrations/<timestamp>-<name>/up.sql` and `down.sql`
//!
//! ## Solution
//! This converter:
//! 1. Detects rogue migration directories/files
//! 2. Parses the SQL to understand the operations
//! 3. Generates proper up.sql and down.sql files
//! 4. Moves them to the correct RustyRoad location

use chrono::Local;
use regex::Regex;

use std::fs::{self, create_dir_all};
use std::io::{self, Read};
use std::path::{Path, PathBuf};

const MIGRATIONS_DIR: &str = "./config/database/migrations";
use crate::generators::create_file;
use crate::writers::write_to_file;

/// Represents a detected SQL migration that needs conversion
#[derive(Debug, Clone)]
pub struct DetectedMigration {
    /// Original file path
    pub source_path: PathBuf,
    /// Inferred migration name
    pub name: String,
    /// The raw SQL content
    pub sql_content: String,
    /// Parsed operations from the SQL
    pub operations: Vec<SqlOperation>,
}

/// Represents a SQL operation parsed from migration content
#[derive(Debug, Clone, PartialEq)]
pub enum SqlOperation {
    CreateTable {
        table_name: String,
        columns: Vec<String>,
        full_sql: String,
    },
    AlterTableAddColumn {
        table_name: String,
        columns: Vec<String>,
        full_sql: String,
    },
    AlterTableDropColumn {
        table_name: String,
        columns: Vec<String>,
        full_sql: String,
    },
    DropTable {
        table_name: String,
    },
    CreateIndex {
        index_name: String,
        table_name: String,
        full_sql: String,
    },
    DropIndex {
        index_name: String,
    },
    /// Raw SQL that couldn't be parsed - will be kept as-is
    RawSql {
        sql: String,
    },
}

/// Result of the conversion process
#[derive(Debug)]
pub struct ConversionResult {
    pub source_path: PathBuf,
    pub destination_path: PathBuf,
    pub migration_name: String,
    pub success: bool,
    pub message: String,
}

/// Directories commonly created by agents that should be checked for SQL migrations
const ROGUE_MIGRATION_DIRS: &[&str] = &[
    "./migrations",
    "./migration",
    "./db/migrations",
    "./db/migrate",
    "./database/migrations",
    "./sql/migrations",
    "./sql",
];

/// Check for rogue migration directories and return detected migrations
pub fn detect_rogue_migrations() -> Vec<DetectedMigration> {
    let mut detected = Vec::new();

    for dir in ROGUE_MIGRATION_DIRS {
        let path = Path::new(dir);
        if path.exists() && path.is_dir() {
            // Skip the correct RustyRoad migrations directory
            if dir == &"./config/database/migrations" {
                continue;
            }

            println!(
                "Detected rogue migration directory: '{}' - scanning for SQL files...",
                dir
            );

            if let Ok(migrations) = scan_directory_for_sql(path) {
                detected.extend(migrations);
            }
        }
    }

    // Also check for standalone SQL files in the project root
    if let Ok(entries) = fs::read_dir(".") {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "sql" {
                        // Check if it looks like a migration file
                        let filename = path
                            .file_stem()
                            .and_then(|s| s.to_str())
                            .unwrap_or("");
                        if looks_like_migration_name(filename) {
                            if let Ok(migration) = parse_sql_file(&path) {
                                detected.push(migration);
                            }
                        }
                    }
                }
            }
        }
    }

    detected
}

/// Scan a directory recursively for SQL files
fn scan_directory_for_sql(dir: &Path) -> io::Result<Vec<DetectedMigration>> {
    let mut migrations = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            // Check if this is already a proper migration directory (has up.sql/down.sql)
            let up_sql = path.join("up.sql");
            let _down_sql = path.join("down.sql");

            if up_sql.exists() {
                // This looks like a proper structure but in the wrong location
                if let Ok(migration) = parse_migration_directory(&path) {
                    migrations.push(migration);
                }
            } else {
                // Recurse into subdirectory
                if let Ok(sub_migrations) = scan_directory_for_sql(&path) {
                    migrations.extend(sub_migrations);
                }
            }
        } else if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "sql" {
                    if let Ok(migration) = parse_sql_file(&path) {
                        migrations.push(migration);
                    }
                }
            }
        }
    }

    Ok(migrations)
}

/// Parse a migration directory that has up.sql (and optionally down.sql)
fn parse_migration_directory(dir: &Path) -> io::Result<DetectedMigration> {
    let up_sql_path = dir.join("up.sql");
    let mut sql_content = String::new();
    let mut file = fs::File::open(&up_sql_path)?;
    file.read_to_string(&mut sql_content)?;

    let name = dir
        .file_name()
        .and_then(|s| s.to_str())
        .map(|s| extract_migration_name(s))
        .unwrap_or_else(|| "unknown_migration".to_string());

    let operations = parse_sql_operations(&sql_content);

    Ok(DetectedMigration {
        source_path: dir.to_path_buf(),
        name,
        sql_content,
        operations,
    })
}

/// Parse a single SQL file as a migration
fn parse_sql_file(path: &Path) -> io::Result<DetectedMigration> {
    let mut sql_content = String::new();
    let mut file = fs::File::open(path)?;
    file.read_to_string(&mut sql_content)?;

    let name = path
        .file_stem()
        .and_then(|s| s.to_str())
        .map(|s| extract_migration_name(s))
        .unwrap_or_else(|| "unknown_migration".to_string());

    let operations = parse_sql_operations(&sql_content);

    Ok(DetectedMigration {
        source_path: path.to_path_buf(),
        name,
        sql_content,
        operations,
    })
}

/// Extract a clean migration name from a filename that might have timestamps
fn extract_migration_name(filename: &str) -> String {
    // Common patterns:
    // - 20231224150552_user -> user
    // - 20231224150552-user -> user
    // - 001_create_users -> create_users
    // - V1__create_users -> create_users (Flyway style)
    // - create_users_table -> create_users_table

    let timestamp_pattern = Regex::new(r"^(\d{14}|\d{3}|V\d+)[-_](.+)$").unwrap();
    let flyway_pattern = Regex::new(r"^V\d+__(.+)$").unwrap();

    if let Some(captures) = flyway_pattern.captures(filename) {
        return captures.get(1).unwrap().as_str().to_string();
    }

    if let Some(captures) = timestamp_pattern.captures(filename) {
        return captures.get(2).unwrap().as_str().to_string();
    }

    // Remove common suffixes
    let name = filename
        .trim_end_matches(".sql")
        .trim_end_matches("_up")
        .trim_end_matches("_down")
        .trim_end_matches("-up")
        .trim_end_matches("-down");

    name.to_string()
}

/// Check if a filename looks like it could be a migration
fn looks_like_migration_name(name: &str) -> bool {
    let migration_keywords = [
        "create",
        "alter",
        "add",
        "drop",
        "modify",
        "update",
        "insert",
        "delete",
        "migration",
        "schema",
        "table",
        "index",
        "column",
    ];

    let name_lower = name.to_lowercase();

    // Check for timestamp prefix (common in migrations)
    let has_timestamp = Regex::new(r"^\d{14}").unwrap().is_match(name)
        || Regex::new(r"^\d{3}_").unwrap().is_match(name)
        || Regex::new(r"^V\d+").unwrap().is_match(name);

    if has_timestamp {
        return true;
    }

    // Check for migration keywords
    migration_keywords.iter().any(|kw| name_lower.contains(kw))
}

/// Parse SQL content into structured operations
pub fn parse_sql_operations(sql: &str) -> Vec<SqlOperation> {
    let mut operations = Vec::new();

    // Normalize the SQL - remove comments and extra whitespace
    let normalized = normalize_sql(sql);

    // Split by semicolons to get individual statements
    for statement in normalized.split(';') {
        let stmt = statement.trim();
        if stmt.is_empty() {
            continue;
        }

        if let Some(op) = parse_single_statement(stmt) {
            operations.push(op);
        }
    }

    operations
}

/// Normalize SQL by removing comments and extra whitespace
fn normalize_sql(sql: &str) -> String {
    // Remove single-line comments
    let single_line_comment = Regex::new(r"--[^\n]*").unwrap();
    let result = single_line_comment.replace_all(sql, "");

    // Remove multi-line comments
    let multi_line_comment = Regex::new(r"/\*[\s\S]*?\*/").unwrap();
    let result = multi_line_comment.replace_all(&result, "");

    // Normalize whitespace
    let extra_whitespace = Regex::new(r"\s+").unwrap();
    extra_whitespace.replace_all(&result, " ").trim().to_string()
}

/// Parse a single SQL statement into an operation
fn parse_single_statement(stmt: &str) -> Option<SqlOperation> {
    let stmt_upper = stmt.to_uppercase();

    // CREATE TABLE
    if stmt_upper.starts_with("CREATE TABLE") {
        return parse_create_table(stmt);
    }

    // DROP TABLE
    if stmt_upper.starts_with("DROP TABLE") {
        return parse_drop_table(stmt);
    }

    // ALTER TABLE
    if stmt_upper.starts_with("ALTER TABLE") {
        return parse_alter_table(stmt);
    }

    // CREATE INDEX
    if stmt_upper.starts_with("CREATE INDEX") || stmt_upper.starts_with("CREATE UNIQUE INDEX") {
        return parse_create_index(stmt);
    }

    // DROP INDEX
    if stmt_upper.starts_with("DROP INDEX") {
        return parse_drop_index(stmt);
    }

    // If we can't parse it, keep it as raw SQL
    Some(SqlOperation::RawSql {
        sql: stmt.to_string(),
    })
}

/// Parse a CREATE TABLE statement
fn parse_create_table(stmt: &str) -> Option<SqlOperation> {
    // Pattern: CREATE TABLE [IF NOT EXISTS] table_name (...)
    // Handles optional quoting with backticks, double quotes, or brackets
    let pattern =
        Regex::new(r#"(?i)CREATE\s+TABLE\s+(?:IF\s+NOT\s+EXISTS\s+)?[`"\[]?(\w+)[`"\]]?\s*\("#)
            .unwrap();

    if let Some(captures) = pattern.captures(stmt) {
        let table_name = captures.get(1)?.as_str().to_string();

        // Extract column definitions
        let columns = extract_column_definitions(stmt);

        return Some(SqlOperation::CreateTable {
            table_name,
            columns,
            full_sql: stmt.to_string(),
        });
    }

    None
}

/// Parse a DROP TABLE statement
fn parse_drop_table(stmt: &str) -> Option<SqlOperation> {
    // Pattern: DROP TABLE [IF EXISTS] table_name
    let pattern =
        Regex::new(r#"(?i)DROP\s+TABLE\s+(?:IF\s+EXISTS\s+)?[`"\[]?(\w+)[`"\]]?"#).unwrap();

    if let Some(captures) = pattern.captures(stmt) {
        let table_name = captures.get(1)?.as_str().to_string();
        return Some(SqlOperation::DropTable { table_name });
    }

    None
}

/// Parse an ALTER TABLE statement
fn parse_alter_table(stmt: &str) -> Option<SqlOperation> {
    let stmt_upper = stmt.to_uppercase();

    // Pattern: ALTER TABLE table_name ADD [COLUMN] column_name ...
    let table_pattern = Regex::new(r#"(?i)ALTER\s+TABLE\s+[`"\[]?(\w+)[`"\]]?"#).unwrap();
    let table_name = table_pattern
        .captures(stmt)?
        .get(1)?
        .as_str()
        .to_string();

    if stmt_upper.contains("ADD COLUMN") || stmt_upper.contains("ADD ") {
        // Extract added columns
        let add_pattern = Regex::new(r#"(?i)ADD\s+(?:COLUMN\s+)?[`"\[]?(\w+)[`"\]]?"#).unwrap();
        let columns: Vec<String> = add_pattern
            .captures_iter(stmt)
            .filter_map(|c| c.get(1).map(|m| m.as_str().to_string()))
            .collect();

        return Some(SqlOperation::AlterTableAddColumn {
            table_name,
            columns,
            full_sql: stmt.to_string(),
        });
    }

    if stmt_upper.contains("DROP COLUMN") || stmt_upper.contains("DROP ") {
        // Extract dropped columns
        let drop_pattern = Regex::new(r#"(?i)DROP\s+(?:COLUMN\s+)?[`"\[]?(\w+)[`"\]]?"#).unwrap();
        let columns: Vec<String> = drop_pattern
            .captures_iter(stmt)
            .filter_map(|c| c.get(1).map(|m| m.as_str().to_string()))
            .collect();

        return Some(SqlOperation::AlterTableDropColumn {
            table_name,
            columns,
            full_sql: stmt.to_string(),
        });
    }

    // Other ALTER TABLE operations - keep as raw
    Some(SqlOperation::RawSql {
        sql: stmt.to_string(),
    })
}

/// Parse a CREATE INDEX statement
fn parse_create_index(stmt: &str) -> Option<SqlOperation> {
    // Pattern: CREATE [UNIQUE] INDEX index_name ON table_name (...)
    let pattern = Regex::new(
        r#"(?i)CREATE\s+(?:UNIQUE\s+)?INDEX\s+(?:IF\s+NOT\s+EXISTS\s+)?[`"\[]?(\w+)[`"\]]?\s+ON\s+[`"\[]?(\w+)[`"\]]?"#,
    )
    .unwrap();

    if let Some(captures) = pattern.captures(stmt) {
        let index_name = captures.get(1)?.as_str().to_string();
        let table_name = captures.get(2)?.as_str().to_string();
        return Some(SqlOperation::CreateIndex {
            index_name,
            table_name,
            full_sql: stmt.to_string(),
        });
    }

    None
}

/// Parse a DROP INDEX statement
fn parse_drop_index(stmt: &str) -> Option<SqlOperation> {
    // Pattern: DROP INDEX [IF EXISTS] index_name
    let pattern =
        Regex::new(r#"(?i)DROP\s+INDEX\s+(?:IF\s+EXISTS\s+)?[`"\[]?(\w+)[`"\]]?"#).unwrap();

    if let Some(captures) = pattern.captures(stmt) {
        let index_name = captures.get(1)?.as_str().to_string();
        return Some(SqlOperation::DropIndex { index_name });
    }

    None
}

/// Extract column definitions from a CREATE TABLE statement
fn extract_column_definitions(stmt: &str) -> Vec<String> {
    // Find the content between the first ( and last )
    let start = stmt.find('(');
    let end = stmt.rfind(')');

    if let (Some(start), Some(end)) = (start, end) {
        let columns_str = &stmt[start + 1..end];

        // Split by comma, but be careful of nested parentheses
        let mut columns = Vec::new();
        let mut current = String::new();
        let mut depth = 0;

        for c in columns_str.chars() {
            match c {
                '(' => {
                    depth += 1;
                    current.push(c);
                }
                ')' => {
                    depth -= 1;
                    current.push(c);
                }
                ',' if depth == 0 => {
                    let col = current.trim().to_string();
                    if !col.is_empty() {
                        columns.push(col);
                    }
                    current.clear();
                }
                _ => current.push(c),
            }
        }

        // Don't forget the last column
        let col = current.trim().to_string();
        if !col.is_empty() {
            columns.push(col);
        }

        return columns;
    }

    Vec::new()
}

/// Generate the down.sql content from the operations
pub fn generate_down_sql(operations: &[SqlOperation]) -> String {
    let mut down_statements = Vec::new();

    // Process operations in reverse order
    for op in operations.iter().rev() {
        match op {
            SqlOperation::CreateTable { table_name, .. } => {
                down_statements.push(format!("DROP TABLE IF EXISTS {};", table_name));
            }
            SqlOperation::DropTable { table_name } => {
                // We can't regenerate a dropped table, but we can add a comment
                down_statements.push(format!(
                    "-- WARNING: Cannot automatically reverse DROP TABLE {}. Manual intervention required.",
                    table_name
                ));
            }
            SqlOperation::AlterTableAddColumn {
                table_name,
                columns,
                ..
            } => {
                for col in columns {
                    down_statements.push(format!(
                        "ALTER TABLE {} DROP COLUMN IF EXISTS {};",
                        table_name, col
                    ));
                }
            }
            SqlOperation::AlterTableDropColumn {
                table_name,
                columns,
                ..
            } => {
                // Can't reverse a DROP COLUMN without knowing the type
                for col in columns {
                    down_statements.push(format!(
                        "-- WARNING: Cannot automatically reverse DROP COLUMN {} on {}. Manual intervention required.",
                        col, table_name
                    ));
                }
            }
            SqlOperation::CreateIndex {
                index_name,
                ..
            } => {
                down_statements.push(format!("DROP INDEX IF EXISTS {};", index_name));
            }
            SqlOperation::DropIndex { index_name } => {
                down_statements.push(format!(
                    "-- WARNING: Cannot automatically reverse DROP INDEX {}. Manual intervention required.",
                    index_name
                ));
            }
            SqlOperation::RawSql { sql } => {
                down_statements.push(format!(
                    "-- WARNING: Cannot automatically generate reverse for:\n-- {}",
                    sql.replace('\n', "\n-- ")
                ));
            }
        }
    }

    if down_statements.is_empty() {
        "-- No reversible operations found\n".to_string()
    } else {
        down_statements.join("\n\n")
    }
}

/// Convert detected migrations to RustyRoad format
pub fn convert_migrations(
    migrations: Vec<DetectedMigration>,
    remove_source: bool,
) -> Vec<ConversionResult> {
    let mut results = Vec::new();

    // Ensure the target directory exists
    if let Err(e) = create_dir_all(MIGRATIONS_DIR) {
        eprintln!("Failed to create migrations directory: {}", e);
        return results;
    }

    for migration in migrations {
        let result = convert_single_migration(&migration, remove_source);
        results.push(result);
    }

    results
}

/// Convert a single migration to RustyRoad format
fn convert_single_migration(migration: &DetectedMigration, remove_source: bool) -> ConversionResult {
    let timestamp = Local::now().format("%Y%m%d%H%M%S");
    let folder_name = format!("{}-{}", timestamp, migration.name);
    let destination_path = PathBuf::from(MIGRATIONS_DIR).join(&folder_name);

    // Create the migration directory
    if let Err(e) = create_dir_all(&destination_path) {
        return ConversionResult {
            source_path: migration.source_path.clone(),
            destination_path,
            migration_name: migration.name.clone(),
            success: false,
            message: format!("Failed to create directory: {}", e),
        };
    }

    // Write up.sql
    let up_sql_path = destination_path.join("up.sql");
    if let Err(e) = create_file(up_sql_path.to_str().unwrap()) {
        return ConversionResult {
            source_path: migration.source_path.clone(),
            destination_path,
            migration_name: migration.name.clone(),
            success: false,
            message: format!("Failed to create up.sql: {}", e),
        };
    }
    if let Err(e) = write_to_file(up_sql_path.to_str().unwrap(), migration.sql_content.as_bytes()) {
        return ConversionResult {
            source_path: migration.source_path.clone(),
            destination_path,
            migration_name: migration.name.clone(),
            success: false,
            message: format!("Failed to write up.sql: {}", e),
        };
    }

    // Generate and write down.sql
    let down_sql_content = generate_down_sql(&migration.operations);
    let down_sql_path = destination_path.join("down.sql");
    if let Err(e) = create_file(down_sql_path.to_str().unwrap()) {
        return ConversionResult {
            source_path: migration.source_path.clone(),
            destination_path,
            migration_name: migration.name.clone(),
            success: false,
            message: format!("Failed to create down.sql: {}", e),
        };
    }
    if let Err(e) = write_to_file(down_sql_path.to_str().unwrap(), down_sql_content.as_bytes()) {
        return ConversionResult {
            source_path: migration.source_path.clone(),
            destination_path,
            migration_name: migration.name.clone(),
            success: false,
            message: format!("Failed to write down.sql: {}", e),
        };
    }

    // Optionally remove the source
    if remove_source {
        if migration.source_path.is_dir() {
            if let Err(e) = fs::remove_dir_all(&migration.source_path) {
                eprintln!(
                    "Warning: Could not remove source directory {:?}: {}",
                    migration.source_path, e
                );
            }
        } else if let Err(e) = fs::remove_file(&migration.source_path) {
            eprintln!(
                "Warning: Could not remove source file {:?}: {}",
                migration.source_path, e
            );
        }
    }

    ConversionResult {
        source_path: migration.source_path.clone(),
        destination_path,
        migration_name: migration.name.clone(),
        success: true,
        message: format!(
            "Successfully converted migration '{}' to RustyRoad format",
            migration.name
        ),
    }
}

/// Main entry point: detect and convert rogue migrations
/// 
/// This function is designed to be called at the start of migration commands
/// to automatically fix agent-created migrations.
/// 
/// # Arguments
/// * `auto_convert` - If true, automatically converts without prompting
/// * `remove_source` - If true, removes the source files after conversion
/// 
/// # Returns
/// Number of migrations converted
pub fn detect_and_convert_rogue_migrations(auto_convert: bool, remove_source: bool) -> usize {
    let detected = detect_rogue_migrations();

    if detected.is_empty() {
        return 0;
    }

    println!("\n=== RustyRoad Migration Converter ===");
    println!("Detected {} rogue migration(s) that need conversion:\n", detected.len());

    for (i, migration) in detected.iter().enumerate() {
        println!(
            "  {}. {} (from {:?})",
            i + 1,
            migration.name,
            migration.source_path
        );
        println!("     Operations detected:");
        for op in &migration.operations {
            match op {
                SqlOperation::CreateTable { table_name, .. } => {
                    println!("       - CREATE TABLE {}", table_name);
                }
                SqlOperation::DropTable { table_name } => {
                    println!("       - DROP TABLE {}", table_name);
                }
                SqlOperation::AlterTableAddColumn {
                    table_name,
                    columns,
                    ..
                } => {
                    println!(
                        "       - ALTER TABLE {} ADD COLUMN {}",
                        table_name,
                        columns.join(", ")
                    );
                }
                SqlOperation::AlterTableDropColumn {
                    table_name,
                    columns,
                    ..
                } => {
                    println!(
                        "       - ALTER TABLE {} DROP COLUMN {}",
                        table_name,
                        columns.join(", ")
                    );
                }
                SqlOperation::CreateIndex {
                    index_name,
                    table_name,
                    ..
                } => {
                    println!("       - CREATE INDEX {} ON {}", index_name, table_name);
                }
                SqlOperation::DropIndex { index_name } => {
                    println!("       - DROP INDEX {}", index_name);
                }
                SqlOperation::RawSql { .. } => {
                    println!("       - [Raw SQL statement]");
                }
            }
        }
        println!();
    }

    if !auto_convert {
        println!("To automatically convert these migrations, run:");
        println!("  rustyroad migration convert\n");
        println!("Or use --auto-convert flag with migration commands.\n");
        return 0;
    }

    println!("Converting migrations to RustyRoad format...\n");

    let results = convert_migrations(detected, remove_source);
    let mut success_count = 0;

    for result in &results {
        if result.success {
            success_count += 1;
            println!("  [OK] {}", result.message);
            println!("       -> {:?}", result.destination_path);
        } else {
            println!("  [FAIL] {}: {}", result.migration_name, result.message);
        }
    }

    println!(
        "\nConversion complete: {}/{} migrations converted successfully.",
        success_count,
        results.len()
    );

    if remove_source {
        println!("Source files have been removed.");
    } else {
        println!("\nNote: Source files were NOT removed. Use --remove-source to delete them.");
    }

    success_count
}

/// Check for rogue migrations and print a warning if found
/// 
/// This is a lightweight check that can be called at the start of any
/// migration command to warn users about improperly placed migrations.
pub fn warn_about_rogue_migrations() {
    let detected = detect_rogue_migrations();

    if !detected.is_empty() {
        eprintln!("\n*** WARNING: Detected {} SQL migration(s) in non-standard locations! ***", detected.len());
        eprintln!("RustyRoad expects migrations in: {}/", MIGRATIONS_DIR);
        eprintln!("\nDetected files:");
        for migration in &detected {
            eprintln!("  - {:?}", migration.source_path);
        }
        eprintln!("\nRun 'rustyroad migration convert' to fix this automatically.\n");
    }
}

/// Clean up empty rogue migration directories after conversion
pub fn cleanup_empty_rogue_dirs() {
    for dir in ROGUE_MIGRATION_DIRS {
        let path = Path::new(dir);
        if path.exists() && path.is_dir() {
            // Check if directory is empty
            if let Ok(mut entries) = fs::read_dir(path) {
                if entries.next().is_none() {
                    if let Err(e) = fs::remove_dir(path) {
                        eprintln!("Warning: Could not remove empty directory {:?}: {}", path, e);
                    } else {
                        println!("Removed empty directory: {}", dir);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_migration_name() {
        assert_eq!(
            extract_migration_name("20231224150552_user"),
            "user"
        );
        assert_eq!(
            extract_migration_name("20231224150552-create_users"),
            "create_users"
        );
        assert_eq!(
            extract_migration_name("001_create_users"),
            "create_users"
        );
        assert_eq!(
            extract_migration_name("V1__create_users"),
            "create_users"
        );
        assert_eq!(
            extract_migration_name("create_users_table"),
            "create_users_table"
        );
    }

    #[test]
    fn test_looks_like_migration_name() {
        assert!(looks_like_migration_name("20231224150552_users"));
        assert!(looks_like_migration_name("create_users_table"));
        assert!(looks_like_migration_name("add_email_to_users"));
        assert!(looks_like_migration_name("001_initial_schema"));
        assert!(looks_like_migration_name("V1__baseline"));
        assert!(!looks_like_migration_name("readme"));
        assert!(!looks_like_migration_name("config"));
    }

    #[test]
    fn test_parse_create_table() {
        let sql = "CREATE TABLE users (id SERIAL PRIMARY KEY, name VARCHAR(255) NOT NULL)";
        let ops = parse_sql_operations(sql);
        
        assert_eq!(ops.len(), 1);
        match &ops[0] {
            SqlOperation::CreateTable { table_name, columns, .. } => {
                assert_eq!(table_name, "users");
                assert_eq!(columns.len(), 2);
            }
            _ => panic!("Expected CreateTable operation"),
        }
    }

    #[test]
    fn test_parse_create_table_if_not_exists() {
        let sql = "CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY)";
        let ops = parse_sql_operations(sql);
        
        assert_eq!(ops.len(), 1);
        match &ops[0] {
            SqlOperation::CreateTable { table_name, .. } => {
                assert_eq!(table_name, "users");
            }
            _ => panic!("Expected CreateTable operation"),
        }
    }

    #[test]
    fn test_parse_alter_table_add_column() {
        let sql = "ALTER TABLE users ADD COLUMN email VARCHAR(255)";
        let ops = parse_sql_operations(sql);
        
        assert_eq!(ops.len(), 1);
        match &ops[0] {
            SqlOperation::AlterTableAddColumn { table_name, columns, .. } => {
                assert_eq!(table_name, "users");
                assert!(columns.contains(&"email".to_string()));
            }
            _ => panic!("Expected AlterTableAddColumn operation"),
        }
    }

    #[test]
    fn test_parse_drop_table() {
        let sql = "DROP TABLE IF EXISTS users";
        let ops = parse_sql_operations(sql);
        
        assert_eq!(ops.len(), 1);
        match &ops[0] {
            SqlOperation::DropTable { table_name } => {
                assert_eq!(table_name, "users");
            }
            _ => panic!("Expected DropTable operation"),
        }
    }

    #[test]
    fn test_parse_create_index() {
        let sql = "CREATE INDEX idx_users_email ON users (email)";
        let ops = parse_sql_operations(sql);
        
        assert_eq!(ops.len(), 1);
        match &ops[0] {
            SqlOperation::CreateIndex { index_name, table_name, .. } => {
                assert_eq!(index_name, "idx_users_email");
                assert_eq!(table_name, "users");
            }
            _ => panic!("Expected CreateIndex operation"),
        }
    }

    #[test]
    fn test_generate_down_sql_create_table() {
        let ops = vec![SqlOperation::CreateTable {
            table_name: "users".to_string(),
            columns: vec!["id SERIAL PRIMARY KEY".to_string()],
            full_sql: "CREATE TABLE users (id SERIAL PRIMARY KEY)".to_string(),
        }];

        let down = generate_down_sql(&ops);
        assert!(down.contains("DROP TABLE IF EXISTS users"));
    }

    #[test]
    fn test_generate_down_sql_add_column() {
        let ops = vec![SqlOperation::AlterTableAddColumn {
            table_name: "users".to_string(),
            columns: vec!["email".to_string()],
            full_sql: "ALTER TABLE users ADD COLUMN email VARCHAR(255)".to_string(),
        }];

        let down = generate_down_sql(&ops);
        assert!(down.contains("DROP COLUMN"));
        assert!(down.contains("email"));
    }

    #[test]
    fn test_parse_multiple_statements() {
        let sql = r#"
            CREATE TABLE users (id SERIAL PRIMARY KEY);
            CREATE TABLE posts (id SERIAL PRIMARY KEY, user_id INTEGER);
            CREATE INDEX idx_posts_user ON posts (user_id);
        "#;
        
        let ops = parse_sql_operations(sql);
        assert_eq!(ops.len(), 3);
    }

    #[test]
    fn test_normalize_sql_removes_comments() {
        let sql = r#"
            -- This is a comment
            CREATE TABLE users (
                id SERIAL PRIMARY KEY -- inline comment
            );
            /* Multi-line
               comment */
            CREATE TABLE posts (id INTEGER);
        "#;
        
        let normalized = normalize_sql(sql);
        assert!(!normalized.contains("--"));
        assert!(!normalized.contains("/*"));
        assert!(!normalized.contains("*/"));
        assert!(normalized.contains("CREATE TABLE users"));
        assert!(normalized.contains("CREATE TABLE posts"));
    }
}
