use crate::mssql::{MsSql, MsSqlTypeInfo};
use crate::value::RawValue;

#[derive(Debug)]
pub struct MsSqlValue<'c> {
    phantom: &'c (),
}

impl<'c> RawValue<'c> for MsSqlValue<'c> {
    type Database = MsSql;

    fn type_info(&self) -> Option<MsSqlTypeInfo> {
        todo!()
    }
}
