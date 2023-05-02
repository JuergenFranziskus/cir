use self::{
    block::{Block, BlockID},
    function::{FuncID, Function},
    instruction::Instruction,
    register::{RegID, Register},
    variable::{VarID, Variable},
};
use crate::types::Type;
use std::ops::{Index, IndexMut};

pub mod block;
pub mod calling_convention;
pub mod function;
pub mod instruction;
pub mod register;
pub mod variable;

#[derive(Clone, Debug)]
pub struct Module {
    pub(crate) functions: Vec<Function>,
    pub(crate) registers: Vec<Register>,
    pub(crate) variables: Vec<Variable>,
    pub(crate) blocks: Vec<Block>,
}
impl Module {
    pub fn new() -> Self {
        Self {
            functions: Vec::new(),
            registers: Vec::new(),
            variables: Vec::new(),
            blocks: Vec::new(),
        }
    }

    pub fn add_function(&mut self, name: impl Into<String>, ret_type: impl Into<Type>) -> FuncID {
        let id = FuncID(self.functions.len());
        self.functions
            .push(Function::new(id, name.into(), ret_type.into()));
        id
    }
    pub fn start_function_definition(&mut self, fid: FuncID) -> BlockID {
        let entry = BlockID(self.blocks.len());
        self.blocks.push(Block::new(fid, entry));
        self[fid].start_definition(entry);
        entry
    }

    pub fn add_register(&mut self, fid: FuncID, reg_type: impl Into<Type>) -> RegID {
        let id = RegID(self.registers.len());
        self.registers.push(Register::new(fid, id, reg_type.into()));
        self[fid].add_register(id);
        id
    }
    pub fn add_parameter(&mut self, fid: FuncID, param_type: impl Into<Type>) -> RegID {
        let id = RegID(self.registers.len());
        self.registers
            .push(Register::new(fid, id, param_type.into()));
        self[fid].add_parameter(id);
        id
    }
    pub fn add_variable(&mut self, fid: FuncID, var_type: impl Into<Type>) -> VarID {
        let id = VarID(self.variables.len());
        self.variables.push(Variable::new(fid, id, var_type.into()));
        self[fid].add_var(id);
        id
    }

    pub fn add_block(&mut self, fid: FuncID) -> BlockID {
        let id = BlockID(self.blocks.len());
        self.blocks.push(Block::new(fid, id));
        self[fid].add_block(id);
        id
    }
    pub fn add_block_parameter(&mut self, bid: BlockID, param_type: impl Into<Type>) -> RegID {
        let fid = self[bid].function();
        let rid = self.add_register(fid, param_type);
        self[bid].add_parameter(rid);
        rid
    }
    pub fn push_instruction(&mut self, bid: BlockID, i: Instruction) {
        self[bid].push_instruction(i);
    }
}
impl Index<FuncID> for Module {
    type Output = Function;
    fn index(&self, index: FuncID) -> &Self::Output {
        &self.functions[index.0]
    }
}
impl IndexMut<FuncID> for Module {
    fn index_mut(&mut self, index: FuncID) -> &mut Self::Output {
        &mut self.functions[index.0]
    }
}
impl Index<BlockID> for Module {
    type Output = Block;
    fn index(&self, index: BlockID) -> &Self::Output {
        &self.blocks[index.0]
    }
}
impl IndexMut<BlockID> for Module {
    fn index_mut(&mut self, index: BlockID) -> &mut Self::Output {
        &mut self.blocks[index.0]
    }
}
impl Index<RegID> for Module {
    type Output = Register;
    fn index(&self, index: RegID) -> &Self::Output {
        &self.registers[index.0]
    }
}
impl IndexMut<RegID> for Module {
    fn index_mut(&mut self, index: RegID) -> &mut Self::Output {
        &mut self.registers[index.0]
    }
}
impl Index<VarID> for Module {
    type Output = Variable;
    fn index(&self, index: VarID) -> &Self::Output {
        &self.variables[index.0]
    }
}
impl IndexMut<VarID> for Module {
    fn index_mut(&mut self, index: VarID) -> &mut Self::Output {
        &mut self.variables[index.0]
    }
}
