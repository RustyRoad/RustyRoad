use std::collections::HashMap;

use super::DataTypeCategory;

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
}

pub struct MySqlTypeMap {
    pub types: HashMap<String, MySqlTypes>,
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
            MySqlTypes::GeometryInner => DataTypeCategory::Other
        }
    }
}
