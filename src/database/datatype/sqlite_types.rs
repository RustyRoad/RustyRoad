use std::{cmp::Ordering, collections::HashMap};

use super::DataTypeCategory;
#[derive(Debug, Clone, PartialEq, std::cmp::Eq, Hash, PartialOrd, Ord)]
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
#[derive(Debug, Clone, PartialEq, std::cmp::Eq)]
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

impl Ord for SqliteTypeMap {
    fn cmp(&self, other: &Self) -> Ordering {
        self.types.len().cmp(&other.types.len())
    }
}

impl PartialOrd for SqliteTypeMap {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numeric_sql_types() {
        let expected_category = DataTypeCategory::Numeric;

        assert_eq!(SqliteTypes::Integer.category(), expected_category);
        assert_eq!(SqliteTypes::Real.category(), expected_category);
        assert_eq!(SqliteTypes::Numeric.category(), expected_category);
    }
}
