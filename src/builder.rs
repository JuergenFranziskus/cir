use crate::*;




pub struct Builder {
    module: Module,
    fun: Option<FunID>,
    block: Option<BlockID>,
}
impl Builder {
    pub fn new(module: Module) -> Self {
        Self {
            module,
            fun: None,
            block: None,
        }
    }
    pub fn finish(self) -> Module {
        self.module
    }


    pub fn begin_fun(&mut self, name: String, ret_ty: impl Into<Ty>) -> FunID {
        let id = self.module.add_function(name, ret_ty.into());
        self.fun = Some(id);
        id
    }
    pub fn create_reg(&mut self, ty: impl Into<Ty>) -> RegID {
        let fun = self.fun.unwrap();
        let id = self.module.add_register(fun, ty.into());
        id
    }
    pub fn create_var(&mut self, ty: impl Into<Ty>) -> VarID {
        let fun = self.fun.unwrap();
        let id = self.module.add_variable(fun, ty.into());
        id
    }
    pub fn add_param(&mut self, reg: RegID) {
        let fun = self.fun.unwrap();
        self.module.add_parameter(fun, reg);
    }
    pub fn create_param(&mut self, ty: impl Into<Ty>) -> RegID {
        let fun = self.fun.unwrap();
        let reg = self.module.add_register(fun, ty.into());
        self.module.add_parameter(fun, reg);
        reg
    }
    pub fn set_entry_block(&mut self) {
        let fun = self.fun.unwrap();
        let block = self.block.unwrap();
        self.module.set_entry_block(fun, block);
    }
    pub fn begin_block(&mut self) -> BlockID {
        let fun = self.fun.unwrap();
        let id = self.module.add_block(fun);
        self.block = Some(id);
        id
    }
    pub fn add_block_param(&mut self, param: RegID) {
        let block = self.block.unwrap();
        self.module.add_block_parameter(block, param);
    }
    pub fn create_block_param(&mut self, ty: impl Into<Ty>) -> RegID {
        let reg = self.create_reg(ty);
        self.add_block_param(reg);
        reg
    }
    pub fn add_instr(&mut self, instr: Instruction) {
        let block = self.block.unwrap();
        self.module.add_instruction(block, instr);
    }

    pub fn set_int(&mut self, ty: IntTy, value: impl Into<i64>) -> RegID {
        let reg = self.create_reg(ty);
        self.add_instr(Instruction::SetInt(reg, value.into()));
        reg
    }
    pub fn set_bool(&mut self, value: bool) -> RegID {
        let reg = self.create_reg(Ty::Bool);
        self.add_instr(Instruction::SetBool(reg, value));
        reg
    }
    pub fn set_array(&mut self, values: impl Into<Vec<RegID>>) -> RegID {
        let values = values.into();
        let ty = self.module[values[0]].ty;
        let arr_ty = self.module.add_array_ty(values.len() as u64, ty);
        let reg = self.create_reg(arr_ty);
        self.add_instr(Instruction::SetArray(reg, values));
        reg
    }
    pub fn set_struct(&mut self, ty: StructTyID, values: impl Into<Vec<RegID>>) -> RegID {
        let reg = self.create_reg(ty);
        self.add_instr(Instruction::SetStruct(reg, values.into()));
        reg
    }

    pub fn add(&mut self, a: RegID, b: RegID) -> RegID {
        let ty = self.module[a].ty;
        let reg = self.create_reg(ty);
        self.add_instr(Instruction::Add(reg, a, b));
        reg
    }
    pub fn sub(&mut self, a: RegID, b: RegID) -> RegID {
        let ty = self.module[a].ty;
        let reg = self.create_reg(ty);
        self.add_instr(Instruction::Sub(reg, a, b));
        reg
    }
    pub fn mul(&mut self, a: RegID, b: RegID) -> RegID {
        let ty = self.module[a].ty;
        let reg = self.create_reg(ty);
        self.add_instr(Instruction::Mul(reg, a, b));
        reg
    }
    pub fn idiv(&mut self, a: RegID, b: RegID) -> RegID {
        let ty = self.module[a].ty;
        let reg = self.create_reg(ty);
        self.add_instr(Instruction::IDiv(reg, a, b));
        reg
    }
    pub fn udiv(&mut self, a: RegID, b: RegID) -> RegID {
        let ty = self.module[a].ty;
        let reg = self.create_reg(ty);
        self.add_instr(Instruction::UDiv(reg, a, b));
        reg
    }
    pub fn imod(&mut self, a: RegID, b: RegID) -> RegID {
        let ty = self.module[a].ty;
        let reg = self.create_reg(ty);
        self.add_instr(Instruction::IMod(reg, a, b));
        reg
    }
    pub fn umod(&mut self, a: RegID, b: RegID) -> RegID {
        let ty = self.module[a].ty;
        let reg = self.create_reg(ty);
        self.add_instr(Instruction::UMod(reg, a, b));
        reg
    }
    pub fn and(&mut self, a: RegID, b: RegID) -> RegID {
        let ty = self.module[a].ty;
        let reg = self.create_reg(ty);
        self.add_instr(Instruction::And(reg, a, b));
        reg
    }
    pub fn or(&mut self, a: RegID, b: RegID) -> RegID {
        let ty = self.module[a].ty;
        let reg = self.create_reg(ty);
        self.add_instr(Instruction::Or(reg, a, b));
        reg
    }
    pub fn xor(&mut self, a: RegID, b: RegID) -> RegID {
        let ty = self.module[a].ty;
        let reg = self.create_reg(ty);
        self.add_instr(Instruction::Xor(reg, a, b));
        reg
    }
    pub fn shl(&mut self, a: RegID, b: RegID) -> RegID {
        let ty = self.module[a].ty;
        let reg = self.create_reg(ty);
        self.add_instr(Instruction::ShiftLeft(reg, a, b));
        reg
    }
    pub fn shr(&mut self, a: RegID, b: RegID) -> RegID {
        let ty = self.module[a].ty;
        let reg = self.create_reg(ty);
        self.add_instr(Instruction::ShiftRightLogical(reg, a, b));
        reg
    }
    pub fn sar(&mut self, a: RegID, b: RegID) -> RegID {
        let ty = self.module[a].ty;
        let reg = self.create_reg(ty);
        self.add_instr(Instruction::ShiftRightArithmetic(reg, a, b));
        reg
    }
    pub fn neg(&mut self, a: RegID) -> RegID {
        let ty = self.module[a].ty;
        let reg = self.create_reg(ty);
        self.add_instr(Instruction::Neg(reg, a));
        reg
    }
    pub fn not(&mut self, a: RegID) -> RegID {
        let ty = self.module[a].ty;
        let reg = self.create_reg(ty);
        self.add_instr(Instruction::Not(reg, a));
        reg
    }
    pub fn int_to_ptr(&mut self, a: RegID) -> RegID {
        let reg = self.create_reg(Ty::Ptr);
        self.add_instr(Instruction::IntToPtr(reg, a));
        reg
    }
    pub fn ptr_to_int(&mut self, ty: IntTy, ptr: RegID) -> RegID {
        let reg = self.create_reg(ty);
        self.add_instr(Instruction::PtrToInt(reg, ptr));
        reg
    }
    pub fn sext(&mut self, ty: IntTy, a: RegID) -> RegID {
        let reg = self.create_reg(ty);
        self.add_instr(Instruction::Sext(reg, a));
        reg
    }
    pub fn zext(&mut self, ty: IntTy, a: RegID) -> RegID {
        let reg = self.create_reg(ty);
        self.add_instr(Instruction::Sext(reg, a));
        reg
    }
    pub fn trunc(&mut self, ty: IntTy, a: RegID) -> RegID {
        let reg = self.create_reg(ty);
        self.add_instr(Instruction::Truncate(reg, a));
        reg
    }

    pub fn poison(&mut self, ty: impl Into<Ty>) -> RegID {
        let reg = self.create_reg(ty);
        self.add_instr(Instruction::Poison(reg));
        reg
    }
    pub fn select(&mut self, c: RegID, a: RegID, b: RegID) -> RegID {
        let ty = self.module[a].ty;
        let reg = self.create_reg(ty);
        self.add_instr(Instruction::Select(reg, c, a, b));
        reg
    }
    pub fn freeze(&mut self, a: RegID) -> RegID {
        let ty = self.module[a].ty;
        let reg = self.create_reg(ty);
        self.add_instr(Instruction::Freeze(reg, a));
        reg
    }

    pub fn get_var_addr(&mut self, v: VarID) -> RegID {
        let reg = self.create_reg(Ty::Ptr);
        self.add_instr(Instruction::GetVarAddr(reg, v));
        reg
    }
    pub fn store(&mut self, ptr: RegID, value: RegID) {
        self.add_instr(Instruction::Store { ptr, value });
    }
    pub fn load(&mut self, ty: impl Into<Ty>, ptr: RegID) -> RegID {
        let dst = self.create_reg(ty);
        self.add_instr(Instruction::Load { ptr, dst });
        dst
    }

    pub fn jump(&mut self, tgt: impl Into<JumpTarget>) {
        self.add_instr(Instruction::Jump(tgt.into()));
    }
    pub fn branch(&mut self, c: RegID, t: impl Into<JumpTarget>, f: impl Into<JumpTarget>) {
        self.add_instr(Instruction::Branch(c, t.into(), f.into()));
    }
    pub fn ret(&mut self, value: Option<RegID>) {
        self.add_instr(Instruction::Ret(value));
    }
    pub fn call(&mut self, void: bool, fun: FunID, args: impl Into<Vec<RegID>>) -> Option<RegID> {
        let ret_ty = self.module[fun].ret_ty;
        let reg = if void { None } else { Some(self.create_reg(ret_ty)) };
        self.add_instr(Instruction::Call(reg, fun, args.into()));
        reg
    }
    pub fn call_ptr(&mut self, void: bool, ptr: RegID, fun_ty: FunTyID, args: impl Into<Vec<RegID>>) -> Option<RegID> {
        let ret_ty = self.module[fun_ty].ret;
        let reg = if void { None } else { Some(self.create_reg(ret_ty)) };
        self.add_instr(Instruction::CallPtr(reg, ptr, fun_ty, args.into()));
        reg
    }

    pub fn get_struct_member(&mut self, strct: RegID, index: u64) -> RegID {
        let Ty::Struct(struct_ty) = self.module[strct].ty else { panic!() };
        let member = self.module[struct_ty].members[index as usize];
        let dst = self.create_reg(member);
        self.add_instr(Instruction::GetStructMember {dst, strct, index });
        dst
    }
    pub fn set_struct_member(&mut self, strct: RegID, index: u64, to: RegID) -> RegID {
        let struct_ty = self.module[strct].ty;
        let dst = self.create_reg(struct_ty);
        self.add_instr(Instruction::SetStructMember { dst, strct, value: to, index });
        dst
    }
    pub fn get_array_element(&mut self, array: RegID, index: RegID) -> RegID {
        let Ty::Array(arr_ty) = self.module[array].ty else { panic!() };
        let element = self.module[arr_ty].element;
        let dst = self.create_reg(element);
        self.add_instr(Instruction::GetArrayElement { dst, array, index });
        dst
    }
    pub fn set_array_element(&mut self, array: RegID, index: RegID, to: RegID) -> RegID {
        let arr_ty = self.module[array].ty;
        let dst = self.create_reg(arr_ty);
        self.add_instr(Instruction::SetArrayElement { dst, array, value: to, index });
        dst
    }
    pub fn index_struct(&mut self, ty: StructTyID, ptr: RegID, index: u64) -> RegID {
        let ty = self.module[ty].members[index as usize];
        let dst = self.create_reg(ty);
        self.add_instr(Instruction::IndexStruct { dst, ptr, index });
        dst
    }
    pub fn index_array(&mut self, ty: ArrayTyID, ptr: RegID, index: RegID) -> RegID {
        let ty = self.module[ty].element;
        let dst = self.create_reg(ty);
        self.add_instr(Instruction::IndexArray { dst, ptr, index });
        dst
    }
}
