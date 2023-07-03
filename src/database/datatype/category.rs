use super::TypesForDatabase;
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
/// use rustyroad::database::DataTypeCategory;
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
    Array,
    BitString,
    Boolean,
    Composite,
    DateTime,
    Geometric,
    Interval,
    Json,
    Money,
    NetworkAddress,
    Numeric,
    Other,
    Range,
    Search,
    Text,
    UUID,
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
            DataTypeCategory::Money => "money",
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
            DataTypeCategory::Money => write!(f, "Money"),
            DataTypeCategory::BitString => write!(f, "BitString"),
            DataTypeCategory::Interval => write!(f, "Interval"),
            DataTypeCategory::Composite => write!(f, "Composite"),
            DataTypeCategory::Range => write!(f, "Range"),
            DataTypeCategory::Other => write!(f, "Other"),
        }
    }
}

impl DataTypeCategory {
    /// Creates a new `DataTypeCategory` from a string representation.
    ///
    /// # Arguments
    ///
    /// * `category` - A string representing the data type category.
    ///
    /// # Returns
    ///
    /// An `Option` containing the corresponding `DataTypeCategory` if the string is valid, or `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustyroad::database::DataTypeCategory;
    /// let category = DataTypeCategory::new("Numeric");
    /// assert_eq!(category, Some(DataTypeCategory::Numeric));
    /// ```
    pub fn new(category: &str) -> Option<DataTypeCategory> {
        // Match the string representation of the category and return the corresponding DataTypeCategory.
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
            "Monetary" => Some(DataTypeCategory::Money),
            "BitString" => Some(DataTypeCategory::BitString),
            "Interval" => Some(DataTypeCategory::Interval),
            "Composite" => Some(DataTypeCategory::Composite),
            "Range" => Some(DataTypeCategory::Range),
            "Other" => Some(DataTypeCategory::Other),
            _ => None,
        }
    }

    /// Retrieves a vector of all `DataTypeCategory` values in a sorted order.
    ///
    /// # Returns
    ///
    /// A vector containing all `DataTypeCategory` values.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustyroad::database::DataTypeCategory;
    ///
    /// let categories = DataTypeCategory::get_all_categories();
    /// assert_eq!(categories.len(), 16);
    /// ```
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
            DataTypeCategory::Money,
            DataTypeCategory::BitString,
            DataTypeCategory::Interval,
            DataTypeCategory::Composite,
            DataTypeCategory::Range,
            DataTypeCategory::Other,
        ];

        categories.sort();

        categories
    }

    /// Retrieves the corresponding `TypesForDatabase` based on the `DataTypeCategory` and `DatabaseType`.
    ///
    /// # Arguments
    ///
    /// * `database_type` - The type of database.
    ///
    /// # Returns
    ///
    /// A `TypesForDatabase` struct containing the relevant data types for the specified `DataTypeCategory` and `DatabaseType`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustyroad::database::datatype::category::{DataTypeCategory, PostgresTypes, TypesForDatabase};
    /// use rustyroad::database::DatabaseType;
    ///
    /// let category = DataTypeCategory::Array;
    /// let database_type = DatabaseType::Postgres;
    /// let types_for_database = category.get_data_types_from_data_type_category(database_type);
    ///
    /// let expected_types = vec![PostgresTypes::Array(Box::new(PostgresTypes::Text))];
    /// let mut expected_types_for_database = TypesForDatabase::new();
    /// expected_types_for_database.add_postgres_type(category.to_string(), expected_types);
    ///
    /// assert_eq!(types_for_database, expected_types_for_database);
    /// ```
    pub fn get_data_types_from_data_type_category(
        &self,
        database_type: DatabaseType,
    ) -> TypesForDatabase {
        match database_type {
            DatabaseType::Postgres => {
                let  types_for_database = TypesForDatabase::new();

                // match self {
                //     DataTypeCategory::Array => types_for_database.add_postgres_type(
                //         self.to_string(),
                //         vec![
                //             PostgresTypes::Array(Box::new(PostgresTypes::Text)),
                //             PostgresTypes::Array(Box::new(PostgresTypes::Integer)),
                //             PostgresTypes::Array(Box::new(PostgresTypes::BigInt)),
                //         ]
                //     ),
                //     DataTypeCategory::Boolean => types_for_database
                //         .add_postgres_type(self.to_string(),
                //             vec![
                //                 PostgresTypes::Boolean,
                //                 PostgresTypes::Bit,
                //             ]
                //         ),
                //     DataTypeCategory::Numeric => {
                //         types_for_database
                //             .add_postgres_type(self.to_string(),
                //             vec![
                //                 PostgresTypes::Numeric,
                //                 PostgresTypes::Decimal,
                //                 PostgresTypes::Real,
                //                 PostgresTypes::DoublePrecision,
                //                 PostgresTypes::SmallInt,
                //                 PostgresTypes::Integer,
                //                 PostgresTypes::BigInt,
                //                 PostgresTypes::Serial,
                //                 PostgresTypes::BigSerial,
                //             ]
                //             );
                //         types_for_database
                //             .add_postgres_type(self.to_string(),
                //             vec![
                //                 PostgresTypes::Money,
                //             ]
                //             )
                //     }
                //     DataTypeCategory::DateTime => types_for_database
                //         .add_postgres_type(self.to_string(),
                //             vec![
                //                 PostgresTypes::Date,
                //                 PostgresTypes::Time,
                //                 PostgresTypes::Timestamp,
                //                 PostgresTypes::TimestampWithTimeZone,
                //                 PostgresTypes::Interval,
                //             ]
                //         ),
                //     DataTypeCategory::Text => {
                //         types_for_database.add_postgres_type(self.to_string(),
                //             vec![
                //                 PostgresTypes::Text,
                //                 PostgresTypes::Char,
                //                 PostgresTypes::VarChar,
                //                 PostgresTypes::Xml,
                //                 PostgresTypes::ByteA,
                //             ]
                //         )
                //     }
                //     DataTypeCategory::Geometric => {
                //         types_for_database.add_postgres_type(self.to_string(),
                //             vec![
                //                 PostgresTypes::Point,
                //                 PostgresTypes::Line,
                //                 PostgresTypes::Box,
                //                 PostgresTypes::Path,
                //                 PostgresTypes::Polygon,
                //                 PostgresTypes::Circle,
                //             ]
                //         )
                //     }
                //     DataTypeCategory::NetworkAddress => {
                //         types_for_database.add_postgres_type(self.to_string(),
                //         vec![
                //             PostgresTypes::Cidr,
                //             PostgresTypes::Inet,
                //             PostgresTypes::MacAddr,
                //             PostgresTypes::MacAddr8,
                //         ]
                //         )
                //     }
                //     DataTypeCategory::Json => {
                //         types_for_database.add_postgres_type(self.to_string(),
                //             vec![
                //                 PostgresTypes::Json,
                //                 PostgresTypes::JsonB,
                //             ]
                //         )
                //     }
                //     DataTypeCategory::Search => types_for_database
                //         .add_postgres_type(self.to_string(),
                //             vec![
                //                 PostgresTypes::TsVector,
                //                 PostgresTypes::TsQuery,
                //             ]
                //         ),
                //     DataTypeCategory::UUID => {
                //         types_for_database.add_postgres_type(self.to_string(),
                //             vec![
                //                 PostgresTypes::Uuid,
                //             ]
                //         )
                //     }
                //     DataTypeCategory::Money => {
                //         types_for_database.add_postgres_type(self.to_string(),
                //             vec![
                //                 PostgresTypes::Money,
                //             ]
                //         )
                //     }
                //     DataTypeCategory::BitString => {
                //         types_for_database.add_postgres_type(self.to_string(),
                //             vec![
                //                 PostgresTypes::Bit,
                //             ]
                //         )
                //     }
                //     DataTypeCategory::Interval => types_for_database
                //         .add_postgres_type(self.to_string(),
                //             vec![
                //                 PostgresTypes::Interval,
                //             ]
                //         ),
                //     DataTypeCategory::Composite => types_for_database
                //         .add_postgres_type(self.to_string(),
                //             vec![
                //                 PostgresTypes::Array(Box::new(PostgresTypes::Text)),
                //             ]
                //         ),
                //     DataTypeCategory::Range => types_for_database
                //         .add_postgres_type(self.to_string(),
                //             vec![
                //                 PostgresTypes::Int4Range,
                //                 PostgresTypes::Int8Range,
                //                 PostgresTypes::NumRange,
                //                 PostgresTypes::TsRange,
                //                 PostgresTypes::TstzRange,
                //                 PostgresTypes::DateRange,
                //             ]
                //         ),
                //     DataTypeCategory::Other => todo!(),
                // }

                types_for_database
            }
            DatabaseType::Mysql => {
                let types_for_database = TypesForDatabase::new();

                types_for_database
            }
            DatabaseType::Sqlite => {
                let types_for_database = TypesForDatabase::new();

                types_for_database
            },
            DatabaseType::Mongo => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_category_methods() {
        let database_type = DatabaseType::Postgres;
        let data_type_category = DataTypeCategory::Boolean;

        let types_for_database =
            data_type_category.get_data_types_from_data_type_category(database_type);

        // assert that categories isn't null
        assert!(!types_for_database.postgres.types.is_empty());

        let postgres_types = types_for_database.get_postgres_types(&data_type_category);

        assert_eq!(postgres_types, vec![PostgresTypes::Boolean]);

        //print them
        for i in postgres_types {
            println!("Postgres Type: {:?}", i);
        }
    }
}
