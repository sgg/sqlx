//! **MsSQL** database and connection types.

pub use arguments::MsSqlArguments;
pub use connection::MsSqlConnection;
pub use cursor::MsSqlCursor;
pub use database::MsSql;
pub use error::MsSqlError;
pub use row::MsSqlRow;
pub use type_info::MsSqlTypeInfo;
pub use value::MsSqlValue;

mod arguments;
mod connection;
mod cursor;
mod database;
mod error;
mod executor;
mod protocol;
mod row;
mod stream;
mod type_info;
pub mod types;
mod value;

/// An alias for [`Pool`], specialized for **MsSQL**.
pub type MsSqlPool = crate::pool::Pool<MsSqlConnection>;

make_query_as!(MsSqlQueryAs, MsSql, MsSqlRow);
impl_map_row_for_row!(MsSql, MsSqlRow);
impl_from_row_for_tuples!(MsSql, MsSqlRow);
