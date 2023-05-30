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
    /// Creates a new `TypesForDatabase` instance.
    ///
    /// # Returns
    ///
    /// A new `TypesForDatabase` instance.
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

    /// Adds a PostgreSQL type to the `TypesForDatabase` instance.
    ///
    /// # Arguments
    ///
    /// * `category` - The category of the type.
    /// * `ty` - A vector of `PostgresTypes` representing the type.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustyroad::database::{TypesForDatabase, PostgresTypes, DataTypeCategory};
    ///
    /// let mut types_for_db = TypesForDatabase::new();
    ///
    /// types_for_db.add_postgres_type("Text".to_string(), vec![PostgresTypes::Text]);
    /// ```
    pub fn add_postgres_type(&mut self, category: String, ty: Vec<PostgresTypes>) {
        match self.postgres.types.entry(category) {
            Entry::Occupied(entry) => {
                let types = entry.into_mut();
                types.extend(ty);
            }
            Entry::Vacant(entry) => {
                entry.insert(ty);
            }
        }
    }

    /// Adds a MySQL type to the `TypesForDatabase` instance.
    ///
    /// # Arguments
    ///
    /// * `category` - The category of the type.
    /// * `ty` - A vector of `MySqlTypes` representing the type.
    ///
    /// # Returns
    ///
    /// A mutable reference to the vector of `MySqlTypes` for the specified category.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustyroad::database::{TypesForDatabase, MySqlTypes, DataTypeCategory};
    ///
    /// let mut types_for_db = TypesForDatabase::new();
    ///
    /// let types = types_for_db.add_mysql_type("Numeric".to_string(), vec![MySqlTypes::Decimal(10, 2)]);
    /// assert_eq!(types, Ok(&mut vec![MySqlTypes::Decimal(10, 2)]));
    /// ```
    pub fn add_mysql_type(
        &mut self,
        category: String,
        ty: Vec<MySqlTypes>,
    ) -> Result<&mut Vec<MySqlTypes>, String> {
        match self.mysql.types.entry(category) {
            Entry::Occupied(entry) => {
                let types = entry.into_mut();
                types.extend(ty);
                Ok(types)
            }
            Entry::Vacant(entry) => Ok(entry.insert(ty)),
        }
    }

    /// Adds a SQLite type to the `TypesForDatabase` instance.
    ///
    /// # Arguments
    ///
    /// * `category` - The category of the type.
    /// * `ty` - A vector of `SqliteTypes` representing the type.
    ///
    /// # Returns
    ///
    /// A mutable reference to the vector of `SqliteTypes` for the specified category.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustyroad::{TypesForDatabase, SqliteTypes, DataTypeCategory};
    ///
    /// let mut types_for_db = TypesForDatabase::new();
    ///
    /// let types = types_for_db.add_sqlite_type("Text".to_string(), vec![SqliteTypes::Text]);
    /// assert_eq!(types, Ok(&mut vec![SqliteTypes::Text]));
    /// ```
    pub fn add_sqlite_type(
        &mut self,
        category: String,
        ty: Vec<SqliteTypes>,
    ) -> Result<&mut Vec<SqliteTypes>, String> {
        match self.sqlite.types.entry(category) {
            Entry::Occupied(entry) => {
                let types = entry.into_mut();
                types.extend(ty);
                Ok(types)
            }
            Entry::Vacant(entry) => Ok(entry.insert(ty)),
        }
    }

    /// Retrieves the PostgreSQL types for a given `DataTypeCategory`.
    ///
    /// # Arguments
    ///
    /// * `category` - The `DataTypeCategory` for which to retrieve the types.
    ///
    /// # Returns
    ///
    /// A vector of `PostgresTypes` corresponding to the specified category.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustyroad::database::{TypesForDatabase, PostgresTypes::*, DataTypeCategory};
    ///
    /// let types_for_db = TypesForDatabase::new();
    ///
    /// let category = &DataTypeCategory::Numeric;
    /// let mut postgres_types = types_for_db.get_postgres_types(category);
    ///
    /// postgres_types.sort();
    ///
    /// assert_eq!(postgres_types, vec![SmallInt, Integer, BigInt, Decimal, Real, DoublePrecision, Numeric, Serial, BigSerial, Money]);
    ///
    /// ```
    pub fn get_postgres_types<'a>(&'a self, category: &'a DataTypeCategory) -> Vec<PostgresTypes> {
        let types_for_database =
            category.get_data_types_from_data_type_category(DatabaseType::Postgres);

        let TypesForDatabase {
            postgres: PostgresTypesMap { types },
            ..
        } = types_for_database;

        let mut entries: Vec<PostgresTypes> = types
            .into_values()
            .flatten()
            .collect();

        entries.sort();

        entries
    }

    /// Retrieves the MySQL types for a given `DataTypeCategory`.
    ///
    /// # Arguments
    ///
    /// * `category` - The `DataTypeCategory` for which to retrieve the types.
    ///
    /// # Returns
    ///
    /// An iterator over the MySQL types corresponding to the specified category.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustyroad::{TypesForDatabase, MySqlTypes, DataTypeCategory};
    ///
    /// let types_for_db = TypesForDatabase::new();
    ///
    /// let category = &DataTypeCategory::Text;
    /// let mysql_types = types_for_db.get_mysql_types(category);
    /// ```
    pub fn get_mysql_types<'a>(&'a self, category: &'a DataTypeCategory) -> impl Iterator + 'a {
        let types_for_database =
            category.get_data_types_from_data_type_category(DatabaseType::Mysql);

        types_for_database.mysql.types.into_iter()
    }

    /// Retrieves the SQLite types for a given `DataTypeCategory`.
    ///
    /// # Arguments
    ///
    /// * `category` - The `DataTypeCategory` for which to retrieve the types.
    ///
    /// # Returns
    ///
    /// An iterator over the SQLite types corresponding to the specified category.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustyroad::{TypesForDatabase, SqliteTypes, DataTypeCategory};
    ///
    /// let types_for_db = TypesForDatabase::new();
    ///
    /// let category = &DataTypeCategory::DateTime;
    /// let sqlite_types = types_for_db.get_sqlite_types(category);
    /// ```
    pub fn get_sqlite_types<'a>(&'a self, category: &'a DataTypeCategory) -> impl Iterator + 'a {
        let types_for_database =
            category.get_data_types_from_data_type_category(DatabaseType::Sqlite);

        types_for_database.sqlite.types.into_iter()
    }

    /// Retrieves the types for a given `DataTypeCategory` and `DatabaseType`.
    ///
    /// # Arguments
    ///
    /// * `category` - The `DataTypeCategory` for which to retrieve the types.
    /// * `database_type` - The `DatabaseType` for which to retrieve the types.
    ///
    /// # Returns
    ///
    /// A vector of `PostgresTypes` corresponding to the specified category and database type.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustyroad::{TypesForDatabase, PostgresTypes, DataTypeCategory, DatabaseType};
    /// use rustyroad::database::TypesForDatabase;
    ///
    /// let types_for_db = TypesForDatabase::new();
    ///
    /// let category = &DataTypeCategory::Numeric;
    /// let database_type = DatabaseType::Postgres;
    /// let types = types_for_db.get_types(category, database_type);
    /// ```
    pub fn get_types(
        &self,
        category: &DataTypeCategory,
        database_type: DatabaseType,
    ) -> Vec<PostgresTypes> {
        let types_for_database =
            category.get_data_types_from_data_type_category(database_type.clone());

        let mut entries: Vec<PostgresTypes> = match database_type {
            DatabaseType::Postgres => {
                let TypesForDatabase {
                    postgres: PostgresTypesMap { types },
                    ..
                } = types_for_database;
                types
                    .into_iter()
                    .map(|(_, types)| types)
                    .flatten()
                    .collect()
            }
            DatabaseType::Mysql => {
                // Assuming you'll have a similar structure for MySQL types
                // Vec::new()
                todo!()
            }
            DatabaseType::Sqlite => {
                // Assuming you'll have a similar structure for SQLite types
                // Vec.new()
                todo!()
            }
            DatabaseType::Mongo => todo!(),
        };

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
