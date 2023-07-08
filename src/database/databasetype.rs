use strum::IntoEnumIterator;

use super::{category::DataTypeCategory, types_for_database::TypesForDatabase};
use crate::database::PostgresTypes;
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

        // Assert that the result contains only one element
        assert_eq!(result.len(), 1);
    }
}
