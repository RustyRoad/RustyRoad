use std::{cmp::Ordering, collections::HashMap};

use super::DataTypeCategory;

/// The PostgreSQL data types are used to define a type of a column of a table.
/// In addition, a column can be defined as a computed column, using an expression
/// that evaluates to a value of scalar type.
/// - https://www.postgresql.org/docs/12/datatype.html
#[derive(Debug, Clone, PartialEq, std::cmp::Eq, Hash, PartialOrd, Ord)]
pub enum PostgresTypes {
    /// A 2 byte signed integer.
    /// - Range: -32768 to +32767
    /// - Storage Size: 2 bytes
    /// - Category: Numeric
    /// - SQL Type: SMALLINT
    /// - Alias: INT2
    /// - Note: The smallint type is generally only used if disk space is at a premium.
    /// Otherwise, integer should be used.
    /// - https://www.postgresql.org/docs/12/datatype-numeric.html#DATATYPE-SMALLINT
    SmallInt,
    /// A 4 byte signed integer.
    /// - Range: -2147483648 to +2147483647
    /// - Storage Size: 4 bytes
    /// - Category: Numeric
    /// - SQL Type: INTEGER
    /// - Alias: INT, INT4
    /// - Note: The integer type is generally the default choice when you need to store a number.
    /// - https://www.postgresql.org/docs/12/datatype-numeric.html#DATATYPE-INTEGER
    Integer,
    /// An 8 byte signed integer.
    /// - Range: -9223372036854775808 to +9223372036854775807
    /// - Storage Size: 8 bytes
    /// - Category: Numeric
    /// - SQL Type: BIGINT
    /// - Alias: INT8
    /// - Note: The bigint type should be used if you need to store numbers outside the range of the integer type.
    /// - https://www.postgresql.org/docs/12/datatype-numeric.html#DATATYPE-BIGINT
    BigInt,
    /// A fixed precision number.
    /// - Range: -10^38 +1 to 10^38 -1
    /// - Storage Size: 4 bytes
    /// - Category: Numeric
    /// - SQL Type: DECIMAL
    /// - Alias: DEC, NUMERIC, FIXED
    /// - Note: The precision, p, can be from 1 to 38. The scale, s, can be from 0 to p.
    /// - https://www.postgresql.org/docs/12/datatype-numeric.html#DATATYPE-DECIMAL
    Decimal,
    /// A 4 byte floating point number.
    /// - Range: 6 decimal digits precision
    /// - Storage Size: 4 bytes
    /// - Category: Numeric
    /// - SQL Type: REAL
    /// - Alias: FLOAT4
    /// - Note: The real type typically has a range of around 6 decimal digits of precision.
    /// - https://www.postgresql.org/docs/12/datatype-numeric.html#DATATYPE-FLOAT
    Real,
    /// An 8 byte floating point number.
    /// - Range: 15 decimal digits precision
    /// - Storage Size: 8 bytes
    /// - Category: Numeric
    /// - SQL Type: DOUBLE PRECISION
    /// - Alias: FLOAT8
    /// - Note: The double precision type typically has a range of around 15 decimal digits of precision.
    /// - https://www.postgresql.org/docs/12/datatype-numeric.html#DATATYPE-FLOAT
    DoublePrecision,
    /// A variable precision number.
    /// - Range: 15 decimal digits precision
    /// - Storage Size: 8 bytes
    /// - Category: Numeric
    /// - SQL Type: NUMERIC
    /// - Alias: NUMERIC, DEC, DECIMAL, FIXED
    /// - Note: The numeric type can store numbers with very large numbers of digits.
    /// It is especially recommended for storing monetary amounts and other quantities where exactness is required.
    /// - https://www.postgresql.org/docs/12/datatype-numeric.html#DATATYPE-NUMERIC
    Numeric,
    /// A 1 byte signed integer.
    /// - Range: -128 to +127
    /// - Storage Size: 1 byte
    /// - Category: Numeric
    /// - SQL Type: SMALLINT
    /// - Alias: INT2
    /// - Note: The smallserial type is generally only used if disk space is at a premium.
    /// Otherwise, serial should be used.
    /// - https://www.postgresql.org/docs/12/datatype-numeric.html#DATATYPE-SERIAL
    SmallSerial,
    /// A 4 byte signed integer.
    /// - Range: 1 to 2147483647
    /// - Storage Size: 4 bytes
    /// - Category: Numeric
    /// - SQL Type: INTEGER
    /// - Alias: INT, INT4
    /// - Note: The serial type is generally the default choice when you need to store a number.
    /// - https://www.postgresql.org/docs/12/datatype-numeric.html#DATATYPE-SERIAL
    Serial,
    /// An 8 byte signed integer.
    /// - Range: 1 to 9223372036854775807
    /// - Storage Size: 8 bytes
    /// - Category: Numeric
    /// - SQL Type: BIGINT
    /// - Alias: INT8
    /// - Note: The bigserial type should be used if you need to store numbers outside the range of the integer type.
    /// - https://www.postgresql.org/docs/12/datatype-numeric.html#DATATYPE-SERIAL
    BigSerial,
    /// A 8 byte currency amount.
    /// - Range: -92233720368547758.08 to +92233720368547758.07
    /// - Storage Size: 8 bytes
    /// - Category: Monetary
    /// - SQL Type: MONEY
    /// - Alias: -
    /// - Note: The money type stores a currency amount with a fixed fractional precision.
    /// - https://www.postgresql.org/docs/12/datatype-money.html
    /// - https://www.postgresql.org/docs/12/datatype-numeric.html#DATATYPE-MONEY
    Money,
    /// A variable length character string.
    /// - Range: 1 to 10485760
    /// - Storage Size: 1 byte + the actual string
    /// - Category: Character
    /// - SQL Type: VARCHAR
    /// - Alias: CHAR VARYING
    /// - Note: The varchar type is used when you want to store a string that can be up to 10485760 bytes long.
    /// - https://www.postgresql.org/docs/12/datatype-character.html#DATATYPE-VARCHAR
    Varchar,
    /// A variable length character string.
    /// - Range: 1 to 10485760
    /// - Storage Size: 1 byte + the actual string
    /// - Category: Character
    /// - SQL Type: VARCHAR
    /// - Alias: CHAR VARYING
    /// - Note: The varchar type is used when you want to store a string that can be up to 10485760 bytes long.
    /// - https://www.postgresql.org/docs/12/datatype-character.html#DATATYPE-VARCHAR
    CharVarying,
    /// A variable length character string.
    /// - Range: 1 to 10485760
    /// - Storage Size: 1 byte + the actual string
    /// - Category: Character
    /// - SQL Type: VARCHAR
    /// - Alias: CHAR VARYING
    /// - Note: The varchar type is used when you want to store a string that can be up to 10485760 bytes long.
    /// - https://www.postgresql.org/docs/12/datatype-character.html#DATATYPE-VARCHAR
    CharacterVarying,
    /// A fixed length character string.
    /// - Range: 1 to 10485760
    /// - Storage Size: 1 byte + the actual string
    /// - Category: Character
    /// - SQL Type: CHAR
    /// - Alias: CHARACTER
    /// - Note: The char type is used when you want to store a string that is exactly n characters long.
    /// - https://www.postgresql.org/docs/12/datatype-character.html#DATATYPE-CHAR
    Char,
    /// A fixed length character string.
    /// - Range: 1 to 10485760
    /// - Storage Size: 1 byte + the actual string
    /// - Category: Character
    /// - SQL Type: CHAR
    /// - Alias: CHARACTER
    /// - Note: The char type is used when you want to store a string that is exactly n characters long.
    /// - https://www.postgresql.org/docs/12/datatype-character.html#DATATYPE-CHAR
    Character,
    /// An unlimited length character string.
    /// - Range: 1 to 1073741824
    /// - Storage Size: 4 bytes + the actual string
    /// - Category: Character
    /// - SQL Type: TEXT
    /// - Alias: -
    /// - Note: The text type is used when you want to store a string with no limit on its length.
    /// - https://www.postgresql.org/docs/12/datatype-character.html#DATATYPE-TEXT
    Text,
    /// A variable length binary string.
    /// - Range: 1 to 10485760
    /// - Storage Size: 4 bytes + the actual string
    /// - Category: Binary
    /// - SQL Type: BYTEA
    /// - Alias: -
    /// - Note: The bytea type is used when you want to store a string of bytes.
    /// - https://www.postgresql.org/docs/12/datatype-binary.html#DATATYPE-BYTEA
    Bytea,
    /// Both date and time (no time zone).
    /// - Range: 4713 BC to 294276 AD
    /// - Storage Size: 8 bytes
    /// - Category: Date/Time
    /// - SQL Type: TIMESTAMP
    /// - Alias: -
    /// - Note: The timestamp type is used when you want to store a date and time.
    /// - https://www.postgresql.org/docs/12/datatype-datetime.html#DATATYPE-TIMESTAMP
    Timestamp,
    /// Both date and time (no time zone).
    /// - Range: 4713 BC to 294276 AD
    /// - Storage Size: 8 bytes
    /// - Category: Date/Time
    /// - SQL Type: TIMESTAMP
    /// - Alias: -
    /// - Note: The timestamp type is used when you want to store a date and time.
    /// - https://www.postgresql.org/docs/12/datatype-datetime.html#DATATYPE-TIMESTAMP
    TimestampWithoutTimeZone,
    /// Both date and time (with time zone).
    /// - Range: 4713 BC to 294276 AD
    /// - Storage Size: 8 bytes
    /// - Category: Date/Time
    /// - SQL Type: TIMESTAMP WITH TIME ZONE
    /// - Alias: TIMESTAMPTZ
    /// - Note: The timestamptz type is used when you want to store a date and time with timezone information.
    /// - https://www.postgresql.org/docs/12/datatype-datetime.html#DATATYPE-TIMESTAMP-TIMEZONE
    TimestampWithTimeZone,
    /// Date (no time of day).
    /// - Range: 4713 BC to 5874897 AD
    /// - Storage Size: 4 bytes
    /// - Category: Date/Time
    /// - SQL Type: DATE
    /// - Alias: -
    /// - Note: The date type is used when you want to store a date only.
    /// - https://www.postgresql.org/docs/12/datatype-datetime.html#DATATYPE-DATE
    Date,
    /// Time of day (no date).
    /// - Range: 00:00:00 to 24:00:00
    /// - Storage Size: 8 bytes
    /// - Category: Date/Time
    /// - SQL Type: TIME
    /// - Alias: -
    /// - Note: The time type is used when you want to store a time of day only.
    /// - https://www.postgresql.org/docs/12/datatype-datetime.html#DATATYPE-TIME
    Time,
    /// Time of day (no date).
    /// - Range: 00:00:00 to 24:00:00
    /// - Storage Size: 8 bytes
    /// - Category: Date/Time
    /// - SQL Type: TIME
    /// - Alias: -
    /// - Note: The time type is used when you want to store a time of day only.
    /// - https://www.postgresql.org/docs/12/datatype-datetime.html#DATATYPE-TIME
    TimeWithoutTimeZone,
    /// Time of day (with time zone).
    /// - Range: 00:00:00+1459 to 24:00:00-1459
    /// - Storage Size: 12 bytes
    /// - Category: Date/Time
    /// - SQL Type: TIME WITH TIME ZONE
    /// - Alias: TIMETZ
    /// - Note: The timetz type is used when you want to store a time of day with timezone information.
    /// - https://www.postgresql.org/docs/12/datatype-datetime.html#DATATYPE-TIME-TIMEZONE
    TimeWithTimeZone,
    /// A time span.
    /// - Range: -178000000 years to 178000000 years
    /// - Storage Size: 16 bytes
    /// - Category: Date/Time
    /// - SQL Type: INTERVAL
    /// - Alias: -
    /// - Note: The interval type is used when you want to store a time span.
    /// - https://www.postgresql.org/docs/12/datatype-datetime.html#DATATYPE-INTERVAL-INPUT
    Interval,
    /// State of true or false.
    /// - Range: true or false
    /// - Storage Size: 1 byte
    /// - Category: Boolean
    /// - SQL Type: BOOLEAN
    /// - Alias: BOOL
    /// - Note: The boolean type is used when you want to store a state of true or false.
    /// - https://www.postgresql.org/docs/12/datatype-boolean.html#DATATYPE-BOOLEAN-BOOL
    Boolean,
    /// Enumerated (enum) types are data types that comprise a static, ordered set of values.
    /// - Range: -
    /// - Storage Size: 4 bytes
    /// - Category: Enum
    /// - SQL Type: ENUM
    /// - Alias: -
    /// - Note: The enum type is used when you want to store a static, ordered set of values.
    /// - https://www.postgresql.org/docs/12/datatype-enum.html#DATATYPE-ENUM
    Enum,
    /// A point on a plane.
    /// - Range: -
    /// - Storage Size: 16 bytes
    /// - Category: Geometric
    /// - SQL Type: POINT
    /// - Alias: -
    /// - Note: The point type is used when you want to store a point on a plane.
    /// - https://www.postgresql.org/docs/12/datatype-geometric.html#DATATYPE-POINT
    Point,
    /// Infinite line.
    /// - Range: -
    /// - Storage Size: 32 bytes
    /// - Category: Geometric
    /// - SQL Type: LINE
    /// - Alias: -
    /// - Note: The line type is used when you want to store an infinite line.
    /// - https://www.postgresql.org/docs/12/datatype-geometric.html#DATATYPE-LINE
    Line,
    /// Finite line segment.
    /// - Range: -
    /// - Storage Size: 32 bytes
    /// - Category: Geometric
    /// - SQL Type: LSEG
    /// - Alias: -
    /// - Note: The lseg type is used when you want to store a finite line segment.
    /// - https://www.postgresql.org/docs/12/datatype-geometric.html#DATATYPE-LSEG
    Lseg,
    /// Rectangular box on a plane.
    /// - Range: -
    /// - Storage Size: 32 bytes
    /// - Category: Geometric
    /// - SQL Type: BOX
    /// - Alias: -
    /// - Note: The box type is used when you want to store a rectangular box on a plane.
    /// - https://www.postgresql.org/docs/12/datatype-geometric.html#DATATYPE-BOX
    Box,
    /// Closed path on a plane.
    /// - Range: -
    /// - Storage Size: 16+16n bytes
    /// - Category: Geometric
    /// - SQL Type: PATH
    /// - Alias: -
    /// - Note: The path type is used when you want to store a closed path on a plane.
    /// - https://www.postgresql.org/docs/12/datatype-geometric.html#DATATYPE-PATH
    Path,
    /// Open path on a plane.
    /// - Range: -
    /// - Storage Size: 16+16n bytes
    /// - Category: Geometric
    /// - SQL Type: PATH
    /// - Alias: -
    /// - Note: The path type is used when you want to store a open path on a plane.
    /// - https://www.postgresql.org/docs/12/datatype-geometric.html#DATATYPE-PATH
    PathOpen,
    /// Polygon on a plane.
    /// - Range: -
    /// - Storage Size: 40+16n bytes
    /// - Category: Geometric
    /// - SQL Type: POLYGON
    /// - Alias: -
    /// - Note: The polygon type is used when you want to store a polygon on a plane.
    /// - https://www.postgresql.org/docs/12/datatype-geometric.html#DATATYPE-POLYGON
    Polygon,
    /// Circle on a plane.
    /// - Range: -
    /// - Storage Size: 24 bytes
    /// - Category: Geometric
    /// - SQL Type: CIRCLE
    /// - Alias: -
    /// - Note: The circle type is used when you want to store a circle on a plane.
    /// - https://www.postgresql.org/docs/12/datatype-geometric.html#DATATYPE-CIRCLE
    Circle,
    /// IPv4 or IPv6 host address.
    /// - Range: -
    /// - Storage Size: 12 bytes
    /// - Category: Network Address
    /// - SQL Type: INET
    /// - Alias: -
    /// - Note: The inet type is used when you want to store an IPv4 or IPv6 host address.
    /// - https://www.postgresql.org/docs/12/datatype-net-types.html#DATATYPE-INET
    Inet,
    /// IPv4 or IPv6 host address (without netmask).
    /// - Range: -
    /// - Storage Size: 12 bytes
    /// - Category: Network Address
    /// - SQL Type: CIDR
    /// - Alias: -
    /// - Note: The cidr type is used when you want to store an IPv4 or IPv6 host address (without netmask).
    /// - https://www.postgresql.org/docs/12/datatype-net-types.html#DATATYPE-CIDR
    Cidr,
    /// The essential difference between the cidr and inet types is that the former stores an address/netmask pair per value, and the latter stores a single address from the pair.
    /// - Range: -
    /// - Storage Size: 24 bytes
    /// - Category: Network Address
    /// - SQL Type: MACADDR
    /// - Alias: -
    /// - Note: The macaddr type is used when you want to store a MAC address.
    /// - https://www.postgresql.org/docs/12/datatype-net-types.html#DATATYPE-MACADDR
    Macaddr,
    /// The macaddr8 type is used when you want to store a MAC address (EUI-64 format).
    /// - Range: -
    /// - Storage Size: 8 bytes
    /// - Category: Network Address
    /// - SQL Type: MACADDR8
    /// - Alias: -
    /// - Note: The macaddr8 type is used when you want to store a MAC address (EUI-64 format).
    /// - https://www.postgresql.org/docs/12/datatype-net-types.html#DATATYPE-MACADDR8
    Macaddr8,
    /// Bit strings are strings of 1's and 0's.
    /// - Range: -
    /// - Storage Size: 1 or 4 bytes + 1 byte for each 8 bits
    /// - Category: Bit String
    /// - SQL Type: BIT
    /// - Alias: -
    /// - Note: The bit type is used when you want to store a bit string.
    /// - https://www.postgresql.org/docs/12/datatype-bit.html#DATATYPE-BIT
    Bit,
    /// Bit strings are strings of 1's and 0's.
    /// - Range: -
    /// - Storage Size: 1 or 4 bytes + 1 byte for each 8 bits
    /// - Category: Bit String
    /// - SQL Type: BIT VARYING
    /// - Alias: VARBIT
    /// - Note: The bit varying type is used when you want to store a bit string with a length limit.
    /// - https://www.postgresql.org/docs/12/datatype-bit.html#DATATYPE-BIT-VARYING
    BitVarying,
    /// Text search document.
    /// - Range: -
    /// - Storage Size: -
    /// - Category: Text Search
    /// - SQL Type: TSVECTOR
    /// - Alias: -
    /// - Note: The tsvector type is used to store a document in a format optimized for text search.
    /// - https://www.postgresql.org/docs/12/datatype-textsearch.html#DATATYPE-TSVECTOR
    TsVector,
    /// Text search query.
    /// - Range: -
    /// - Storage Size: -
    /// - Category: Text Search
    /// - SQL Type: TSQUERY
    /// - Alias: -
    /// - Note: The tsquery type is used to store a text search query.
    /// - https://www.postgresql.org/docs/12/datatype-textsearch.html#DATATYPE-TSQUERY
    TsQuery,
    /// XML data.
    /// - Range: -
    /// - Storage Size: 4 bytes + the actual binary string
    /// - Category: XML
    /// - SQL Type: XML
    /// - Alias: -
    /// - Note: The xml type is used to store XML data.
    /// - https://www.postgresql.org/docs/12/datatype-xml.html#DATATYPE-XML
    Xml,
    /// JSON data.
    /// - Range: -
    /// - Storage Size: 1 byte + the actual binary string
    /// - Category: JSON
    /// - SQL Type: JSON
    /// - Alias: -
    /// - Note: The json type stores an exact copy of the input text, which processing functions must reparse on each execution; while jsonb data is stored in a decomposed binary format that makes it slightly slower to input due to added conversion overhead, but significantly faster to process, since no reparsing is needed. jsonb also supports indexing, which can be a significant advantage.
    /// - https://www.postgresql.org/docs/12/datatype-json.html#DATATYPE-JSON
    Json,
    /// Binary JSON data, decomposed.
    /// - Range: -
    /// - Storage Size: 1 byte + the actual binary string
    /// - Category: JSON
    /// - SQL Type: JSONB
    /// - Alias: -
    /// - Note: The json type stores an exact copy of the input text, which processing functions must reparse on each execution; while jsonb data is stored in a decomposed binary format that makes it slightly slower to input due to added conversion overhead, but significantly faster to process, since no reparsing is needed. jsonb also supports indexing, which can be a significant advantage.
    /// - https://www.postgresql.org/docs/12/datatype-json.html#DATATYPE-JSONB
    JsonB,
    /// UUID datatype.
    /// - Range: 0 to 2^128-1
    /// - Storage Size: 16 bytes
    /// - Category: UUID
    /// - SQL Type: UUID
    /// - Alias: -
    /// - Note: The uuid type stores Universally Unique Identifiers (UUID) as defined by RFC 4122, ISO/IEC 9834-8:2005, and related standards. (A Universally Unique Identifier (UUID) URN Namespace, P. Leach, M. Mealling, R. Salz, December 2005.)
    /// - https://www.postgresql.org/docs/12/datatype-uuid.html#DATATYPE-UUID
    Uuid,
    /// The pg_lsn type is used to store LSN (Log Sequence Number) values, as used in WAL (Write-Ahead Log) records.
    /// - Range: 0 to 2^64-1
    /// - Storage Size: 8 bytes
    /// - Category: LSN
    /// - SQL Type: PG_LSN
    /// - Alias: -
    /// - Note: The pg_lsn type is used to store LSN (Log Sequence Number) values, as used in WAL (Write-Ahead Log) records.
    /// - https://www.postgresql.org/docs/12/datatype-pg-lsn.html#DATATYPE-PG-LSN
    PgLsn,
    /// The pg_snapshot type is used to store snapshot information for use by the txid_current_snapshot() function.
    /// - Range: -
    /// - Storage Size: 8 bytes
    /// - Category: Snapshot
    /// - SQL Type: PG_SNAPSHOT
    /// - Alias: -
    /// - Note: The pg_snapshot type is used to store snapshot information for use by the txid_current_snapshot() function.
    /// - https://www.postgresql.org/docs/12/datatype-pg-snapshot.html#DATATYPE-PG-SNAPSHOT
    PgSnapshot,
    /// The txid_snapshot type is used to store transaction snapshot information for use by the txid_snapshot_in() and txid_snapshot_out() functions.
    /// - Range: -
    /// - Storage Size: 8 bytes
    /// - Category: Snapshot
    /// - SQL Type: TXID_SNAPSHOT
    /// - Alias: -
    /// - Note: The txid_snapshot type is used to store transaction snapshot information for use by the txid_snapshot_in() and txid_snapshot_out() functions.
    /// - https://www.postgresql.org/docs/12/datatype-pg-snapshot.html#DATATYPE-TXID-SNAPSHOT
    TxidSnapshot,
    /// Range of integer.
    /// - Range: -
    /// - Storage Size: 16 bytes
    /// - Category: Range
    /// - SQL Type: INT4RANGE
    /// - Alias: -
    /// - Note: The int4range type is used to represent a range of integer values.
    /// - https://www.postgresql.org/docs/12/rangetypes.html#RANGETYPES-INT4RANGE
    Int4Range,
    /// Range of bigint.
    /// - Range: -
    /// - Storage Size: 16 bytes
    /// - Category: Range
    /// - SQL Type: INT8RANGE
    /// - Alias: -
    /// - Note: The int8range type is used to represent a range of bigint values.
    /// - https://www.postgresql.org/docs/12/rangetypes.html#RANGETYPES-INT8RANGE
    Int8Range,
    /// Range of numeric.
    /// - Range: -
    /// - Storage Size: 16 bytes
    /// - Category: Range
    /// - SQL Type: NUMRANGE
    /// - Alias: -
    /// - Note: The numrange type is used to represent a range of numeric values.
    /// - https://www.postgresql.org/docs/12/rangetypes.html#RANGETYPES-NUMRANGE
    NumRange,
    /// Range of timestamp without time zone.
    /// - Range: -
    /// - Storage Size: 16 bytes
    /// - Category: Range
    /// - SQL Type: TSRANGE
    /// - Alias: -
    /// - Note: The tsrange type is used to represent a range of timestamp without time zone values.
    /// - https://www.postgresql.org/docs/12/rangetypes.html#RANGETYPES-TSRANGE
    TsRange,
    /// Range of timestamp with time zone.
    /// - Range: -
    /// - Storage Size: 16 bytes
    /// - Category: Range
    /// - SQL Type: TSTZRANGE
    /// - Alias: -
    /// - Note: The tstzrange type is used to represent a range of timestamp with time zone values.
    /// - https://www.postgresql.org/docs/12/rangetypes.html#RANGETYPES-TSTZRANGE
    TstzRange,
    /// Range of date.
    /// - Range: -
    /// - Storage Size: 16 bytes
    /// - Category: Range
    /// - SQL Type: DATERANGE
    /// - Alias: -
    /// - Note: The daterange type is used to represent a range of date values.
    /// - https://www.postgresql.org/docs/12/rangetypes.html#RANGETYPES-DATERANGE
    DateRange,
    /// Array of select types.
    /// - Range: -
    /// - Storage Size: 1 or 4 bytes + N * length of element type
    /// - Category: Array
    /// - SQL Type: _TYPE
    /// - Alias: -
    /// - Note: Arrays of any built-in or user-defined base type, enum type, or composite type can be created. Arrays of domains are not yet supported.
    /// - https://www.postgresql.org/docs/12/arrays.html#ARRAYS-DECLARATION
    Array(Box<PostgresTypes>),
}

#[derive(Debug, Clone, PartialEq, std::cmp::Eq)]
pub struct PostgresTypesMap {
    pub types: HashMap<String, Vec<PostgresTypes>>,
}

impl PostgresTypes {
    pub fn category(&self) -> DataTypeCategory {
        match self {
            PostgresTypes::Boolean
            | PostgresTypes::Serial
            | PostgresTypes::Bit
            | PostgresTypes::BitVarying
            | PostgresTypes::Money => DataTypeCategory::Numeric,

            PostgresTypes::TimeWithTimeZone
            | PostgresTypes::TimestampWithTimeZone
            | PostgresTypes::Interval => DataTypeCategory::DateTime,

            PostgresTypes::Json
            | PostgresTypes::JsonB
            | PostgresTypes::Xml
            | PostgresTypes::Enum => DataTypeCategory::Text,

            PostgresTypes::Point
            | PostgresTypes::Line
            | PostgresTypes::Lseg
            | PostgresTypes::Box
            | PostgresTypes::Path
            | PostgresTypes::Polygon
            | PostgresTypes::Circle => DataTypeCategory::Geometric,

            PostgresTypes::Inet | PostgresTypes::Cidr => DataTypeCategory::NetworkAddress,

            PostgresTypes::TsVector | PostgresTypes::TsQuery => DataTypeCategory::Search,

            PostgresTypes::Array(_) => DataTypeCategory::Array,

            PostgresTypes::Uuid => DataTypeCategory::UUID,

            PostgresTypes::PgSnapshot
            | PostgresTypes::TxidSnapshot
            | PostgresTypes::Int4Range
            | PostgresTypes::Int8Range
            | PostgresTypes::NumRange
            | PostgresTypes::TsRange
            | PostgresTypes::TstzRange
            | PostgresTypes::PgLsn
            | PostgresTypes::DateRange => DataTypeCategory::Range,
            PostgresTypes::SmallInt => todo!(),
            PostgresTypes::Integer => todo!(),
            PostgresTypes::BigInt => todo!(),
            PostgresTypes::Decimal => todo!(),
            PostgresTypes::Real => todo!(),
            PostgresTypes::DoublePrecision => todo!(),
            PostgresTypes::Numeric => todo!(),
            PostgresTypes::SmallSerial => todo!(),
            PostgresTypes::BigSerial => todo!(),
            PostgresTypes::Varchar => todo!(),
            PostgresTypes::CharVarying => todo!(),
            PostgresTypes::CharacterVarying => todo!(),
            PostgresTypes::Char => todo!(),
            PostgresTypes::Character => todo!(),
            PostgresTypes::Text => todo!(),
            PostgresTypes::Bytea => todo!(),
            PostgresTypes::Timestamp => todo!(),
            PostgresTypes::TimestampWithoutTimeZone => todo!(),
            PostgresTypes::Date => todo!(),
            PostgresTypes::Time => todo!(),
            PostgresTypes::TimeWithoutTimeZone => todo!(),
            PostgresTypes::PathOpen => todo!(),
            PostgresTypes::Macaddr => todo!(),
            PostgresTypes::Macaddr8 => todo!(),
        }
    }

    /// Orders the types by alphabetical order.
    pub fn order_by_alphabetical_order(types: &mut Vec<PostgresTypes>) {
        types.sort_by(|a, b| {
            let a = format!("{:?}", a);
            let b = format!("{:?}", b);
            a.cmp(&b)
        });
    }
}

impl Ord for PostgresTypesMap {
    fn cmp(&self, other: &Self) -> Ordering {
        self.types.len().cmp(&other.types.len())
    }
}

impl PartialOrd for PostgresTypesMap {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
