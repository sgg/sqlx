use crate::error::DatabaseError;

pub struct MsSqlError();

impl DatabaseError for MsSqlError {
    fn message(&self) -> &str {
        todo!()
    }
}

impl_fmt_error!(MsSqlError);
