use std::collections::HashMap;
use std::convert::TryFrom;
use std::sync::Arc;

use crate::error::UnexpectedNullError;
use crate::mssql::protocol;
use crate::mssql::MsSql;
use crate::row::{ColumnIndex, Row};

#[derive(Debug)]
pub enum MsSqlValue<'c> {
    Binary(&'c [u8]),
    Text(&'c [u8]),
}

pub struct MsSqlRow<'c> {
    pub(super) buffer: &'c [u8],
    pub(super) columns: Arc<HashMap<Box<str>, usize>>,
}

impl<'c> TryFrom<Option<MsSqlValue<'c>>> for MsSqlValue<'c> {
    type Error = crate::Error;

    #[inline]
    fn try_from(value: Option<MsSqlValue<'c>>) -> Result<Self, Self::Error> {
        match value {
            Some(value) => Ok(value),
            None => Err(crate::Error::decode(UnexpectedNullError)),
        }
    }
}

impl<'c> Row<'c> for MsSqlRow<'c> {
    type Database = MsSql;

    fn len(&self) -> usize {
        todo!()
    }

    fn try_get_raw<'r, I>(&'r self, index: I) -> crate::Result<Option<MsSqlValue<'c>>>
    where
        'c: 'r,
        I: ColumnIndex<Self::Database>,
    {
        todo!()
    }
}
