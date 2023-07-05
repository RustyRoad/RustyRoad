use super::{
    category::DataTypeCategory, mysql_types::MySqlTypes, types_for_database::TypesForDatabase,
};
use crate::database::{PostgresTypes, SqliteTypes};
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
    ) -> Vec<Self::DatabaseType> {
        let mut entries: Vec<PostgresTypes> =
            self.get_database_types(data_types_for_category, data_type_category);

        entries.sort();

        entries
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
    ) -> Vec<Self::DatabaseType> {
        let mut entries: Vec<MySqlTypes> =
        

        entries.sort();

        entries
    }
}

impl DatabaseTypeTrait for SqliteDatabaseType {
    type DatabaseType = SqliteTypes;
    type DataType = TypesForDatabase;
    type DataTypeCategory = DataTypeCategory;

    fn get_database_types(
        &self,
        data_types_for_category: &Self::DataType,
        data_type_category: &Self::DataTypeCategory,
    ) -> Vec<Self::DatabaseType> {
        let mut entries: Vec<SqliteTypes> =
            self.get_database_types(data_types_for_category, data_type_category);
        entries
    }
}
