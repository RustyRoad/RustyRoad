use std::collections::HashMap;

use super::DataTypeCategory;
#[derive(Debug, Clone, PartialEq, std::cmp::Eq, Hash)]
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
    pub types: HashMap<String, Vec<SqliteTypes>>,
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
            SqliteTypes::Boolean => DataTypeCategory::Other,
            SqliteTypes::Null => todo!(),
        }
    }
}
