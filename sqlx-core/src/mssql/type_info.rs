use crate::types::TypeInfo;
use std::fmt::{self, Debug, Display};

#[derive(Clone, Debug, Default)]
pub struct MsSqlTypeInfo {}

impl MsSqlTypeInfo {
    pub(crate) const fn new() -> Self {
        Self {}
    }
}

impl PartialEq<MsSqlTypeInfo> for MsSqlTypeInfo {
    fn eq(&self, other: &MsSqlTypeInfo) -> bool {
        todo!()
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
