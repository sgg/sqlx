use std::convert::TryInto;
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
    pub(super) async fn new(_url: crate::Result<Url>) -> crate::Result<Self> {
        Ok(MsSqlConnection{})
    }
}

impl Connect for MsSqlConnection {
    fn connect<T>(url: T) -> BoxFuture<'static, crate::Result<MsSqlConnection>>
    where
        T: TryInto<Url, Error = crate::Error>,
        Self: Sized,
    {
        Box::pin(MsSqlConnection::new(url.try_into()))
    }
}

impl Connection for MsSqlConnection {
    fn close(self) -> BoxFuture<'static, crate::Result<()>> {
        Box::pin(async move {
            Ok(())
        })
    }

    fn ping(&mut self) -> BoxFuture<crate::Result<()>> {
        Box::pin(async move {
            Ok(())
        })
    }
}
