use crate::database::DatabaseType;
use sqlparser::ast::{ColumnDef, ColumnOption};

pub(super) fn required(column: &ColumnDef, database: &DatabaseType) -> bool {
    column
        .options
        .iter()
        .any(|option| match (&option.option, database) {
            (
                ColumnOption::Generated {
                    generation_expr: None,
                    ..
                },
                DatabaseType::Postgres,
            ) => true,
            (ColumnOption::DialectSpecific(tokens), DatabaseType::Mysql) => tokens
                .iter()
                .any(|token| token.to_string().eq_ignore_ascii_case("AUTO_INCREMENT")),
            _ => false,
        })
}
