use std::convert::TryInto;
use crate::mssql::MsSql;
use futures_core::future::BoxFuture;
use crate::connection::{Connect, Connection};
use crate::executor::Executor;
use crate::url::Url;

pub struct MsSqlConnection {
// pub(super) stream: MsSqlStream,
// pub(super) is_ready: bool,
// pub(super) cache_statement: HashMap<Box<str>, u32>,
}

impl MsSqlConnection {
    pub(super) async fn new(_url: std::result::Result<Url, url::ParseError>) -> crate::Result<MsSql, Self> {
        Ok(MsSqlConnection{})
    }
}

impl Connect for MsSqlConnection {
    fn connect<T>(url: T) -> BoxFuture<'static, crate::Result<MsSql, MsSqlConnection>>
    where
        T: TryInto<Url, Error = url::ParseError>,
        Self: Sized,
    {
        Box::pin(MsSqlConnection::new(url.try_into()))
    }
}

impl Connection for MsSqlConnection {
    fn close(self) -> BoxFuture<'static, crate::Result<MsSql, ()>> {
        Box::pin(async move {
            Ok(())
        })
    }

    fn ping(&mut self) -> BoxFuture<crate::Result<MsSql, ()>> {
        Box::pin(async move {
            Ok(())
        })
    }
}
