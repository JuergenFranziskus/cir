use super::{function::FuncID, instruction::Instruction, register::RegID};
use std::fmt::Display;

#[derive(Clone, Debug)]
pub struct Block {
    pub(crate) id: BlockID,
    pub(crate) function: FuncID,
    pub(crate) parameters: Vec<RegID>,
    pub(crate) body: Vec<Instruction>,
}
impl Block {
    pub(super) fn new(function: FuncID, id: BlockID) -> Self {
        Self {
            id,
            function,
            parameters: Vec::new(),
            body: Vec::new(),
        }
    }
    pub(super) fn add_parameter(&mut self, id: RegID) {
        self.parameters.push(id);
    }
    pub(super) fn push_instruction(&mut self, i: Instruction) {
        self.body.push(i);
    }

    pub fn id(&self) -> BlockID {
        self.id
    }
    pub fn function(&self) -> FuncID {
        self.function
    }
    pub fn parameters(&self) -> &[RegID] {
        &self.parameters
    }
    pub fn body(&self) -> &[Instruction] {
        &self.body
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BlockID(pub usize);
impl Display for BlockID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "@{}", self.0)
    }
}
