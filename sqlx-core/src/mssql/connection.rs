use std::convert::TryInto;
use crate::mssql::MsSql;
use futures_core::future::BoxFuture;
use crate::connection::{Connect, Connection};
use crate::executor::Executor;
use crate::url::Url;
use crate::mssql::stream::MsSqlStream;
use crate::mssql::protocol::{Prelogin, PreloginOption, Encryption};
use crate::mssql::protocol::Version;

pub struct MsSqlConnection {
// pub(super) stream: MsSqlStream,
// pub(super) is_ready: bool,
// pub(super) cache_statement: HashMap<Box<str>, u32>,
}

impl MsSqlConnection {
    pub async fn new(url: std::result::Result<Url, url::ParseError>) -> crate::Result<MsSql, Self> {
        let url = url?;
        let mut stream = MsSqlStream::new(&url).await?;

        establish(&mut stream, &url).await?;

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

async fn establish(stream: &mut MsSqlStream, url: &Url) -> crate::Result<MsSql, ()> {
    let prelogin = Prelogin::default();

    stream.write(prelogin);
    stream.flush().await?;

    let packet = stream.receive().await?;

    Ok(())
}
