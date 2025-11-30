# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.22] - 2025-11-30

### Added
- **Migration Generator Constraint Support**: Added comprehensive constraint parsing for both `CREATE TABLE` and `ALTER TABLE` migrations
  - `nullable` / `null` - Explicitly mark columns as nullable (no warning)
  - `not_null` - Add NOT NULL constraint
  - `primary_key` - Add PRIMARY KEY constraint
  - `unique` - Add UNIQUE constraint
  - `default=<value>` - Set default value (auto-quotes strings, supports booleans and NULL)
  - `references=table(column)` - Add foreign key constraint with explicit table/column reference

### Fixed
- Fixed "Unsupported constraint 'nullable'" warning when using nullable columns in migrations
- `ALTER TABLE` migrations now properly parse and apply column constraints (previously ignored)

### Example Usage
```bash
# Create table with constraints
rustyroad migration generate users \
  id:serial:primary_key \
  email:string:not_null,unique \
  name:string:nullable \
  role:string:default=user \
  created_at:timestamp:not_null

# Add columns with constraints  
rustyroad migration generate add_quickbooks_fields_to_employees \
  quickbooks_employee_id:string:nullable \
  quickbooks_employee_name:string:nullable \
  quickbooks_mapped_at:timestamp:nullable

# Foreign key with explicit reference
rustyroad migration generate add_author_to_posts \
  author_id:integer:not_null,references=users(id)
```

## [1.0.21] - Previous Release

- Initial constraint support for CREATE TABLE migrations
- Basic migration generation with type mapping
