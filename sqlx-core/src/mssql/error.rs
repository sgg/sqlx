use super::MsSql;
use crate::error::DatabaseError;
use std::error::Error as StdError;
use std::fmt::{self, Display};

#[derive(Debug)]
pub struct MsSqlError();

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
}

impl Display for MsSqlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!()
    }
}

impl StdError for MsSqlError {}

impl From<MsSqlError> for crate::Error<MsSql> {
    fn from(err: MsSqlError) -> Self {
        crate::Error::Database(Box::new(err))
    }
}
