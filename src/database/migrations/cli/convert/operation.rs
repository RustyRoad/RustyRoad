use super::super::super::SqlOperation;

pub(super) fn print(operation: &SqlOperation) {
    match operation {
        SqlOperation::CreateTable { table_name, .. } => {
            println!("       - CREATE TABLE {table_name}")
        }
        SqlOperation::DropTable { table_name } => println!("       - DROP TABLE {table_name}"),
        SqlOperation::AlterTableAddColumn {
            table_name,
            columns,
            ..
        } => println!(
            "       - ALTER TABLE {} ADD COLUMN {}",
            table_name,
            columns.join(", ")
        ),
        SqlOperation::AlterTableDropColumn {
            table_name,
            columns,
            ..
        } => println!(
            "       - ALTER TABLE {} DROP COLUMN {}",
            table_name,
            columns.join(", ")
        ),
        SqlOperation::CreateIndex {
            index_name,
            table_name,
            ..
        } => {
            println!("       - CREATE INDEX {index_name} ON {table_name}")
        }
        SqlOperation::DropIndex { index_name } => println!("       - DROP INDEX {index_name}"),
        SqlOperation::RawSql { .. } => println!("       - [Raw SQL statement]"),
    }
}
