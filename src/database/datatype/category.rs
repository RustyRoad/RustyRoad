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
#[derive(Debug, Clone, PartialEq, std::cmp::Eq, Hash, PartialOrd, Ord)]
pub enum DataTypeCategory {
    Boolean,
    DateTime,
    Numeric,
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
    Text,
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
        let mut categories = vec![
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
        ];

        categories.sort();

        categories
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
            DatabaseType::Mysql => {
                let mut types_for_database =
                    TypesForDatabase::get_types(self, &database_type, category);

                match self {
                    DataTypeCategory::Array => {
                        types_for_database
                            .add_mysql_type(self.to_string(), MySqlTypes::Json)
                            .unwrap_or_else(|_| {
                                panic!("Could not add {} to types_for_database", self.to_string())
                            });
                    }

                    DataTypeCategory::Boolean => {
                        types_for_database
                            .add_mysql_type(self.to_string(), MySqlTypes::Boolean)
                            .unwrap_or_else(|_| {
                                panic!("Could not add {} to types_for_database", self.to_string())
                            });
                    }
                    DataTypeCategory::Numeric => {
                        types_for_database
                            .add_mysql_type(self.to_string(), MySqlTypes::Decimal)
                            .unwrap_or_else(|_| {
                                panic!("Could not add {} to types_for_database", self.to_string())
                            });
                    }
                    DataTypeCategory::DateTime => {
                        types_for_database
                            .add_mysql_type(self.to_string(), MySqlTypes::DateTime)
                            .unwrap_or_else(|_| {
                                panic!("Could not add {} to types_for_database", self.to_string())
                            });
                    }
                    DataTypeCategory::Text => {
                        types_for_database
                            .add_mysql_type(self.to_string(), MySqlTypes::Text)
                            .unwrap_or_else(|_| {
                                panic!("Could not add {} to types_for_database", self.to_string())
                            });
                    }
                    DataTypeCategory::Geometric => {
                        types_for_database
                            .add_mysql_type(self.to_string(), MySqlTypes::Point)
                            .unwrap_or_else(|_| {
                                panic!("Could not add {} to types_for_database", self.to_string())
                            });
                    }
                    DataTypeCategory::NetworkAddress => {
                        types_for_database
                            .add_mysql_type(self.to_string(), MySqlTypes::BigInt)
                            .unwrap_or_else(|_| {
                                panic!("Could not add {} to types_for_database", self.to_string())
                            });
                    }
                    DataTypeCategory::Json => {
                        types_for_database
                            .add_mysql_type(self.to_string(), MySqlTypes::Json)
                            .unwrap_or_else(|_| {
                                panic!("Could not add {} to types_for_database", self.to_string())
                            });
                    }
                    DataTypeCategory::Search => todo!(),
                    DataTypeCategory::UUID => {
                        types_for_database
                            .add_mysql_type(self.to_string(), MySqlTypes::VarChar)
                            .unwrap_or_else(|_| {
                                panic!("Could not add {} to types_for_database", self.to_string())
                            });
                    }
                    DataTypeCategory::Monetary => {
                        types_for_database
                            .add_mysql_type(self.to_string(), MySqlTypes::Float)
                            .unwrap_or_else(|_| {
                                panic!("Could not add {} to types_for_database", self.to_string())
                            });
                    }
                    DataTypeCategory::BitString => {
                        types_for_database
                            .add_mysql_type(self.to_string(), MySqlTypes::Bit)
                            .unwrap_or_else(|_| {
                                panic!("Could not add {} to types_for_database", self.to_string())
                            });
                    }
                    DataTypeCategory::Interval => {
                        types_for_database
                            .add_mysql_type(self.to_string(), MySqlTypes::Int)
                            .unwrap_or_else(|_| {
                                panic!("Could not add {} to types_for_database", self.to_string())
                            });
                    }
                    DataTypeCategory::Composite => {
                        types_for_database
                            .add_mysql_type(self.to_string(), MySqlTypes::Json)
                            .unwrap_or_else(|_| {
                                panic!("Could not add {} to types_for_database", self.to_string())
                            });
                    }
                    DataTypeCategory::Range => {
                        types_for_database
                            .add_mysql_type(self.to_string(), MySqlTypes::Json)
                            .unwrap_or_else(|_| {
                                panic!("Could not add {} to types_for_database", self.to_string())
                            });
                    }
                    DataTypeCategory::Other => todo!(),
                }

                types_for_database
            }
            DatabaseType::Sqlite => {
                let mut types_for_database = TypesForDatabase::new();

                match self {
                    DataTypeCategory::Array => {
                        types_for_database
                            .add_sqlite_type(self.to_string(), SqliteTypes::Text)
                            .unwrap_or_else(|_| {
                                panic!("Could not add {} to types_for_database", self.to_string())
                            });
                    }

                    DataTypeCategory::Boolean => {
                        types_for_database
                            .add_sqlite_type(self.to_string(), SqliteTypes::Integer)
                            .unwrap_or_else(|_| {
                                panic!("Could not add {} to types_for_database", self.to_string())
                            });
                    }
                    DataTypeCategory::Numeric => {
                        types_for_database
                            .add_sqlite_type(self.to_string(), SqliteTypes::Real)
                            .unwrap_or_else(|_| {
                                panic!("Could not add {} to types_for_database", self.to_string())
                            });
                    }
                    DataTypeCategory::DateTime => {
                        types_for_database
                            .add_sqlite_type(self.to_string(), SqliteTypes::Text)
                            .unwrap_or_else(|_| {
                                panic!("Could not add {} to types_for_database", self.to_string())
                            });
                    }
                    DataTypeCategory::Text => {
                        types_for_database
                            .add_sqlite_type(self.to_string(), SqliteTypes::Text)
                            .unwrap_or_else(|_| {
                                panic!("Could not add {} to types_for_database", self.to_string())
                            });
                    }
                    DataTypeCategory::Geometric => {
                        types_for_database
                            .add_sqlite_type(self.to_string(), SqliteTypes::Blob)
                            .unwrap_or_else(|_| {
                                panic!("Could not add {} to types_for_database", self.to_string())
                            });
                    }
                    DataTypeCategory::NetworkAddress => {
                        types_for_database
                            .add_sqlite_type(self.to_string(), SqliteTypes::Text)
                            .unwrap_or_else(|_| {
                                panic!("Could not add {} to types_for_database", self.to_string())
                            });
                    }
                    DataTypeCategory::Json => {
                        types_for_database
                            .add_sqlite_type(self.to_string(), SqliteTypes::Text)
                            .unwrap_or_else(|_| {
                                panic!("Could not add {} to types_for_database", self.to_string())
                            });
                    }
                    DataTypeCategory::Search => todo!(),
                    DataTypeCategory::UUID => {
                        types_for_database
                            .add_sqlite_type(self.to_string(), SqliteTypes::Text)
                            .unwrap_or_else(|_| {
                                panic!("Could not add {} to types_for_database", self.to_string())
                            });
                    }
                    DataTypeCategory::Monetary => {
                        types_for_database
                            .add_sqlite_type(self.to_string(), SqliteTypes::Real)
                            .unwrap_or_else(|_| {
                                panic!("Could not add {} to types_for_database", self.to_string())
                            });
                    }
                    DataTypeCategory::BitString => {
                        types_for_database
                            .add_sqlite_type(self.to_string(), SqliteTypes::Text)
                            .unwrap_or_else(|_| {
                                panic!("Could not add {} to types_for_database", self.to_string())
                            });
                    }
                    DataTypeCategory::Interval => {
                        types_for_database
                            .add_sqlite_type(self.to_string(), SqliteTypes::Text)
                            .unwrap_or_else(|_| {
                                panic!("Could not add {} to types_for_database", self.to_string())
                            });
                    }
                    DataTypeCategory::Composite => {
                        types_for_database
                            .add_sqlite_type(self.to_string(), SqliteTypes::Text)
                            .unwrap_or_else(|_| {
                                panic!("Could not add {} to types_for_database", self.to_string())
                            });
                    }
                    DataTypeCategory::Range => {
                        types_for_database
                            .add_sqlite_type(self.to_string(), SqliteTypes::Text)
                            .unwrap_or_else(|_| {
                                panic!("Could not add {} to types_for_database", self.to_string())
                            });
                    }
                    DataTypeCategory::Other => todo!(),
                }

                types_for_database
            }
            DatabaseType::Mongo => todo!(),
        }
    }
}
