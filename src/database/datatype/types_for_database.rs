use std::collections::{hash_map::Entry, HashMap};

use crate::database::DatabaseType;

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
        match self.postgres.types.entry(category) {
            Entry::Occupied(entry) => {
                entry.into_mut().push(ty);
            }
            Entry::Vacant(entry) => {
                entry.insert(vec![ty]);
            }
        };
    }

    pub fn add_mysql_type(
        &mut self,
        category: String,
        ty: MySqlTypes,
    ) -> Result<&mut Vec<MySqlTypes>, String> {
        match self.mysql.types.entry(category) {
            Entry::Occupied(entry) => Ok(entry.into_mut()),
            Entry::Vacant(entry) => Ok(entry.insert(vec![ty])),
        }
    }

    pub fn add_sqlite_type(
        &mut self,
        category: String,
        ty: SqliteTypes,
    ) -> Result<&mut Vec<SqliteTypes>, String> {
        match self.sqlite.types.entry(category) {
            Entry::Occupied(entry) => Ok(entry.into_mut()),
            Entry::Vacant(entry) => Ok(entry.insert(vec![ty])),
        }
    }

    pub fn get_postgres_types<'a>(&'a self, category: &DataTypeCategory) -> impl Iterator {
        let types_for_database =
            category.get_data_types_from_data_type_category(DatabaseType::Postgres);

        let ite_types = types_for_database.postgres.types.into_iter();

        ite_types
    }

    pub fn get_mysql_types<'a>(&'a self, category: &DataTypeCategory) -> impl Iterator {
        let types_for_database =
            category.get_data_types_from_data_type_category(DatabaseType::Mysql);

        let ite_types = types_for_database.mysql.types.into_iter();

        ite_types
    }

    pub fn get_sqlite_types<'a>(&'a self, category: &DataTypeCategory) -> impl Iterator {
        let types_for_database =
            category.get_data_types_from_data_type_category(DatabaseType::Sqlite);

        let ite_types = types_for_database.sqlite.types.into_iter();

        ite_types
    }
}
