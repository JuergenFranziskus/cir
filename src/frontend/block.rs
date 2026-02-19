use super::{function::FunID, instruction::Instruction, register::RegID};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Block {
    pub id: BlockID,
    pub fun: FunID,
    pub parameters: Vec<RegID>,
    pub instructions: Vec<Instruction>,
}
impl Block {
    pub fn new(id: BlockID, fun: FunID) -> Self {
        Self {
            id,
            fun,
            parameters: Vec::new(),
            instructions: Vec::new(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BlockID(pub(crate) usize);
