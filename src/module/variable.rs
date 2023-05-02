use crate::types::Type;
use super::function::FuncID;


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
