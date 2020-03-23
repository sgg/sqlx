use crate::mssql::MsSql;

pub trait Decode<'de> {
    fn decode(buf: &'de [u8]) -> crate::Result<MsSql, Self>
    where
        Self: Sized;
}
