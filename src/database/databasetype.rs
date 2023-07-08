use super::{
    category::DataTypeCategory, mysql_types::MySqlTypes, types_for_database::TypesForDatabase,
};
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
        let mut database_types = Vec::new();
        match self {
            PostgresDatabaseType => {
                let postgres_types = data_types_for_category.get_postgres_types(data_type_category);
                for postgres_type in postgres_types {
                    let mut types_for_database = TypesForDatabase::new();
                    types_for_database
                        .add_postgres_type(data_type_category.to_string(), vec![postgres_type]);
                    database_types.push(types_for_database);
                }
            }
        }
        database_types
    }
}

impl DatabaseTypeTrait for MySqlDatabaseType {
    type DatabaseType = MySqlTypes;
    type DataType = TypesForDatabase;
    type DataTypeCategory = DataTypeCategory;

    fn get_database_types(
        &self,
        data_types_for_category: &Self::DataType,
        data_type_category: &Self::DataTypeCategory,
    ) -> Vec<TypesForDatabase> {
        let mut database_types = Vec::new();
        match self {
            MySqlDatabaseType => {
                let mysql_types = data_types_for_category.get_mysql_types(data_type_category);
                for mysql_type in mysql_types {
                    let mut types_for_database = TypesForDatabase::new();
                    types_for_database
                        .add_mysql_type(data_type_category.to_string(), vec![mysql_type]);
                    database_types.push(types_for_database);
                }
            }
        }
        database_types
    }
}

impl DatabaseTypeTrait for SqliteDatabaseType {
    type DatabaseType = String;
    type DataType = TypesForDatabase;
    type DataTypeCategory = DataTypeCategory;

    fn get_database_types(
        &self,
        data_types_for_category: &Self::DataType,
        data_type_category: &Self::DataTypeCategory,
    ) -> Vec<TypesForDatabase> {
        let mut database_types = Vec::new();
        match self {
            SqliteDatabaseType => {
                let sqlite_types = data_types_for_category.get_sqlite_types(data_type_category);
                for sqlite_type in sqlite_types {
                    let mut types_for_database = TypesForDatabase::new();
                    types_for_database
                        .add_sqlite_type(data_type_category.to_string(), vec![sqlite_type]);
                    database_types.push(types_for_database);
                }
            }
        }
        database_types
    }
}
