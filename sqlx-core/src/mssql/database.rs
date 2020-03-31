use crate::cursor::HasCursor;
use crate::database::Database;
use crate::row::HasRow;
use crate::value::HasRawValue;

/// **MsSql** database driver.
#[derive(Debug)]
pub struct MsSql;

impl Database for MsSql {
    type Connection = super::MsSqlConnection;

    type Arguments = super::MsSqlArguments;

    type TypeInfo = super::MsSqlTypeInfo;

    type TableId = Box<str>;

    type RawBuffer = Vec<u8>;

    type Error = super::MsSqlError;
}

impl<'c> HasRow<'c> for MsSql {
    type Database = MsSql;

    type Row = super::MsSqlRow<'c>;
}

impl<'c, 'q> HasCursor<'c, 'q> for MsSql {
    type Database = MsSql;

    type Cursor = super::MsSqlCursor<'c, 'q>;
}

impl<'c> HasRawValue<'c> for MsSql {
    type Database = MsSql;

    type RawValue = super::MsSqlValue<'c>;
}
