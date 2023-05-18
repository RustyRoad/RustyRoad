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
    Null,
}

pub struct SqliteTypeMap {
    pub types: HashMap<String, SqliteTypes>,
}

impl SqliteTypes {
    pub fn category(&self) -> DataTypeCategory {
        match &self {
            SqliteTypes::Integer | SqliteTypes::Real | SqliteTypes::Numeric => {
                DataTypeCategory::Numeric
            }
            SqliteTypes::Date | SqliteTypes::Time | SqliteTypes::DateTime => {
                DataTypeCategory::DateTime
            }
            SqliteTypes::Text => DataTypeCategory::Text,
            SqliteTypes::Blob => DataTypeCategory::Other,
            SqliteTypes::Boolean => DataTypeCategory::Other
        }
    }
}
