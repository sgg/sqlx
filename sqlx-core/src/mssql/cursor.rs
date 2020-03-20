use std::collections::HashMap;
use std::sync::Arc;

use futures_core::future::BoxFuture;

use crate::connection::ConnectionSource;
use crate::cursor::Cursor;
use crate::executor::Execute;
use crate::mssql::{MsSql, MsSqlArguments, MsSqlConnection, MsSqlRow};
use crate::pool::Pool;

pub struct MsSqlCursor<'c, 'q> {
    source: ConnectionSource<'c, MsSqlConnection>,
    query: Option<(&'q str, Option<MsSqlArguments>)>,
    column_names: Arc<HashMap<Box<str>, u16>>,
    binary: bool,
}

impl<'c, 'q> Cursor<'c, 'q> for MsSqlCursor<'c, 'q> {
    type Database = MsSql;

    #[doc(hidden)]
    fn from_pool<E>(pool: &Pool<MsSqlConnection>, query: E) -> Self
    where
        Self: Sized,
        E: Execute<'q, MsSql>,
    {
        todo!()
    }

    #[doc(hidden)]
    fn from_connection<E>(conn: &'c mut MsSqlConnection, query: E) -> Self
    where
        Self: Sized,
        E: Execute<'q, MsSql>,
    {
        todo!()
    }

    fn next(&mut self) -> BoxFuture<crate::Result<Option<MsSqlRow<'_>>>> {
        todo!()
    }
}
