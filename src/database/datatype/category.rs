use super::TypesForDatabase;
use crate::database::datatype::*;
use crate::DatabaseType;
use std::borrow::Borrow;
use std::fmt;
use std::hash::Hash;
/// # Name: DataTypeCategory
/// ## Type: Enum
/// ## Description
/// This enum is used to categorize the data types.
/// This is used when creating a new project and the user
/// selects the data types they want to use.
/// Example:
/// ```rust
/// use crate::database::datatype::DataTypeCategory;
/// let data_type_category = DataTypeCategory::Boolean;
/// ```
/// ## Variants
/// - Boolean
/// - Numeric
/// - DateTime
/// - Text
/// - Geometric
/// - NetworkAddress
/// - Json
/// - Search
/// - Array
/// - UUID
/// - Monetary
/// - BitString
/// - Interval
/// - Composite
/// - Range
/// - Other
#[derive(Debug, Clone, PartialEq, std::cmp::Eq, Hash)]
pub enum DataTypeCategory {
    Boolean,
    Numeric,
    DateTime,
    Text,
    Geometric,
    NetworkAddress,
    Json,
    Search,
    Array,
    UUID,
    Monetary,
    BitString,
    Interval,
    Composite,
    Range,
    Other,
}

impl Borrow<str> for DataTypeCategory {
    fn borrow(&self) -> &str {
        // Implement the `Borrow` trait for `DataTypeCategory`
        match self {
            // Return the string representation of each enum variant
            DataTypeCategory::Array => "array",
            DataTypeCategory::BitString => "bit string",
            DataTypeCategory::Boolean => "boolean",
            DataTypeCategory::Composite => "composite",
            DataTypeCategory::DateTime => "date/time",
            DataTypeCategory::Geometric => "geometric",
            DataTypeCategory::Interval => "interval",
            DataTypeCategory::Json => "json",
            DataTypeCategory::Monetary => "monetary",
            DataTypeCategory::NetworkAddress => "network address",
            DataTypeCategory::Numeric => "numeric",
            DataTypeCategory::Other => "other",
            DataTypeCategory::Range => "range",
            DataTypeCategory::Search => "search",
            DataTypeCategory::Text => "text",
            DataTypeCategory::UUID => "uuid",
        }
    }
}

impl fmt::Display for DataTypeCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataTypeCategory::Boolean => write!(f, "Boolean"),
            DataTypeCategory::Numeric => write!(f, "Numeric"),
            DataTypeCategory::DateTime => write!(f, "DateTime"),
            DataTypeCategory::Text => write!(f, "Text"),
            DataTypeCategory::Geometric => write!(f, "Geometric"),
            DataTypeCategory::NetworkAddress => write!(f, "NetworkAddress"),
            DataTypeCategory::Json => write!(f, "Json"),
            DataTypeCategory::Search => write!(f, "Search"),
            DataTypeCategory::Array => write!(f, "Array"),
            DataTypeCategory::UUID => write!(f, "UUID"),
            DataTypeCategory::Monetary => write!(f, "Monetary"),
            DataTypeCategory::BitString => write!(f, "BitString"),
            DataTypeCategory::Interval => write!(f, "Interval"),
            DataTypeCategory::Composite => write!(f, "Composite"),
            DataTypeCategory::Range => write!(f, "Range"),
            DataTypeCategory::Other => write!(f, "Other"),
        }
    }
}

impl DataTypeCategory {
    pub fn new(category: &str) -> Option<DataTypeCategory> {
        match category {
            "Boolean" => Some(DataTypeCategory::Boolean),
            "Numeric" => Some(DataTypeCategory::Numeric),
            "DateTime" => Some(DataTypeCategory::DateTime),
            "Text" => Some(DataTypeCategory::Text),
            "Geometric" => Some(DataTypeCategory::Geometric),
            "NetworkAddress" => Some(DataTypeCategory::NetworkAddress),
            "Json" => Some(DataTypeCategory::Json),
            "Search" => Some(DataTypeCategory::Search),
            "Array" => Some(DataTypeCategory::Array),
            "UUID" => Some(DataTypeCategory::UUID),
            "Monetary" => Some(DataTypeCategory::Monetary),
            "BitString" => Some(DataTypeCategory::BitString),
            "Interval" => Some(DataTypeCategory::Interval),
            "Composite" => Some(DataTypeCategory::Composite),
            "Range" => Some(DataTypeCategory::Range),
            "Other" => Some(DataTypeCategory::Other),
            _ => None,
        }
    }

    pub fn get_all_categories() -> Vec<DataTypeCategory> {
        vec![
            DataTypeCategory::Boolean,
            DataTypeCategory::Numeric,
            DataTypeCategory::DateTime,
            DataTypeCategory::Text,
            DataTypeCategory::Geometric,
            DataTypeCategory::NetworkAddress,
            DataTypeCategory::Json,
            DataTypeCategory::Search,
            DataTypeCategory::Array,
            DataTypeCategory::UUID,
            DataTypeCategory::Monetary,
            DataTypeCategory::BitString,
            DataTypeCategory::Interval,
            DataTypeCategory::Composite,
            DataTypeCategory::Range,
            DataTypeCategory::Other,
        ]
    }

    pub fn get_data_types_from_data_type_category(
        &self,
        database_type: DatabaseType,
    ) -> TypesForDatabase {
        match database_type {
            DatabaseType::Postgres => {
                let mut types_for_database = TypesForDatabase::new();

                match self {
                    DataTypeCategory::Array => types_for_database.add_postgres_type(
                        self.to_string(),
                        PostgresTypes::Array(Box::new(PostgresTypes::Text)),
                    ),
                    DataTypeCategory::Boolean => types_for_database
                        .add_postgres_type(self.to_string(), PostgresTypes::Boolean),
                    DataTypeCategory::Numeric => types_for_database
                        .add_postgres_type(self.to_string(), PostgresTypes::Numeric),
                    DataTypeCategory::DateTime => types_for_database
                        .add_postgres_type(self.to_string(), PostgresTypes::Timestamp),
                    DataTypeCategory::Text => {
                        types_for_database.add_postgres_type(self.to_string(), PostgresTypes::Text)
                    }
                    DataTypeCategory::Geometric => {
                        types_for_database.add_postgres_type(self.to_string(), PostgresTypes::Point)
                    }
                    DataTypeCategory::NetworkAddress => {
                        types_for_database.add_postgres_type(self.to_string(), PostgresTypes::Inet)
                    }
                    DataTypeCategory::Json => {
                        types_for_database.add_postgres_type(self.to_string(), PostgresTypes::Json)
                    }
                    DataTypeCategory::Search => types_for_database
                        .add_postgres_type(self.to_string(), PostgresTypes::TsVector),
                    DataTypeCategory::UUID => {
                        types_for_database.add_postgres_type(self.to_string(), PostgresTypes::Uuid)
                    }
                    DataTypeCategory::Monetary => {
                        types_for_database.add_postgres_type(self.to_string(), PostgresTypes::Money)
                    }
                    DataTypeCategory::BitString => {
                        types_for_database.add_postgres_type(self.to_string(), PostgresTypes::Bit)
                    }
                    DataTypeCategory::Interval => types_for_database
                        .add_postgres_type(self.to_string(), PostgresTypes::Interval),
                    DataTypeCategory::Composite => types_for_database
                        .add_postgres_type(self.to_string(), PostgresTypes::Circle),
                    DataTypeCategory::Range => types_for_database
                        .add_postgres_type(self.to_string(), PostgresTypes::Int4Range),
                    DataTypeCategory::Other => todo!(),
                }

                types_for_database
            }
            DatabaseType::Mysql => todo!(),
            DatabaseType::Sqlite => todo!(),
            DatabaseType::Mongo => todo!(),
        }
    }
}
