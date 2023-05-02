use super::{
    block::BlockID, calling_convention::CallingConvention, register::RegID, variable::VarID,
};
use crate::types::Type;
use std::{collections::HashSet, iter::once};

#[derive(Clone, Debug)]
pub struct Function {
    id: FuncID,
    name: String,
    return_type: Type,
    calling_convention: CallingConvention,
    parameters: Vec<FunctionParameter>,
    registers: HashSet<RegID>,
    definition: Option<FunctionDefinition>,
}
impl Function {
    pub(super) fn new(id: FuncID, name: String, return_type: Type) -> Self {
        Self {
            id,
            name,
            return_type,
            calling_convention: CallingConvention::default(),
            parameters: Vec::new(),
            registers: HashSet::new(),
            definition: None,
        }
    }
    pub(super) fn add_register(&mut self, id: RegID) {
        self.registers.insert(id);
    }
    pub(super) fn add_parameter(&mut self, id: RegID) {
        self.parameters.push(id.into());
        self.registers.insert(id);
    }
    pub(super) fn start_definition(&mut self, entry: BlockID) {
        self.definition = Some(FunctionDefinition {
            variables: HashSet::new(),
            blocks: once(entry).collect(),
            entry,
        })
    }
    pub(super) fn add_block(&mut self, id: BlockID) {
        let def = self.definition.as_mut().unwrap();
        def.blocks.insert(id);
    }
    pub(super) fn add_var(&mut self, id: VarID) {
        let def = self.definition.as_mut().unwrap();
        def.variables.insert(id);
    }

    pub fn id(&self) -> FuncID {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn return_type(&self) -> Type {
        self.return_type
    }
    pub fn calling_convention(&self) -> CallingConvention {
        self.calling_convention
    }
    pub fn parameters(&self) -> &[FunctionParameter] {
        &self.parameters
    }
    pub fn registers(&self) -> &HashSet<RegID> {
        &self.registers
    }
    pub fn definition(&self) -> Option<&FunctionDefinition> {
        self.definition.as_ref()
    }
    pub fn variables(&self) -> Option<&HashSet<VarID>> {
        self.definition().map(FunctionDefinition::variables)
    }
    pub fn blocks(&self) -> Option<&HashSet<BlockID>> {
        self.definition().map(FunctionDefinition::blocks)
    }
    pub fn entry_block(&self) -> Option<BlockID> {
        self.definition().map(FunctionDefinition::entry)
    }
}

#[derive(Clone, Debug)]
pub struct FunctionDefinition {
    variables: HashSet<VarID>,
    blocks: HashSet<BlockID>,
    entry: BlockID,
}
impl FunctionDefinition {
    pub fn variables(&self) -> &HashSet<VarID> {
        &self.variables
    }
    pub fn blocks(&self) -> &HashSet<BlockID> {
        &self.blocks
    }
    pub fn entry(&self) -> BlockID {
        self.entry
    }
}

#[derive(Clone, Debug)]
pub struct FunctionParameter {
    register: RegID,
}
impl FunctionParameter {
    pub fn register(&self) -> RegID {
        self.register
    }
}
impl From<RegID> for FunctionParameter {
    fn from(value: RegID) -> Self {
        Self { register: value }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct FuncID(pub usize);
