use crate::DatabaseType;
use crate::database::datatype::*;
use super::TypesForDatabase;

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
            DatabaseType::Mysql => {
                let mut types_for_database = TypesForDatabase::new();
                match &self {
                    DataTypeCategory::Boolean => types_for_database
                        .add_mysql_type(mysql_types::MySqlTypes::TinyInt),
                                            DataTypeCategory::Numeric => {
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::TinyInt);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::SmallInt);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::MediumInt);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::Int);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::BigInt);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::Decimal);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::Float);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::Double);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::Bit)
                    },
                    DataTypeCategory::DateTime => {
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::Date);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::DateTime);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::Time);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::Timestamp);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::Year)
                    },
                    DataTypeCategory::Array => {
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::Char);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::VarChar);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::Binary);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::VarBinary);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::TinyBlob);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::Blob);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::MediumBlob);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::LongBlob);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::TinyText);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::Text);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::MediumText);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::LongText);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::Enum);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::Set);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::Json)
                    },
                    DataTypeCategory::UUID => {
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::Uuid)
                    },
                    DataTypeCategory::Monetary => {
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::Decimal);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::Float);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::Double)
                    },
                    DataTypeCategory::BitString => {
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::Bit);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::Char);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::VarChar)
                    },
                    DataTypeCategory::NetworkAddress => {
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::Inet);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::Cidr);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::MacAddr)
                    },
                    DataTypeCategory::Numeric => {
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::TinyInt);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::SmallInt);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::MediumInt);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::Int);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::BigInt);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::Decimal);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::Float);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::Double);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::Bit)
                    },
                    DataTypeCategory::Geometric => {
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::Point);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::LineString);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::Polygon);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::Geometry);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::MultiPoint);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::MultiLineString);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::MultiPolygon);
                        types_for_database.add_mysql_type(mysql_types::MySqlTypes::GeometryCollection)
                    },
                }
            },
            DatabaseType::Postgres => {
                let mut types_for_database = TypesForDatabase::new();

                match &self {
                    DataTypeCategory::Boolean => {
                        types_for_database.add_postgres_type(postgres_types::PostgresTypes::Integer)
                    },
                    DataTypeCategory::Numeric => {
                        types_for_database.add_postgres_type(postgres_types::PostgresTypes::SmallInt);
                        types_for_database.add_postgres_type(postgres_types::PostgresTypes::Integer);
                        types_for_database.add_postgres_type(postgres_types::PostgresTypes::BigInt);
                        types_for_database.add_postgres_type(postgres_types::PostgresTypes::Decimal);
                        types_for_database.add_postgres_type(postgres_types::PostgresTypes::Numeric);
                        types_for_database.add_postgres_type(postgres_types::PostgresTypes::Real);
                        types_for_database.add_postgres_type(postgres_types::PostgresTypes::DoublePrecision);
                        types_for_database.add_postgres_type(postgres_types::PostgresTypes::SmallSerial);
                        types_for_database.add_postgres_type(postgres_types::PostgresTypes::Serial);
                        types_for_database.add_postgres_type(postgres_types::PostgresTypes::BigSerial)
                    },
                    DataTypeCategory::DateTime => {
                        types_for_database.add_postgres_type(postgres_types::PostgresTypes::Date);
                        types_for_database.add_postgres_type(postgres_types::PostgresTypes::Time);
                        types_for_database.add_postgres_type(postgres_types::PostgresTypes::Timestamp);
                        types_for_database.add_postgres_type(postgres_types::PostgresTypes::TimestampWithTimeZone);
                        types_for_database.add_postgres_type(postgres_types::PostgresTypes::Interval)
                    },
                    DataTypeCategory::Text => {
                        types_for_database.add_postgres_type(postgres_types::PostgresTypes::Char);
                        types_for_database.add_postgres_type(postgres_types::PostgresTypes::Varchar);
                        types_for_database.add_postgres_type(postgres_types::PostgresTypes::Text);
                        types_for_database.add_postgres_type(postgres_types::PostgresTypes::Name);
                        types_for_database.add_postgres_type(postgres_types::PostgresTypes::Cidr);
                        types_for_database.add_postgres_type(postgres_types::PostgresTypes::Inet);
                        types_for_database.add_postgres_type(postgres_types::PostgresTypes::MacAddr);
                        types_for_database.add_postgres_type(postgres_types::PostgresTypes::Json);
                        types_for_database.add_postgres_type(postgres_types::PostgresTypes::JsonB);
                        types_for_database.add_postgres_type(postgres_types::PostgresTypes::Xml);
                        types_for_database.add_postgres_type(postgres_types::PostgresTypes::Uuid);
                        types_for_database.add_postgres_type(postgres_types::PostgresTypes::TsVector);
                        types_for_database.add_postgres_type(postgres_types::PostgresTypes::TsQuery);
                        types_for_database.add_postgres_type(postgres_types::PostgresTypes::SearchVector)
                    },
                    DataTypeCategory::Geometric => todo!(),
                    DataTypeCategory::NetworkAddress => todo!(),
                    DataTypeCategory::Json => todo!(),
                    DataTypeCategory::Search => todo!(),
                    DataTypeCategory::Array => todo!(),
                    DataTypeCategory::UUID => todo!(),
                    DataTypeCategory::Monetary => todo!(),
                    DataTypeCategory::BitString => todo!(),
                    DataTypeCategory::Interval => todo!(),
                    DataTypeCategory::Composite => todo!(),
                    DataTypeCategory::Range => todo!(),
                    DataTypeCategory::Other => todo!(),
                }
            }
        }
    }
}
