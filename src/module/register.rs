use std::fmt::Display;

use super::function::FuncID;
use crate::types::Type;

#[derive(Clone, Debug)]
pub struct Register {
    id: RegID,
    function: FuncID,
    reg_type: Type,
}
impl Register {
    pub(super) fn new(fid: FuncID, id: RegID, reg_type: Type) -> Self {
        Self {
            id,
            function: fid,
            reg_type,
        }
    }

    pub fn id(&self) -> RegID {
        self.id
    }
    pub fn function(&self) -> FuncID {
        self.function
    }
    pub fn reg_type(&self) -> Type {
        self.reg_type
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct RegID(pub usize);
impl Display for RegID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "%{}", self.0)
    }
}
