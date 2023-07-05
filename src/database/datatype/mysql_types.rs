use std::{cmp::Ordering, collections::HashMap};

use strum_macros::{EnumIter,Display};

use super::DataTypeCategory;
#[derive(
    Clone,
    Display,
    PartialEq,
    std::cmp::Eq,
    Hash,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
    EnumIter,
)]
pub enum MySqlTypes {
    Bit,
    Boolean,
    TinyInt,
    SmallInt,
    MediumInt,
    Int,
    BigInt,
    Float,
    Double,
    Decimal,
    Date,
    DateTime,
    Time,
    Timestamp,
    Year,
    Char,
    VarChar,
    Binary,
    VarBinary,
    TinyBlob,
    Blob,
    MediumBlob,
    LongBlob,
    TinyText,
    Text,
    MediumText,
    LongText,
    Enum,
    Set,
    Geometry,
    Point,
    LineString,
    Polygon,
    MultiPoint,
    MultiLineString,
    MultiPolygon,
    GeometryCollection,
    Json,
    BinaryJson,
    BitField,
    NewDecimal,
    EnumInner,
    SetInner,
    GeometryInner,
    /// Error type for when a type is not found in the database type map.
    NotFound,
}
#[derive(Debug, Clone, PartialEq, std::cmp::Eq)]
pub struct MySqlTypeMap {
    pub types: HashMap<String, Vec<MySqlTypes>>,
}

impl MySqlTypes {
    pub fn category(&self) -> DataTypeCategory {
        match self {
            MySqlTypes::Bit => DataTypeCategory::BitString,
            MySqlTypes::Boolean => DataTypeCategory::Numeric,
            MySqlTypes::TinyInt
            | MySqlTypes::SmallInt
            | MySqlTypes::MediumInt
            | MySqlTypes::Int
            | MySqlTypes::BigInt
            | MySqlTypes::Float
            | MySqlTypes::Double
            | MySqlTypes::Decimal
            | MySqlTypes::NewDecimal => DataTypeCategory::Numeric,
            MySqlTypes::Date
            | MySqlTypes::DateTime
            | MySqlTypes::Time
            | MySqlTypes::Timestamp
            | MySqlTypes::Year => DataTypeCategory::DateTime,
            MySqlTypes::Char
            | MySqlTypes::VarChar
            | MySqlTypes::Binary
            | MySqlTypes::VarBinary
            | MySqlTypes::TinyBlob
            | MySqlTypes::Blob
            | MySqlTypes::MediumBlob
            | MySqlTypes::LongBlob
            | MySqlTypes::TinyText
            | MySqlTypes::Text
            | MySqlTypes::MediumText
            | MySqlTypes::LongText
            | MySqlTypes::Enum
            | MySqlTypes::Set
            | MySqlTypes::Json
            | MySqlTypes::BinaryJson => DataTypeCategory::Text,
            MySqlTypes::Geometry
            | MySqlTypes::Point
            | MySqlTypes::LineString
            | MySqlTypes::Polygon
            | MySqlTypes::MultiPoint
            | MySqlTypes::MultiLineString
            | MySqlTypes::MultiPolygon
            | MySqlTypes::GeometryCollection => DataTypeCategory::Geometric,
            MySqlTypes::BitField => DataTypeCategory::Other,
            MySqlTypes::EnumInner => DataTypeCategory::Other,
            MySqlTypes::SetInner => DataTypeCategory::Other,
            MySqlTypes::GeometryInner => DataTypeCategory::Other,
            MySqlTypes::NotFound => DataTypeCategory::Other,
        }
    }
}

impl Ord for MySqlTypeMap {
    fn cmp(&self, other: &Self) -> Ordering {
        self.types.len().cmp(&other.types.len())
    }
}

impl PartialOrd for MySqlTypeMap {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
