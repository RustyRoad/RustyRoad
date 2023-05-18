use super::TypesForDatabase;
use crate::database::datatype::*;
use crate::DatabaseType;

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
#[derive(Debug, Clone, PartialEq, std::cmp::Eq)]
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

impl DataTypeCategory {
    pub fn get_data_types_from_data_type_category(
        &self,
        database_type: DatabaseType,
    ) -> TypesForDatabase {
        match database_type {
            // need to map the control flow to the database types
            DatabaseType::Postgres => {
                let mut types_for_database = TypesForDatabase::new();

                match &self {
                    DataTypeCategory::Boolean => {
                        types_for_database.add_postgres_type(postgres_types::PostgresTypes::Integer)
                    }
                    DataTypeCategory::Numeric => {
                        types_for_database
                            .add_postgres_type(postgres_types::PostgresTypes::SmallInt);
                        types_for_database
                            .add_postgres_type(postgres_types::PostgresTypes::Integer);
                        types_for_database.add_postgres_type(postgres_types::PostgresTypes::BigInt);
                        types_for_database
                            .add_postgres_type(postgres_types::PostgresTypes::Decimal);
                        types_for_database
                            .add_postgres_type(postgres_types::PostgresTypes::Numeric);
                        types_for_database.add_postgres_type(postgres_types::PostgresTypes::Real);
                        types_for_database
                            .add_postgres_type(postgres_types::PostgresTypes::DoublePrecision);
                        types_for_database
                            .add_postgres_type(postgres_types::PostgresTypes::SmallSerial);
                        types_for_database.add_postgres_type(postgres_types::PostgresTypes::Serial);
                        types_for_database
                            .add_postgres_type(postgres_types::PostgresTypes::BigSerial)
                    }
                    DataTypeCategory::DateTime => {
                        types_for_database.add_postgres_type(postgres_types::PostgresTypes::Date);
                        types_for_database.add_postgres_type(postgres_types::PostgresTypes::Time);
                        types_for_database
                            .add_postgres_type(postgres_types::PostgresTypes::Timestamp);
                        types_for_database
                            .add_postgres_type(postgres_types::PostgresTypes::TimestampWithTimeZone)
                    }
                    DataTypeCategory::Text => {
                        types_for_database.add_postgres_type(postgres_types::PostgresTypes::Char);
                        types_for_database
                            .add_postgres_type(postgres_types::PostgresTypes::Varchar);
                        types_for_database.add_postgres_type(postgres_types::PostgresTypes::Text)
                    }
                    DataTypeCategory::Geometric => {
                        types_for_database.add_postgres_type(postgres_types::PostgresTypes::Point);
                        types_for_database.add_postgres_type(postgres_types::PostgresTypes::Line);
                        types_for_database.add_postgres_type(postgres_types::PostgresTypes::Lseg);
                        types_for_database.add_postgres_type(postgres_types::PostgresTypes::Box);
                        types_for_database.add_postgres_type(postgres_types::PostgresTypes::Path);
                        types_for_database
                            .add_postgres_type(postgres_types::PostgresTypes::Polygon);
                        types_for_database.add_postgres_type(postgres_types::PostgresTypes::Circle)
                    }
                    DataTypeCategory::NetworkAddress => {
                        types_for_database.add_postgres_type(postgres_types::PostgresTypes::Cidr);
                        types_for_database.add_postgres_type(postgres_types::PostgresTypes::Inet);
                        types_for_database.add_postgres_type(postgres_types::PostgresTypes::Macaddr)
                    }
                    DataTypeCategory::Json => {
                        types_for_database.add_postgres_type(postgres_types::PostgresTypes::Json);
                        types_for_database.add_postgres_type(postgres_types::PostgresTypes::JsonB)
                    }
                    DataTypeCategory::Search => {
                        types_for_database
                            .add_postgres_type(postgres_types::PostgresTypes::TsVector);
                        types_for_database.add_postgres_type(postgres_types::PostgresTypes::TsQuery)
                    }
                    DataTypeCategory::Array => {
                        types_for_database.add_postgres_type(postgres_types::PostgresTypes::Array(
                            Box::new(postgres_types::PostgresTypes::Integer),
                        ));
                    }
                    DataTypeCategory::UUID => {
                        types_for_database.add_postgres_type(postgres_types::PostgresTypes::Uuid)
                    }
                    DataTypeCategory::Monetary => {
                        types_for_database.add_postgres_type(postgres_types::PostgresTypes::Money)
                    }
                    DataTypeCategory::BitString => {
                        types_for_database.add_postgres_type(postgres_types::PostgresTypes::Bit)
                    }
                    DataTypeCategory::Interval => types_for_database
                        .add_postgres_type(postgres_types::PostgresTypes::Interval),
                    DataTypeCategory::Range => {
                        types_for_database
                            .add_postgres_type(postgres_types::PostgresTypes::TsRange);
                        types_for_database
                            .add_postgres_type(postgres_types::PostgresTypes::NumRange);
                        types_for_database
                            .add_postgres_type(postgres_types::PostgresTypes::DateRange);
                        types_for_database
                            .add_postgres_type(postgres_types::PostgresTypes::Int4Range);
                        types_for_database
                            .add_postgres_type(postgres_types::PostgresTypes::Int8Range);
                        types_for_database
                            .add_postgres_type(postgres_types::PostgresTypes::TstzRange)
                    }
                }
                types_for_database
            }
            DatabaseType::Mysql => {
                let mut types_for_database = TypesForDatabase::new();

                match &self {
                    DataTypeCategory::Boolean => {
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::Boolean)
                    }
                    DataTypeCategory::Numeric => {
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::TinyInt);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::SmallInt);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::MediumInt);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::Int);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::BigInt);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::Decimal);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::Float);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::Double)
                    }
                    DataTypeCategory::DateTime => {
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::Date);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::DateTime);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::Timestamp);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::Time);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::Year)
                    }
                    DataTypeCategory::Text => {
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::Char);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::VarChar);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::TinyText);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::Text);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::MediumText);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::LongText);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::Binary);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::VarBinary);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::TinyBlob);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::Blob);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::MediumBlob);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::Enum);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::LongBlob)
                    }
                    DataTypeCategory::Geometric => {
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::Geometry);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::Point);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::LineString);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::Polygon);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::MultiPoint);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::MultiLineString);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::MultiPolygon);
                        types_for_database
                            .add_mysql_type(mysql_types::MySqlTypes::GeometryCollection)
                    }
                    DataTypeCategory::Json => {
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::Json)
                    }
                }
                types_for_database
            }
            DatabaseType::Sqlite => {
                let mut types_for_database = TypesForDatabase::new();

                match &self {
                    DataTypeCategory::Boolean => {
                        types_for_database.add_sqlite_type(sqlite_types::SqliteTypes::Boolean)
                    }
                    DataTypeCategory::Numeric => {
                        types_for_database.add_sqlite_type(sqlite_types::SqliteTypes::Integer);
                        types_for_database.add_sqlite_type(sqlite_types::SqliteTypes::Real);
                        types_for_database.add_sqlite_type(sqlite_types::SqliteTypes::Numeric)
                    }
                    DataTypeCategory::DateTime => {
                        types_for_database.add_sqlite_type(sqlite_types::SqliteTypes::Date);
                        types_for_database.add_sqlite_type(sqlite_types::SqliteTypes::DateTime);
                        types_for_database.add_sqlite_type(sqlite_types::SqliteTypes::Time)
                    }
                    DataTypeCategory::Text => {
                        types_for_database.add_sqlite_type(sqlite_types::SqliteTypes::Text);
                        types_for_database.add_sqlite_type(sqlite_types::SqliteTypes::Blob)
                    }
                    DataTypeCategory::Other => {
                        types_for_database.add_sqlite_type(sqlite_types::SqliteTypes::Null)
                    }
                }
                types_for_database
            }
            DatabaseType::Mongo => todo!(),
        }
    }
}
