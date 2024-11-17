use strum::IntoEnumIterator;

use super::{category::DataTypeCategory, types_for_database::TypesForDatabase};
use crate::database::{MySqlTypes, PostgresTypes};
use std::fmt::{self, Display, Formatter};
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DatabaseType {
    Postgres,
    Mysql,
    Sqlite,
    Mongo,
}

impl Display for DatabaseType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let database_type = match *self {
            DatabaseType::Postgres => "postgres",
            DatabaseType::Mysql => "mysql",
            DatabaseType::Sqlite => "sqlite",
            DatabaseType::Mongo => "mongo",
        };
        write!(f, "{}", database_type)
    }
}

pub trait DatabaseTypeTrait {
    type DatabaseType;
    type DataType: Clone;
    type DataTypeCategory: Clone;

    fn get_database_types(
        &self,
        data_types_for_category: &Self::DataType,
        data_type_category: &Self::DataTypeCategory,
    ) -> Vec<Self::DataType>;
}

pub struct PostgresDatabaseType;
pub struct MySqlDatabaseType;
pub struct SqliteDatabaseType;

impl DatabaseTypeTrait for PostgresDatabaseType {
    type DatabaseType = PostgresTypes;
    type DataType = TypesForDatabase;
    type DataTypeCategory = DataTypeCategory;

    /// # Name: get_database_types
    /// ## Description
    /// This function returns a vector of all the database types for the given data type category.
    /// ## Parameters
    /// * `data_types_for_category` - A reference to a vector of data types for the given data type category.
    /// * `data_type_category` - A reference to the data type category.
    /// ## Returns
    /// A vector of all the database types for the given data type category.
    /// ## Example
    /// ```
    /// use rustyroad::database::{DatabaseTypeTrait, PostgresDatabaseType, types_for_database::TypesForDatabase, category::DataTypeCategory, databasetype::DatabaseType};
    /// let postgres_database_type = PostgresDatabaseType;
    /// let data_types_for_category = TypesForDatabase::new();
    /// let data_type_category = DataTypeCategory::new("Text");
    ///
    /// match data_type_category {
    ///    Some(data_type_category) => {
    ///       let database_types = postgres_database_type.get_database_types(&data_types_for_category, &data_type_category);
    ///      assert_eq!(database_types.len(), 1);
    ///     assert_eq!(database_types[0].get_postgres_types(&data_type_category).len(), 5);
    ///   },
    ///  None => {
    ///    println!("No data type category found");
    /// }
    /// }
    /// ```
    fn get_database_types(
        &self,
        data_types_for_category: &Self::DataType,
        data_type_category: &Self::DataTypeCategory,
    ) -> Vec<TypesForDatabase> {
        let data_types = PostgresTypes::iter();

        let mut types_for_database = TypesForDatabase::new();

        for data_type in data_types {
            let _data_type = data_type.to_string();
            let _data_type =
                data_type_category.get_data_types_from_data_type_category(DatabaseType::Postgres);
            let data_type = data_types_for_category.get_postgres_types(data_type_category);
            types_for_database.add_postgres_type(data_type_category.to_string(), data_type);
        }

        let types_for_database = vec![types_for_database];
        types_for_database
    }
}

impl DatabaseTypeTrait for MySqlDatabaseType {
    type DatabaseType = MySqlTypes;
    type DataType = TypesForDatabase;
    type DataTypeCategory = DataTypeCategory;

    /// # Name: get_database_types
    /// ## Description
    /// This function returns a vector of all the database types for the given data type category.
    /// ## Parameters
    /// * `data_types_for_category` - A reference to a vector of data types for the given data type category.
    /// * `data_type_category` - A reference to the data type category.
    /// ## Returns
    /// A vector of all the database types for the given data type category.
    /// ## Example
    /// ```
    /// use rustyroad::database::{DatabaseTypeTrait, MySqlDatabaseType, types_for_database::TypesForDatabase, category::DataTypeCategory, databasetype::DatabaseType};
    /// let mysql_database_type = MySqlDatabaseType;
    /// let data_types_for_category = TypesForDatabase::new();
    /// let data_type_category = DataTypeCategory::new("Text");
    ///
    /// match data_type_category {
    ///   Some(data_type_category) => {
    ///     let database_types = mysql_database_type.get_database_types(&data_types_for_category, &data_type_category);
    ///    assert_eq!(database_types.len(), 1);
    ///  assert_eq!(database_types[0].get_mysql_types(&data_type_category).len(), 5);
    /// }
    /// None => {
    ///  println!("No data type category found");
    /// }
    /// }
    /// ```
    fn get_database_types(&self, data_types_for_category: &Self::DataType, data_type_category: &Self::DataTypeCategory) -> Vec<TypesForDatabase> {
        let   database_types = MySqlTypes::iter();

        let mut types_for_database = TypesForDatabase::new();

        for data_type in database_types {
            let _data_type = data_type.to_string();
            let _data_type = data_type_category.get_data_types_from_data_type_category(DatabaseType::Mysql);
            let data_type = data_types_for_category.get_mysql_types(data_type_category);
            types_for_database.add_mysql_type(data_type_category.to_string(), data_type).expect("Failed to add mysql type");
        }

        let types_for_database = vec![types_for_database];
        types_for_database


    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_postgres_database_type() {
        let postgres_db_type = PostgresDatabaseType;

        let data_types_for_category = TypesForDatabase::new();
        let data_type_category = DataTypeCategory::Text;

        let result =
            postgres_db_type.get_database_types(&data_types_for_category, &data_type_category);

        // Assert that the result is not empty
        assert!(!result.is_empty());

        println!("{:?}", result);
        let types_for_database = &result[0].postgres.types;
        // Assert that the result contains only one element
        if let Some(text_types) = types_for_database.get("Text") {
            // text_types is a reference to the Vec of types for "Text"
            for text_type in text_types {
                println!("{:?}", text_type);
            }

            assert_eq!(text_types.len(), 5);
        } else {
            println!("No types found for \"Text\"");
        }
        assert!(types_for_database.contains_key("Text"));
        assert_eq!(types_for_database.len(), 1);
    }
}
