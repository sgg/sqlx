use futures_core::future::BoxFuture;

use crate::database::{Database, HasRow};
use crate::executor::Execute;
use crate::pool::Pool;

/// Represents a result set, which is generated by executing a query against the database.
///
/// A `Cursor` can be created by either [`Executor::execute`](trait.Execute.html) or
/// [`Query::fetch`](struct.Query.html).
///
/// Initially the `Cursor` is positioned before the first row. The `next` method moves the cursor
/// to the next row, and because it returns `None` when there are no more rows, it can be used
/// in a `while` loop to iterate through all returned rows.
pub trait Cursor<'c, 'q>
where
    Self: Send,
{
    type Database: Database;

    fn from_pool<E>(pool: &Pool<<Self::Database as Database>::Connection>, query: E) -> Self
    where
        Self: Sized,
        E: Execute<'q, Self::Database>;

    fn from_connection<E>(
        connection: &'c mut <Self::Database as Database>::Connection,
        query: E,
    ) -> Self
    where
        Self: Sized,
        E: Execute<'q, Self::Database>;

    /// Fetch the next row in the result. Returns `None` if there are no more rows.
    fn next<'cur>(
        &'cur mut self,
    ) -> BoxFuture<'cur, crate::Result<Self::Database, Option<<Self::Database as HasRow<'cur>>::Row>>>;
}
