use super::{Balance, Names};
use crate::database::introspection::Schema;
use crate::generators::rust::repositories::equality;
use std::io;

/// Limits balances to textual polarity, small/integer amounts and timestamp expiry.
/// SUM over these integer inputs returns bigint without a lossy numeric cast.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn validate(
    schema: &Schema,
    names: &Names,
    name: &str,
    balance: &Balance,
) -> io::Result<()> {
    let invalid = |reason: &str| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Invalid balance {name:?}: {reason}"),
        )
    };
    if name.is_empty()
        || !name.starts_with(|c: char| c.is_ascii_lowercase())
        || !name
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
        || syn::parse_str::<syn::Ident>(name).is_err()
        || names.function_reserved(name)
    {
        return Err(invalid("invalid or reserved function name"));
    }
    let table = schema
        .table(&balance.table)
        .ok_or_else(|| invalid("table outside pull scope"))?;
    if equality::keys(table).is_empty() || balance.positive == balance.negative {
        return Err(invalid(
            "requires a typed filter and distinct polarity labels",
        ));
    }
    let polarity = table
        .column(&balance.polarity)
        .ok_or_else(|| invalid("missing polarity column"))?;
    let kind = polarity.sql_type.trim().to_ascii_lowercase();
    if kind.ends_with("[]")
        || !(matches!(
            kind.as_str(),
            "text" | "varchar" | "character varying" | "char" | "character" | "bpchar"
        ) || kind.starts_with("varchar(")
            || kind.starts_with("character varying("))
    {
        return Err(invalid("polarity must have a scalar text codec"));
    }
    let amount = table
        .column(&balance.amount)
        .ok_or_else(|| invalid("missing amount column"))?;
    if !matches!(
        amount.sql_type.trim().to_ascii_lowercase().as_str(),
        "smallint" | "int2" | "integer" | "int4" | "smallserial" | "serial"
    ) {
        return Err(invalid("amount must be smallint or integer"));
    }
    if let Some(expires) = &balance.expires {
        let column = table
            .column(expires)
            .ok_or_else(|| invalid("missing expiration column"))?;
        let kind = column.sql_type.trim().to_ascii_lowercase();
        if !(kind.starts_with("timestamp") || kind == "timestamptz") || kind.ends_with("[]") {
            return Err(invalid("expiration must be a scalar timestamp"));
        }
    }
    Ok(())
}
