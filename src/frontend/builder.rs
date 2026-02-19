use super::*;

pub struct Builder {
    pub module: Module,
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
        self.block = None;
        id
    }
    pub fn select_fun(&mut self, fun: FunID) {
        self.fun = Some(fun);
        self.block = None;
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
    pub fn create_block(&mut self) -> BlockID {
        let fun = self.fun.unwrap();
        let id = self.module.add_block(fun);
        id
    }
    pub fn select_block(&mut self, block: BlockID) {
        let fun = self.fun.unwrap();
        assert_eq!(self.module[block].fun, fun);
        self.block = Some(block);
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

    pub fn set(&mut self, value: impl Into<Value>) -> RegID {
        let value = value.into();
        let ty = self.ty(value);
        let reg = self.create_reg(ty);
        self.add_instr(Instruction::Set(reg, value));
        reg
    }
    pub fn set_fun_ptr(&mut self, fid: FunID) -> RegID {
        let reg = self.create_reg(Ty::Ptr);
        self.add_instr(Instruction::SetFunPtr(reg, fid));
        reg
    }
    pub fn set_array(&mut self, elem_ty: Ty, values: impl Into<Values>) -> RegID {
        let values = values.into();
        let arr_ty = self.module.add_array_ty(values.0.len() as u64, elem_ty);
        let reg = self.create_reg(arr_ty);
        self.add_instr(Instruction::SetArray(reg, values));
        reg
    }
    pub fn set_array_splat(&mut self, elem_ty: Ty, size: u64, value: impl Into<Value>) -> RegID {
        let ty = self.module.add_array_ty(size, elem_ty);
        let dst = self.create_reg(ty);
        self.add_instr(Instruction::SetArraySplat(dst, value.into()));
        dst
    }
    pub fn set_struct(&mut self, ty: StructTyID, values: impl Into<Values>) -> RegID {
        let reg = self.create_reg(ty);
        self.add_instr(Instruction::SetStruct(reg, values.into()));
        reg
    }

    fn binary(&mut self, op: BinOp, a: impl Into<Value>, b: impl Into<Value>) -> RegID {
        let a: Value = a.into();
        let ty = self.ty(a);
        let b = coerce(b, ty);
        let reg = self.create_reg(ty);
        self.add_instr(Instruction::Binary(op, reg, a, b));
        reg
    }
    pub fn add(&mut self, a: impl Into<Value>, b: impl Into<Value>) -> RegID {
        self.binary(BinOp::Add, a, b)
    }
    pub fn sub(&mut self, a: impl Into<Value>, b: impl Into<Value>) -> RegID {
        self.binary(BinOp::Sub, a, b)
    }
    pub fn mul(&mut self, a: impl Into<Value>, b: impl Into<Value>) -> RegID {
        self.binary(BinOp::Mul, a, b)
    }
    pub fn idiv(&mut self, a: impl Into<Value>, b: impl Into<Value>) -> RegID {
        self.binary(BinOp::IDiv, a, b)
    }
    pub fn udiv(&mut self, a: impl Into<Value>, b: impl Into<Value>) -> RegID {
        self.binary(BinOp::UDiv, a, b)
    }
    pub fn imod(&mut self, a: impl Into<Value>, b: impl Into<Value>) -> RegID {
        self.binary(BinOp::IMod, a, b)
    }
    pub fn umod(&mut self, a: impl Into<Value>, b: impl Into<Value>) -> RegID {
        self.binary(BinOp::UMod, a, b)
    }
    pub fn and(&mut self, a: impl Into<Value>, b: impl Into<Value>) -> RegID {
        self.binary(BinOp::And, a, b)
    }
    pub fn or(&mut self, a: impl Into<Value>, b: impl Into<Value>) -> RegID {
        self.binary(BinOp::Or, a, b)
    }
    pub fn xor(&mut self, a: impl Into<Value>, b: impl Into<Value>) -> RegID {
        self.binary(BinOp::Xor, a, b)
    }
    pub fn shl(&mut self, a: impl Into<Value>, b: impl Into<Value>) -> RegID {
        self.binary(BinOp::Shl, a, b)
    }
    pub fn shr(&mut self, a: impl Into<Value>, b: impl Into<Value>) -> RegID {
        self.binary(BinOp::Shr, a, b)
    }
    pub fn sar(&mut self, a: impl Into<Value>, b: impl Into<Value>) -> RegID {
        self.binary(BinOp::Sar, a, b)
    }

    fn test(&mut self, op: BinOp, a: impl Into<Value>, b: impl Into<Value>) -> RegID {
        let a: Value = a.into();
        let a_ty = self.ty(a);
        let b = coerce(b, a_ty);
        let reg = self.create_reg(Ty::Bool);
        self.add_instr(Instruction::Binary(op, reg, a, b));
        reg
    }
    pub fn test_eq(&mut self, a: impl Into<Value>, b: impl Into<Value>) -> RegID {
        self.test(BinOp::Equal, a, b)
    }
    pub fn test_ne(&mut self, a: impl Into<Value>, b: impl Into<Value>) -> RegID {
        self.test(BinOp::NotEqual, a, b)
    }
    pub fn test_g(&mut self, a: impl Into<Value>, b: impl Into<Value>) -> RegID {
        self.test(BinOp::Greater, a, b)
    }
    pub fn test_ge(&mut self, a: impl Into<Value>, b: impl Into<Value>) -> RegID {
        self.test(BinOp::GreaterEqual, a, b)
    }
    pub fn test_a(&mut self, a: impl Into<Value>, b: impl Into<Value>) -> RegID {
        self.test(BinOp::Above, a, b)
    }
    pub fn test_ae(&mut self, a: impl Into<Value>, b: impl Into<Value>) -> RegID {
        self.test(BinOp::AboveEqual, a, b)
    }
    pub fn test_l(&mut self, a: impl Into<Value>, b: impl Into<Value>) -> RegID {
        self.test(BinOp::Less, a, b)
    }
    pub fn test_le(&mut self, a: impl Into<Value>, b: impl Into<Value>) -> RegID {
        self.test(BinOp::LessEqual, a, b)
    }
    pub fn test_b(&mut self, a: impl Into<Value>, b: impl Into<Value>) -> RegID {
        self.test(BinOp::Below, a, b)
    }
    pub fn test_be(&mut self, a: impl Into<Value>, b: impl Into<Value>) -> RegID {
        self.test(BinOp::BelowEqual, a, b)
    }

    pub fn neg(&mut self, a: impl Into<Value>) -> RegID {
        let a = a.into();
        let ty = self.ty(a);
        let reg = self.create_reg(ty);
        self.add_instr(Instruction::Unary(UnOp::Neg, reg, a));
        reg
    }
    pub fn not(&mut self, a: impl Into<Value>) -> RegID {
        let a = a.into();
        let ty = self.ty(a);
        let reg = self.create_reg(ty);
        self.add_instr(Instruction::Unary(UnOp::Not, reg, a));
        reg
    }
    pub fn int_to_ptr(&mut self, a: impl Into<Value>) -> RegID {
        let a = a.into();
        let reg = self.create_reg(Ty::Ptr);
        self.add_instr(Instruction::Unary(UnOp::IntToPtr, reg, a));
        reg
    }
    pub fn ptr_to_int(&mut self, ty: IntTy, a: impl Into<Value>) -> RegID {
        let reg = self.create_reg(ty);
        self.add_instr(Instruction::Unary(UnOp::PtrToInt, reg, a.into()));
        reg
    }
    pub fn sext(&mut self, ty: IntTy, a: impl Into<Value>) -> RegID {
        let reg = self.create_reg(ty);
        self.add_instr(Instruction::Unary(UnOp::Sext, reg, a.into()));
        reg
    }
    pub fn zext(&mut self, ty: IntTy, a: impl Into<Value>) -> RegID {
        let reg = self.create_reg(ty);
        self.add_instr(Instruction::Unary(UnOp::Zext, reg, a.into()));
        reg
    }
    pub fn trunc(&mut self, ty: IntTy, a: impl Into<Value>) -> RegID {
        let reg = self.create_reg(ty);
        self.add_instr(Instruction::Unary(UnOp::Trunc, reg, a.into()));
        reg
    }

    pub fn poison(&mut self, ty: impl Into<Ty>) -> RegID {
        let reg = self.create_reg(ty);
        self.add_instr(Instruction::Poison(reg));
        reg
    }
    pub fn select(
        &mut self,
        c: impl Into<Value>,
        a: impl Into<Value>,
        b: impl Into<Value>,
    ) -> RegID {
        let c = c.into();
        let a = a.into();
        let ty = self.ty(a);
        let b = coerce(b, ty);
        let dst = self.create_reg(ty);
        self.add_instr(Instruction::Select(dst, c, a, b));
        dst
    }
    pub fn freeze(&mut self, a: impl Into<Value>) -> RegID {
        let a = a.into();
        let ty = self.ty(a);
        let dst = self.create_reg(ty);
        self.add_instr(Instruction::Freeze(dst, a));
        dst
    }

    pub fn get_var_addr(&mut self, var: VarID) -> RegID {
        let dst = self.create_reg(Ty::Ptr);
        self.add_instr(Instruction::GetVarAddr(dst, var));
        dst
    }
    pub fn store(&mut self, ptr: RegID, value: impl Into<Value>) {
        let value = value.into();
        self.add_instr(Instruction::Store { ptr, value });
    }
    pub fn load(&mut self, ty: impl Into<Ty>, ptr: RegID) -> RegID {
        let dst = self.create_reg(ty);
        self.add_instr(Instruction::Load { dst, ptr });
        dst
    }

    pub fn jump(&mut self, tgt: impl Into<JumpTarget>) {
        self.add_instr(Instruction::Jump(tgt.into()));
    }
    pub fn branch(
        &mut self,
        c: impl Into<Value>,
        t: impl Into<JumpTarget>,
        f: impl Into<JumpTarget>,
    ) {
        self.add_instr(Instruction::Branch(c.into(), t.into(), f.into()))
    }
    pub fn ret(&mut self, value: impl Into<Value>) {
        self.add_instr(Instruction::Ret(value.into()));
    }
    pub fn call(&mut self, fun: FunID, args: impl Into<Values>) -> RegID {
        let ret_ty = self.module[fun].ret_ty;
        let dst = self.create_reg(ret_ty);
        let mut args: Values = args.into();
        for (arg, &param) in args.0.iter_mut().zip(&self.module[fun].parameters) {
            let ty = self.module[param].ty;
            *arg = coerce(*arg, ty);
        }
        self.add_instr(Instruction::Call(dst, fun, args));
        dst
    }
    pub fn call_ptr(&mut self, fun_ty: FunTyID, ptr: RegID, args: impl Into<Values>) -> RegID {
        let ret_ty = self.module[fun_ty].ret;
        let dst = self.create_reg(ret_ty);
        let mut args: Values = args.into();
        for (arg, &ty) in args.0.iter_mut().zip(&self.module[fun_ty].params) {
            *arg = coerce(*arg, ty);
        }
        self.add_instr(Instruction::CallPtr(dst, ptr, fun_ty, args));
        dst
    }

    pub fn get_struct_member(&mut self, strct: RegID, index: u64) -> RegID {
        let Ty::Struct(struct_ty) = self.module[strct].ty else {
            unreachable!()
        };
        let member = self.module[struct_ty].members[index as usize];
        let dst = self.create_reg(member);
        self.add_instr(Instruction::GetStructMember { dst, strct, index });
        dst
    }
    pub fn set_struct_member(
        &mut self,
        strct: RegID,
        index: u64,
        value: impl Into<Value>,
    ) -> RegID {
        let ty = self.module[strct].ty;
        let dst = self.create_reg(ty);
        self.add_instr(Instruction::SetStructMember {
            dst,
            strct,
            value: value.into(),
            index,
        });
        dst
    }
    pub fn get_array_element(&mut self, array: RegID, index: impl Into<Value>) -> RegID {
        let Ty::Array(arr_ty) = self.module[array].ty else {
            unreachable!()
        };
        let element = self.module[arr_ty].element;
        let dst = self.create_reg(element);
        self.add_instr(Instruction::GetArrayElement {
            dst,
            array,
            index: index.into(),
        });
        dst
    }
    pub fn index_struct(&mut self, struct_ty: StructTyID, ptr: RegID, index: u64) -> RegID {
        let dst = self.create_reg(Ty::Ptr);
        self.add_instr(Instruction::IndexStruct {
            dst,
            ptr,
            struct_ty,
            index,
        });
        dst
    }
    pub fn index_array(
        &mut self,
        element_ty: impl Into<Ty>,
        ptr: RegID,
        index: impl Into<Value>,
    ) -> RegID {
        let element_ty = element_ty.into();
        let dst = self.create_reg(Ty::Ptr);
        self.add_instr(Instruction::IndexArray {
            dst,
            ptr,
            element_ty,
            index: index.into(),
        });
        dst
    }

    pub fn syscall_x86_64(
        &mut self,
        ty: impl Into<Ty>,
        call_id: impl Into<Value>,
        args: impl Into<Values>,
    ) -> RegID {
        let dst = self.create_reg(ty);
        let mut args: Values = args.into();
        for arg in args.0.iter_mut() {
            *arg = coerce(*arg, IntTy::I64);
        }
        self.add_instr(Instruction::SyscallX86_64 {
            dst,
            call_number: call_id.into(),
            args: args.into(),
        });
        dst
    }

    fn ty(&self, v: impl Into<Value>) -> Ty {
        let v: Value = v.into();
        match v {
            Value::Reg(r) => self.module[r].ty,
            Value::Int(ty, _) => ty.into(),
            Value::Bool(_) => Ty::Bool,
            Value::Void => Ty::Void,
        }
    }
}

fn coerce(v: impl Into<Value>, to: impl Into<Ty>) -> Value {
    let v = v.into();
    let to = to.into();
    match (to, v) {
        (Ty::Int(ty), Value::Int(_, val)) => Value::Int(ty, val),
        _ => v,
    }
}
