use std::collections::HashMap;

use super::DataTypeCategory;

pub enum SqliteTypes {
    Integer,
    Real,
    Text,
    Blob,
    Numeric,
    Date,
    Time,
    DateTime,
    Boolean,
    Unknown,
}

pub struct SqliteTypeMap {
    pub types: HashMap<String, SqliteTypes>,
}

impl SqliteTypes {
    pub fn category(&self) -> DataTypeCategory {
        match self {
            SqliteType::Integer | SqliteType::Real | SqliteType::Numeric => {
                DataTypeCategory::Numeric
            }
            SqliteType::Date | SqliteType::Time | SqliteType::DateTime => {
                DataTypeCategory::DateTime
            }
            SqliteType::Text => DataTypeCategory::Text,
            SqliteType::Blob => DataTypeCategory::Other,
            SqliteType::Boolean => DataTypeCategory::Other,
            SqliteType::Unknown => DataTypeCategory::Other,
        }
    }
}
