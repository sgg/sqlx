use crate::decode::Decode;
use crate::mssql::{MsSql, MsSqlValue};

impl<'de, T> Decode<'de, MsSql> for Option<T>
where
    T: Decode<'de, MsSql>,
{
    fn decode(value: MsSqlValue<'de>) -> crate::Result<Self> {
        todo!()
    }
}
