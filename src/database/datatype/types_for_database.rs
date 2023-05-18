use std::collections::HashMap;

use super::{
    DataTypeCategory, MySqlTypeMap, MySqlTypes, PostgresTypes, PostgresTypesMap, SqliteTypeMap,
    SqliteTypes,
};

pub struct TypesForDatabase {
    pub postgres: PostgresTypesMap,
    pub mysql: MySqlTypeMap,
    pub sqlite: SqliteTypeMap,
}

impl TypesForDatabase {
    pub fn new() -> Self {
        Self {
            postgres: PostgresTypesMap {
                types: HashMap::new(),
            },
            mysql: MySqlTypeMap {
                types: HashMap::new(),
            },
            sqlite: SqliteTypeMap {
                types: HashMap::new(),
            },
        }
    }

    pub fn add_postgres_type(&mut self, category: String, ty: PostgresTypes) {
        let vec = self
            .postgres
            .types
            .entry(category)
            .or_insert_with(PostgresTypes);
        vec.push(ty);
    }

    pub fn add_mysql_type(&mut self, ty: MySqlTypes) {
        let category = ty.category();
        self.mysql
            .types
            .entry(category)
            .or_insert_with(Vec::new)
            .push(ty);
    }

    pub fn add_sqlite_type(&mut self, ty: SqliteTypes) {
        let category = ty.category();
        self.sqlite
            .entry(category)
            .or_insert_with(Vec::new)
            .push(ty);
    }

    pub fn get_postgres_types(&self, category: &DataTypeCategory) -> Option<&Vec<PostgresTypes>> {
        self.postgres.get(category)
    }

    pub fn get_mysql_types(&self, category: &DataTypeCategory) -> Option<&Vec<MySqlTypes>> {
        self.mysql.get(category)
    }

    pub fn get_sqlite_types(&self, category: &DataTypeCategory) -> Option<&Vec<SqliteTypes>> {
        self.sqlite.get(category)
    }
}
