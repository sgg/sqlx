use super::MsSql;
use crate::error::DatabaseError;
use std::error::{Error as StdError, Error};
use std::fmt::{self, Display};

#[derive(Debug)]
pub struct MsSqlError {}

impl DatabaseError for MsSqlError {
    fn message(&self) -> &str {
        todo!()
    }

    fn code(&self) -> Option<&str> {
        todo!()
    }

    fn details(&self) -> Option<&str> {
        todo!()
    }

    fn hint(&self) -> Option<&str> {
        todo!()
    }

    fn table_name(&self) -> Option<&str> {
        todo!()
    }

    fn column_name(&self) -> Option<&str> {
        todo!()
    }

    fn constraint_name(&self) -> Option<&str> {
        todo!()
    }

    fn as_ref_err(&self) -> &(dyn StdError + Send + Sync + 'static) {
        self
    }

    fn as_mut_err(&mut self) -> &mut (dyn StdError + Send + Sync + 'static) {
        self
    }

    fn into_box_err(self: Box<Self>) -> Box<dyn StdError + Send + Sync + 'static> {
        self
    }
}

impl Display for MsSqlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!()
    }
}

impl StdError for MsSqlError {}

impl From<MsSqlError> for crate::Error {
    fn from(err: MsSqlError) -> Self {
        crate::Error::Database(Box::new(err))
    }
}
