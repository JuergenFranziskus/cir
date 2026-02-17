use std::ops::Index;

use crate::{block::{Block, BlockID}, function::{CallConvention, FunID, Function}, instruction::Instruction, register::{RegID, Register}, target::Target, types::{ArrayTy, ArrayTyID, FunTy, FunTyID, StructTy, StructTyID, Ty, Types}, variable::{VarID, Variable}};


pub struct Module {
    types: Types,
    
    target: Target,
    functions: Vec<Function>,
    registers: Vec<Register>,
    variables: Vec<Variable>,

    blocks: Vec<Block>,
}
impl Module {
    pub fn new(target: Target) -> Self {
        Self {
            types: Types::new(),

            target,
            functions: Vec::new(),
            registers: Vec::new(),
            variables: Vec::new(),

            blocks: Vec::new(),
        }
    }

    pub fn target(&self) -> Target {
        self.target
    }

    pub fn add_fun_ty(&mut self, call_convention: CallConvention, ret: Ty, params: impl Into<Vec<Ty>>) -> FunTyID {
        self.types.add_func_type(call_convention, ret, params)
    }
    pub fn add_array_ty(&mut self, size: u64, element: Ty) -> ArrayTyID {
        self.types.add_array_type(size, element)
    }
    pub fn add_struct_ty(&mut self) -> StructTyID {
        self.types.add_struct_type()
    }
    pub fn add_struct_member(&mut self, strct: StructTyID, member: Ty) {
        self.types.add_struct_member(strct, member);
    }

    pub fn add_function(&mut self, name: String, ret_ty: Ty) -> FunID {
        let id = FunID(self.functions.len());
        self.functions.push(Function::new(id, name, ret_ty));
        id
    }
    pub fn set_call_convention(&mut self, fun: FunID, convention: CallConvention) {
        self.functions[fun.0].call_convention = convention;
    }
    pub fn add_register(&mut self, fun: FunID, ty: Ty) -> RegID {
        let id = RegID(self.registers.len());
        self.registers.push(Register::new(id, fun, ty));

        self.functions[fun.0].registers.insert(id);

        id
    }
    pub fn add_parameter(&mut self, fun: FunID, reg: RegID) {
        assert_eq!(self[reg].fun, fun);
        let func = &mut self.functions[fun.0];
        func.parameters.push(reg);
    }
    pub fn add_variable(&mut self, fun: FunID, ty: Ty) -> VarID {
        let id = VarID(self.registers.len());
        self.variables.push(Variable::new(id, fun, ty));

        self.functions[fun.0].variables.insert(id);

        id
    }
    pub fn set_entry_block(&mut self, fun: FunID, block: BlockID) {
        assert_eq!(self[block].fun, fun);
        assert!(self[fun].blocks.contains(&block));
        self.functions[fun.0].entry_block = Some(block);
    }

    pub fn add_block(&mut self, fun: FunID) -> BlockID {
        let id = BlockID(self.blocks.len());
        self.blocks.push(Block::new(id, fun));
        self.functions[fun.0].blocks.insert(id);
        id
    }
    pub fn add_block_parameter(&mut self, block: BlockID, param: RegID) {
        self.blocks[block.0].parameters.push(param);
    }
    pub fn add_instruction(&mut self, block: BlockID, instruction: Instruction) {
        self.blocks[block.0].instructions.push(instruction);
    }


    pub fn functions(&self) -> &[Function] {
        &self.functions
    }
}
impl Index<FunTyID> for Module {
    type Output = FunTy;
    fn index(&self, index: FunTyID) -> &Self::Output {
        &self.types[index]
    }
}
impl Index<ArrayTyID> for Module {
    type Output = ArrayTy;
    fn index(&self, index: ArrayTyID) -> &Self::Output {
        &self.types[index]
    }
}
impl Index<StructTyID> for Module {
    type Output = StructTy;
    
    fn index(&self, index: StructTyID) -> &Self::Output {
        &self.types[index]
    }
}
impl Index<FunID> for Module {
    type Output = Function;
    
    fn index(&self, index: FunID) -> &Self::Output {
        &self.functions[index.0]
    }
}
impl Index<RegID> for Module {
    type Output = Register;
    fn index(&self, index: RegID) -> &Self::Output {
        &self.registers[index.0]
    }
}
impl Index<VarID> for Module {
    type Output = Variable;
    fn index(&self, index: VarID) -> &Self::Output {
        &self.variables[index.0]
    }
}
impl Index<BlockID> for Module {
    type Output = Block;
    fn index(&self, index: BlockID) -> &Self::Output {
        &self.blocks[index.0]
    }
}
