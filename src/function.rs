use std::collections::HashSet;

use crate::{block::BlockID, register::RegID, types::Ty, variable::VarID};



#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Function {
    pub id: FunID,
    pub name: String,
    pub call_convention: CallConvention,
    pub ret_ty: Ty,
    pub registers: HashSet<RegID>,
    pub parameters: Vec<RegID>,
    pub variables: HashSet<VarID>,
    pub blocks: HashSet<BlockID>,
    pub entry_block: Option<BlockID>,
}
impl Function {
    pub(crate) fn new(id: FunID, name: String, ret_ty: Ty) -> Self {
        Self {
            id,
            name,
            call_convention: CallConvention::Simple,
            ret_ty,
            registers: HashSet::new(),
            parameters: Vec::new(),
            variables: HashSet::new(),
            blocks: HashSet::new(),
            entry_block: None,
        }        
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct FunID(pub(crate) usize);


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum CallConvention {
    Simple,
}
