use crate::error::DatabaseError;

#[derive(Debug)]
pub struct MsSqlError();

impl DatabaseError for MsSqlError {
    fn message(&self) -> &str {
        todo!()
    }
}

impl_fmt_error!(MsSqlError);
