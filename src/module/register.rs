use crate::types::Type;
use super::function::FuncID;



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


