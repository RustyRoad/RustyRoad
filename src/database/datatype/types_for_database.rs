use std::{
    collections::{hash_map::Entry, HashMap},
    fmt,
};

use crate::database::DatabaseType;

use super::{
    DataTypeCategory, MySqlTypeMap, MySqlTypes, PostgresTypes, PostgresTypesMap, SqliteTypeMap,
    SqliteTypes,
};

/// A struct that holds the types for each database.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
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

    pub fn get_postgres_types<'a>(&'a self, category: &'a DataTypeCategory) -> impl Iterator + 'a {
        let types_for_database =
            category.get_data_types_from_data_type_category(DatabaseType::Postgres);

        types_for_database.postgres.types.into_iter()
    }

    pub fn get_mysql_types<'a>(&'a self, category: &'a DataTypeCategory) -> impl Iterator + 'a {
        let types_for_database =
            category.get_data_types_from_data_type_category(DatabaseType::Mysql);

        types_for_database.mysql.types.into_iter()
    }

    pub fn get_sqlite_types<'a>(&'a self, category: &'a DataTypeCategory) -> impl Iterator + 'a {
        let types_for_database =
            category.get_data_types_from_data_type_category(DatabaseType::Sqlite);

        types_for_database.sqlite.types.into_iter()
    }
    // need to create a generic type for this function
    /// Returns an iterator over the given datase type and category.
    pub fn get_types<'a>(
        &'a self,
        database_type: &DatabaseType,
        category: &DataTypeCategory,
    ) -> Vec<PostgresTypes> {
        let types_for_database =
            category.get_data_types_from_data_type_category(database_type.clone());
        let mut entries: Vec<PostgresTypes> = match database_type {
            DatabaseType::Postgres => match types_for_database {
                TypesForDatabase {
                    postgres: PostgresTypesMap { types },
                    ..
                } => types
                    .into_iter()
                    .map(|(_, types)| types)
                    .flatten()
                    .collect(),
            },
            DatabaseType::Mysql => {
                // Assuming you'll have a similar structure for MySQL types
                // Vec::new()
                todo!()
            }
            DatabaseType::Sqlite => {
                // Assuming you'll have a similar structure for SQLite types
                // Vec::new()
                todo!()
            }
            DatabaseType::Mongo => todo!(),
        };

        // This will not work unless you implement or derive PartialOrd for PostgresTypes
        entries.sort();
        entries
    }
}

impl fmt::Display for TypesForDatabase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut string = String::new();

        string.push_str("Postgres Types:\n");
        for (category, types) in &self.postgres.types {
            string.push_str(&format!("{}: {:?}\n", category, types));
        }

        string.push_str("\nMySQL Types:\n");
        for (category, types) in &self.mysql.types {
            string.push_str(&format!("{}: {:?}\n", category, types));
        }

        string.push_str("\nSQLite Types:\n");
        for (category, types) in &self.sqlite.types {
            string.push_str(&format!("{}: {:?}\n", category, types));
        }

        write!(f, "{}", string)
    }
}

// Implement the IntoIterator trait for TypesForDatabase
impl<'a> IntoIterator for &'a TypesForDatabase {
    type Item = String;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut vec: Vec<Self::Item> = Vec::new();

        vec.push("Postgres Types:".to_string());
        for (category, types) in &self.postgres.types {
            vec.push(format!("{}: {:?}\n", category, types));
        }

        vec.push("\nMySQL Types:".to_string());
        for (category, types) in &self.mysql.types {
            vec.push(format!("{}: {:?}\n", category, types));
        }

        vec.push("\nSQLite Types:".to_string());
        for (category, types) in &self.sqlite.types {
            vec.push(format!("{}: {:?}\n", category, types));
        }

        vec.into_iter()
    }
}
