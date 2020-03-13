use crate::arguments::Arguments;
use crate::encode::{Encode, IsNull};
use crate::mssql::MsSql;
use crate::types::Type;

#[derive(Default)]
pub struct MsSqlArguments {}

impl Arguments for MsSqlArguments {
    type Database = MsSql;

    fn reserve(&mut self, len: usize, size: usize) {
        todo!()
    }

    fn add<T>(&mut self, value: T)
    where
        T: Type<Self::Database>,
        T: Encode<Self::Database>,
    {
        todo!()
    }
}
