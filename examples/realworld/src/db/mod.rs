use async_trait::async_trait;

/// Database implementation for PostgreSQL
#[cfg(feature = "postgres")]
pub mod pg;

/// Database implementation for SQLite
///
/// The implementation of the handler functions is a bit more complex than Postgres
/// as sqlite (1) does not support nested transactions and (2) does not support the RETURNING
/// clause.
#[cfg(feature = "sqlite")]
pub mod sqlite;

/// Database models
pub mod model;

/// A type that abstracts a database
#[async_trait]
pub trait Db {
    /// The type of connection returned by the database
    type Conn;

    /// Establish a connection with the database
    async fn conn(&self) -> sqlx::Result<Self::Conn>;
}

#[async_trait]
pub(crate) trait DbPool {
    type Conn;

    async fn conn(&self) -> sqlx::Result<Self::Conn>;
}

#[async_trait]
impl<DB: sqlx::Database> DbPool for sqlx::Pool<DB> {
    type Conn = sqlx::pool::PoolConnection<DB>;

    async fn conn(&self) -> sqlx::Result<Self::Conn> {
        self.acquire().await
    }
}

#[async_trait]
impl<DB: sqlx::Database> Db for sqlx::Pool<DB> {
    type Conn = sqlx::pool::PoolConnection<DB>;

    async fn conn(&self) -> sqlx::Result<Self::Conn> {
        self.acquire().await
    }
}

/// Create a batch insert statement
///
/// This incantation borrowed from @mehcode
/// https://discordapp.com/channels/665528275556106240/665528275556106243/694835667401703444
fn build_batch_insert(rows: usize, columns: usize) -> String {
    use itertools::Itertools;

    (0..rows)
        .format_with(",", |i, f| {
            f(&format_args!(
                "({})",
                (1..=columns).format_with(",", |j, f| f(&format_args!("${}", j + (i * columns))))
            ))
        })
        .to_string()
}
