use crate::database::{Database, HasCursor, HasRawValue, HasRow};

/// **MsSql** database driver.
pub struct MsSql;

impl Database for MsSql {
    type Connection = super::MsSqlConnection;

    type Arguments = super::MsSqlArguments;

    type TypeInfo = super::MsSqlTypeInfo;

    type TableId = Box<str>;

    type RawBuffer = Vec<u8>;
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
    type RawValue = Option<super::MsSqlValue<'c>>;
}
