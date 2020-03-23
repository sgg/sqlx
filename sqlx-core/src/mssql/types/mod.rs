use std::fmt::{self, Debug, Display};

use crate::decode::Decode;
use crate::mssql::{MsSql, MsSqlValue};
use crate::types::TypeInfo;

#[derive(Clone, Debug, Default)]
pub struct MsSqlTypeInfo {}

impl MsSqlTypeInfo {
    pub(crate) const fn new() -> Self {
        Self {}
    }
}

impl Display for MsSqlTypeInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl TypeInfo for MsSqlTypeInfo {
    fn compatible(&self, other: &Self) -> bool {
        todo!()
    }
}

impl<'de, T> Decode<'de, MsSql> for Option<T>
where
    T: Decode<'de, MsSql>,
{
    fn decode(value: Option<MsSqlValue<'de>>) -> crate::Result<MsSql, Self> {
        value
            .map(|value| <T as Decode<MsSql>>::decode(Some(value)))
            .transpose()
    }
}
