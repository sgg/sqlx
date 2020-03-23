use futures_core::future::BoxFuture;

use crate::cursor::Cursor;
use crate::describe::{Column, Describe};
use crate::executor::{Execute, Executor, RefExecutor};
use crate::mssql::protocol;
use crate::mssql::{MsSql, MsSqlArguments, MsSqlCursor, MsSqlTypeInfo};

impl Executor for super::MsSqlConnection {
    type Database = MsSql;

    fn execute<'e, 'q: 'e, 'c: 'e, E: 'e>(
        &'c mut self,
        query: E,
    ) -> BoxFuture<'e, crate::Result<MsSql, u64>>
    where
        E: Execute<'q, Self::Database>,
    {
        todo!()
    }

    fn fetch<'q, E>(&mut self, query: E) -> MsSqlCursor<'_, 'q>
    where
        E: Execute<'q, Self::Database>,
    {
        todo!()
    }

    fn describe<'e, 'q, E: 'e>(
        &'e mut self,
        query: E,
    ) -> BoxFuture<'e, crate::Result<MsSql, Describe<Self::Database>>>
    where
        E: Execute<'q, Self::Database>,
    {
        todo!()
    }
}

impl<'c> RefExecutor<'c> for &'c mut super::MsSqlConnection {
    type Database = MsSql;

    fn fetch_by_ref<'q, E>(self, query: E) -> MsSqlCursor<'c, 'q>
    where
        E: Execute<'q, Self::Database>,
    {
        todo!()
    }
}
