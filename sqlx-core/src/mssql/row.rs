use std::collections::HashMap;
use std::convert::TryFrom;
use std::sync::Arc;

use crate::error::UnexpectedNullError;
use crate::mssql::MsSql;
use crate::mssql::{protocol, MsSqlValue};
use crate::row::{ColumnIndex, Row};

pub struct MsSqlRow<'c> {
    pub(super) buffer: &'c [u8],
    pub(super) columns: Arc<HashMap<Box<str>, usize>>,
}

impl crate::row::private_row::Sealed for MsSqlRow<'_> {}

impl<'c> Row<'c> for MsSqlRow<'c> {
    type Database = MsSql;

    fn len(&self) -> usize {
        todo!()
    }

    #[doc(hidden)]
    fn try_get_raw<I>(&self, index: I) -> crate::Result<MsSqlValue<'c>>
    where
        I: ColumnIndex<'c, Self>,
    {
        todo!()
    }
}

impl<'c> ColumnIndex<'c, MsSqlRow<'c>> for usize {
    fn index(&self, row: &MsSqlRow<'c>) -> crate::Result<usize> {
        todo!()
    }
}

impl<'c> ColumnIndex<'c, MsSqlRow<'c>> for str {
    fn index(&self, row: &MsSqlRow<'c>) -> crate::Result<usize> {
        todo!()
    }
}
