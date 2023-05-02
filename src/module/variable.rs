use std::fmt::Display;

use super::function::FuncID;
use crate::types::Type;

#[derive(Clone, Debug)]
pub struct Variable {
    id: VarID,
    function: FuncID,
    var_type: Type,
}
impl Variable {
    pub(super) fn new(function: FuncID, id: VarID, var_type: Type) -> Self {
        Self {
            id,
            function,
            var_type,
        }
    }

    pub fn id(&self) -> VarID {
        self.id
    }
    pub fn function(&self) -> FuncID {
        self.function
    }
    pub fn var_type(&self) -> Type {
        self.var_type
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct VarID(pub usize);
impl Display for VarID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "!{}", self.0)
    }
}
