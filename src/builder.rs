use crate::{
    module::{
        block::BlockID,
        calling_convention::CallingConvention,
        function::FuncID,
        instruction::{BinaryOp, BlockTarget, Expr, Instruction, TestOp, UnaryOp},
        register::RegID,
        variable::VarID,
        Module,
    },
    struct_type::StructTypeID,
    types::{IntegerSize, Type, Types},
};

pub struct Builder {
    module: Module,
    types: Types,
    at_block: Option<BlockID>,
}
impl Builder {
    pub fn new(module: Module, types: Types) -> Self {
        Self {
            module,
            types,
            at_block: None,
        }
    }
    pub fn finish(self) -> (Module, Types) {
        (self.module, self.types)
    }

    pub fn module(&self) -> &Module {
        &self.module
    }
    pub fn module_mut(&mut self) -> &mut Module {
        &mut self.module
    }
    pub fn types(&self) -> &Types {
        &self.types
    }
    pub fn types_mut(&mut self) -> &mut Types {
        &mut self.types
    }

    pub fn select_block(&mut self, block: BlockID) {
        self.at_block = Some(block);
    }
    pub fn unselect_block(&mut self) {
        self.at_block = None;
    }
    pub fn selected_function(&self) -> Option<FuncID> {
        self.at_block.map(|b| self.module[b].function())
    }

    pub fn create_function(
        &mut self,
        name: impl Into<String>,
        return_type: impl Into<Type>,
    ) -> FuncID {
        self.module.add_function(name, return_type)
    }
    pub fn start_function(&mut self, fid: FuncID) {
        let entry = self.module.start_function_definition(fid);
        self.select_block(entry);
    }

    pub fn add_func_param(&mut self, param_type: impl Into<Type>) -> RegID {
        let fid = self.selected_function().unwrap();
        self.module.add_parameter(fid, param_type)
    }
    pub fn add_block_param(&mut self, param_type: impl Into<Type>) -> RegID {
        let block = self.at_block.unwrap();
        self.module.add_block_parameter(block, param_type)
    }
    pub fn add_register(&mut self, reg_type: impl Into<Type>) -> RegID {
        let fid = self.selected_function().unwrap();
        self.module.add_register(fid, reg_type)
    }
    pub fn add_variable(&mut self, var_type: impl Into<Type>) -> VarID {
        let fid = self.selected_function().unwrap();
        self.module.add_variable(fid, var_type)
    }
    pub fn add_block(&mut self) -> BlockID {
        let fid = self.selected_function().unwrap();
        self.module.add_block(fid)
    }
    fn push_instruction(&mut self, i: Instruction) {
        let block = self.at_block.unwrap();
        self.module.push_instruction(block, i);
    }

    pub fn nop(&mut self) {
        self.push_instruction(Instruction::Nop);
    }
    pub fn set(&mut self, to: impl Into<Expr>) -> RegID {
        let to = to.into();
        let target_type = self.expr_type(&to);
        let target = self.add_register(target_type);
        self.push_instruction(Instruction::Set(target, to));
        target
    }
    pub fn add(&mut self, a: impl Into<Expr>, b: impl Into<Expr>) -> RegID {
        let a = a.into();
        let b = b.into();
        let at = self.expr_type(&a);
        let bt = self.expr_type(&b);
        let Type::Integer(ai) = at else { panic!() };
        let Type::Integer(bi) = bt else { panic!() };
        assert!(ai == bi);

        let target = self.add_register(ai);
        self.push_instruction(Instruction::BinaryOp(target, BinaryOp::Add, a, b));
        target
    }
    pub fn sub(&mut self, a: impl Into<Expr>, b: impl Into<Expr>) -> RegID {
        let a = a.into();
        let b = b.into();
        let at = self.expr_type(&a);
        let bt = self.expr_type(&b);
        let Type::Integer(ai) = at else { panic!() };
        let Type::Integer(bi) = bt else { panic!() };
        assert!(ai == bi);

        let target = self.add_register(ai);
        self.push_instruction(Instruction::BinaryOp(target, BinaryOp::Sub, a, b));
        target
    }
    pub fn mul(&mut self, a: impl Into<Expr>, b: impl Into<Expr>) -> RegID {
        let a = a.into();
        let b = b.into();
        let at = self.expr_type(&a);
        let bt = self.expr_type(&b);
        let Type::Integer(ai) = at else { panic!() };
        let Type::Integer(bi) = bt else { panic!() };
        assert!(ai == bi);

        let target = self.add_register(ai);
        self.push_instruction(Instruction::BinaryOp(target, BinaryOp::Mul, a, b));
        target
    }
    pub fn udiv(&mut self, a: impl Into<Expr>, b: impl Into<Expr>) -> RegID {
        let a = a.into();
        let b = b.into();
        let at = self.expr_type(&a);
        let bt = self.expr_type(&b);
        let Type::Integer(ai) = at else { panic!() };
        let Type::Integer(bi) = bt else { panic!() };
        assert!(ai == bi);

        let target = self.add_register(ai);
        self.push_instruction(Instruction::BinaryOp(target, BinaryOp::UDiv, a, b));
        target
    }
    pub fn idiv(&mut self, a: impl Into<Expr>, b: impl Into<Expr>) -> RegID {
        let a = a.into();
        let b = b.into();
        let at = self.expr_type(&a);
        let bt = self.expr_type(&b);
        let Type::Integer(ai) = at else { panic!() };
        let Type::Integer(bi) = bt else { panic!() };
        assert!(ai == bi);

        let target = self.add_register(ai);
        self.push_instruction(Instruction::BinaryOp(target, BinaryOp::IDiv, a, b));
        target
    }
    pub fn shl(&mut self, a: impl Into<Expr>, b: impl Into<Expr>) -> RegID {
        let a = a.into();
        let b = b.into();
        let at = self.expr_type(&a);
        let bt = self.expr_type(&b);
        let Type::Integer(ai) = at else { panic!() };
        let Type::Integer(bi) = bt else { panic!() };
        assert!(ai == bi);

        let target = self.add_register(ai);
        self.push_instruction(Instruction::BinaryOp(target, BinaryOp::ShiftLeft, a, b));
        target
    }
    pub fn shr(&mut self, a: impl Into<Expr>, b: impl Into<Expr>) -> RegID {
        let a = a.into();
        let b = b.into();
        let at = self.expr_type(&a);
        let bt = self.expr_type(&b);
        let Type::Integer(ai) = at else { panic!() };
        let Type::Integer(bi) = bt else { panic!() };
        assert!(ai == bi);

        let target = self.add_register(ai);
        self.push_instruction(Instruction::BinaryOp(
            target,
            BinaryOp::ShiftLogicalRight,
            a,
            b,
        ));
        target
    }
    pub fn sar(&mut self, a: impl Into<Expr>, b: impl Into<Expr>) -> RegID {
        let a = a.into();
        let b = b.into();
        let at = self.expr_type(&a);
        let bt = self.expr_type(&b);
        let Type::Integer(ai) = at else { panic!() };
        let Type::Integer(bi) = bt else { panic!() };
        assert!(ai == bi);

        let target = self.add_register(ai);
        self.push_instruction(Instruction::BinaryOp(
            target,
            BinaryOp::ShiftArithmeticRight,
            a,
            b,
        ));
        target
    }
    pub fn and(&mut self, a: impl Into<Expr>, b: impl Into<Expr>) -> RegID {
        let a = a.into();
        let b = b.into();
        let at = self.expr_type(&a);
        let bt = self.expr_type(&b);
        let Type::Integer(ai) = at else { panic!() };
        let Type::Integer(bi) = bt else { panic!() };
        assert!(ai == bi);

        let target = self.add_register(ai);
        self.push_instruction(Instruction::BinaryOp(target, BinaryOp::And, a, b));
        target
    }
    pub fn nand(&mut self, a: impl Into<Expr>, b: impl Into<Expr>) -> RegID {
        let a = a.into();
        let b = b.into();
        let at = self.expr_type(&a);
        let bt = self.expr_type(&b);
        let Type::Integer(ai) = at else { panic!() };
        let Type::Integer(bi) = bt else { panic!() };
        assert!(ai == bi);

        let target = self.add_register(ai);
        self.push_instruction(Instruction::BinaryOp(target, BinaryOp::Nand, a, b));
        target
    }
    pub fn or(&mut self, a: impl Into<Expr>, b: impl Into<Expr>) -> RegID {
        let a = a.into();
        let b = b.into();
        let at = self.expr_type(&a);
        let bt = self.expr_type(&b);
        let Type::Integer(ai) = at else { panic!() };
        let Type::Integer(bi) = bt else { panic!() };
        assert!(ai == bi);

        let target = self.add_register(ai);
        self.push_instruction(Instruction::BinaryOp(target, BinaryOp::Or, a, b));
        target
    }
    pub fn nor(&mut self, a: impl Into<Expr>, b: impl Into<Expr>) -> RegID {
        let a = a.into();
        let b = b.into();
        let at = self.expr_type(&a);
        let bt = self.expr_type(&b);
        let Type::Integer(ai) = at else { panic!() };
        let Type::Integer(bi) = bt else { panic!() };
        assert!(ai == bi);

        let target = self.add_register(ai);
        self.push_instruction(Instruction::BinaryOp(target, BinaryOp::Nor, a, b));
        target
    }
    pub fn xor(&mut self, a: impl Into<Expr>, b: impl Into<Expr>) -> RegID {
        let a = a.into();
        let b = b.into();
        let at = self.expr_type(&a);
        let bt = self.expr_type(&b);
        let Type::Integer(ai) = at else { panic!() };
        let Type::Integer(bi) = bt else { panic!() };
        assert!(ai == bi);

        let target = self.add_register(ai);
        self.push_instruction(Instruction::BinaryOp(target, BinaryOp::Xor, a, b));
        target
    }
    pub fn xnor(&mut self, a: impl Into<Expr>, b: impl Into<Expr>) -> RegID {
        let a = a.into();
        let b = b.into();
        let at = self.expr_type(&a);
        let bt = self.expr_type(&b);
        let Type::Integer(ai) = at else { panic!() };
        let Type::Integer(bi) = bt else { panic!() };
        assert!(ai == bi);

        let target = self.add_register(ai);
        self.push_instruction(Instruction::BinaryOp(target, BinaryOp::XNor, a, b));
        target
    }
    pub fn not(&mut self, a: impl Into<Expr>) -> RegID {
        let a = a.into();
        let at = self.expr_type(&a);
        let Type::Integer(ai) = at else { panic!() };

        let target = self.add_register(ai);
        self.push_instruction(Instruction::UnaryOp(target, UnaryOp::Not, a));
        target
    }
    pub fn neg(&mut self, a: impl Into<Expr>) -> RegID {
        let a = a.into();
        let at = self.expr_type(&a);
        let Type::Integer(ai) = at else { panic!() };

        let target = self.add_register(ai);
        self.push_instruction(Instruction::UnaryOp(target, UnaryOp::Neg, a));
        target
    }
    pub fn freeze(&mut self, a: impl Into<Expr>) -> RegID {
        let a = a.into();
        let at = self.expr_type(&a);

        let target = self.add_register(at);
        self.push_instruction(Instruction::UnaryOp(target, UnaryOp::Freeze, a));
        target
    }
    pub fn trunc(&mut self, a: impl Into<Expr>, to: impl Into<IntegerSize>) -> RegID {
        let to = to.into();
        let a = a.into();
        let at = self.expr_type(&a);
        let Type::Integer(ai) = at else { panic!() };
        assert!(to < ai);

        let target = self.add_register(to);
        self.push_instruction(Instruction::UnaryOp(target, UnaryOp::Truncate, a));
        target
    }
    pub fn sext(&mut self, a: impl Into<Expr>, to: impl Into<IntegerSize>) -> RegID {
        let to = to.into();
        let a = a.into();
        let at = self.expr_type(&a);
        let Type::Integer(ai) = at else { panic!() };
        assert!(to > ai);

        let target = self.add_register(to);
        self.push_instruction(Instruction::UnaryOp(target, UnaryOp::SignExtend, a));
        target
    }
    pub fn zext(&mut self, a: impl Into<Expr>, to: impl Into<IntegerSize>) -> RegID {
        let to = to.into();
        let a = a.into();
        let at = self.expr_type(&a);
        let Type::Integer(ai) = at else { panic!() };
        assert!(to > ai);

        let target = self.add_register(to);
        self.push_instruction(Instruction::UnaryOp(target, UnaryOp::ZeroExtend, a));
        target
    }
    pub fn int_to_ptr(&mut self, a: impl Into<Expr>) -> RegID {
        let a = a.into();
        let at = self.expr_type(&a);
        let Type::Integer(_ai) = at else { panic!() };

        let target = self.add_register(Type::Pointer);
        self.push_instruction(Instruction::UnaryOp(target, UnaryOp::IntToPtr, a));
        target
    }
    pub fn ptr_to_int(&mut self, a: impl Into<Expr>, to: impl Into<IntegerSize>) -> RegID {
        let to = to.into();
        let a = a.into();
        let at = self.expr_type(&a);
        let Type::Pointer = at else { panic!() };

        let target = self.add_register(to);
        self.push_instruction(Instruction::UnaryOp(target, UnaryOp::PtrToInt, a));
        target
    }

    pub fn test_equal(&mut self, a: impl Into<Expr>, b: impl Into<Expr>) -> RegID {
        let a = a.into();
        let b = b.into();

        let target = self.add_register(Type::integer(1));
        self.push_instruction(Instruction::TestOp(target, TestOp::Equal, a, b));
        target
    }
    pub fn test_not_equal(&mut self, a: impl Into<Expr>, b: impl Into<Expr>) -> RegID {
        let a = a.into();
        let b = b.into();

        let target = self.add_register(Type::integer(1));
        self.push_instruction(Instruction::TestOp(target, TestOp::NotEqual, a, b));
        target
    }
    pub fn test_greater(&mut self, a: impl Into<Expr>, b: impl Into<Expr>) -> RegID {
        let a = a.into();
        let b = b.into();

        let target = self.add_register(Type::integer(1));
        self.push_instruction(Instruction::TestOp(target, TestOp::Greater, a, b));
        target
    }
    pub fn test_greater_equal(&mut self, a: impl Into<Expr>, b: impl Into<Expr>) -> RegID {
        let a = a.into();
        let b = b.into();

        let target = self.add_register(Type::integer(1));
        self.push_instruction(Instruction::TestOp(target, TestOp::GreaterEqual, a, b));
        target
    }
    pub fn test_less(&mut self, a: impl Into<Expr>, b: impl Into<Expr>) -> RegID {
        let a = a.into();
        let b = b.into();

        let target = self.add_register(Type::integer(1));
        self.push_instruction(Instruction::TestOp(target, TestOp::Less, a, b));
        target
    }
    pub fn test_less_equal(&mut self, a: impl Into<Expr>, b: impl Into<Expr>) -> RegID {
        let a = a.into();
        let b = b.into();

        let target = self.add_register(Type::integer(1));
        self.push_instruction(Instruction::TestOp(target, TestOp::LessEqual, a, b));
        target
    }
    pub fn test_above(&mut self, a: impl Into<Expr>, b: impl Into<Expr>) -> RegID {
        let a = a.into();
        let b = b.into();

        let target = self.add_register(Type::integer(1));
        self.push_instruction(Instruction::TestOp(target, TestOp::Above, a, b));
        target
    }
    pub fn test_above_equal(&mut self, a: impl Into<Expr>, b: impl Into<Expr>) -> RegID {
        let a = a.into();
        let b = b.into();

        let target = self.add_register(Type::integer(1));
        self.push_instruction(Instruction::TestOp(target, TestOp::AboveEqual, a, b));
        target
    }
    pub fn test_below(&mut self, a: impl Into<Expr>, b: impl Into<Expr>) -> RegID {
        let a = a.into();
        let b = b.into();

        let target = self.add_register(Type::integer(1));
        self.push_instruction(Instruction::TestOp(target, TestOp::Below, a, b));
        target
    }
    pub fn test_below_equal(&mut self, a: impl Into<Expr>, b: impl Into<Expr>) -> RegID {
        let a = a.into();
        let b = b.into();

        let target = self.add_register(Type::integer(1));
        self.push_instruction(Instruction::TestOp(target, TestOp::BelowEqual, a, b));
        target
    }

    pub fn select(&mut self, c: impl Into<Expr>, t: impl Into<Expr>, f: impl Into<Expr>) -> RegID {
        let c = c.into();
        let t = t.into();
        let f = f.into();

        let tt = self.expr_type(&t);
        let target = self.add_register(tt);
        self.push_instruction(Instruction::Select {
            target,
            condition: c,
            true_value: t,
            false_value: f,
        });
        target
    }
    pub fn get_element(&mut self, array: impl Into<Expr>, index: impl Into<Expr>) -> RegID {
        let array = array.into();
        let index = index.into();
        let Type::Array(id) = self.expr_type(&array) else { panic!() };
        let element_type = self.types[id].member();

        let target = self.add_register(element_type);
        self.push_instruction(Instruction::GetElement {
            target,
            array,
            index,
        });
        target
    }
    pub fn get_element_ptr(
        &mut self,
        arr_ptr: RegID,
        index: impl Into<Expr>,
        element_type: impl Into<Type>,
    ) -> RegID {
        let index = index.into();
        let member_type = element_type.into();

        let target = self.add_register(Type::Pointer);
        self.push_instruction(Instruction::GetElementPtr {
            target,
            array_pointer: arr_ptr,
            index,
            element_type: member_type,
        });
        target
    }
    pub fn get_member(&mut self, structure: impl Into<Expr>, member: u32) -> RegID {
        let structure = structure.into();
        let Type::Struct(id) = self.expr_type(&structure) else { panic!() };
        let member_type = self.types[id].members()[member as usize];

        let target = self.add_register(member_type);
        self.push_instruction(Instruction::GetMember {
            target,
            structure,
            index: member,
        });
        target
    }
    pub fn get_member_ptr(
        &mut self,
        struct_pointer: RegID,
        member: u32,
        struct_type: StructTypeID,
    ) -> RegID {
        let member_type = self.types[struct_type].members()[member as usize];

        let target = self.add_register(member_type);
        self.push_instruction(Instruction::GetMemberPtr {
            target,
            struct_pointer,
            member,
            struct_type,
        });
        target
    }

    pub fn get_var_ptr(&mut self, var: VarID) -> RegID {
        let target = self.add_register(Type::Pointer);
        self.push_instruction(Instruction::GetVarPointer(target, var));
        target
    }
    pub fn get_func_ptr(&mut self, func: FuncID) -> RegID {
        let target = self.add_register(Type::Pointer);
        self.push_instruction(Instruction::GetFunctionPointer(target, func));
        target
    }
    pub fn load(&mut self, ptr: RegID, pointee_type: impl Into<Type>) -> RegID {
        let target = self.add_register(pointee_type);
        self.push_instruction(Instruction::Load {
            target,
            pointer: ptr,
        });
        target
    }
    pub fn store(&mut self, ptr: RegID, value: impl Into<Expr>) {
        let value = value.into();
        self.push_instruction(Instruction::Store {
            pointer: ptr,
            value,
        });
    }

    pub fn jump(&mut self, target: impl Into<BlockTarget>) {
        self.push_instruction(Instruction::Jump(target.into()));
    }
    pub fn branch(
        &mut self,
        c: impl Into<Expr>,
        true_branch: impl Into<BlockTarget>,
        false_branch: impl Into<BlockTarget>,
    ) {
        self.push_instruction(Instruction::Branch(
            c.into(),
            true_branch.into(),
            false_branch.into(),
        ));
    }
    pub fn call(&mut self, func: FuncID, args: Vec<Expr>) -> RegID {
        let ret_type = self.module[func].return_type();
        let target = self.add_register(ret_type);
        self.push_instruction(Instruction::Call {
            target,
            function: func,
            parameters: args,
        });
        target
    }
    pub fn call_ptr(
        &mut self,
        func_ptr: RegID,
        ret_type: impl Into<Type>,
        args: Vec<Expr>,
        convention: CallingConvention,
    ) -> RegID {
        let target = self.add_register(ret_type);
        self.push_instruction(Instruction::CallPtr {
            target,
            function_ptr: func_ptr,
            parameters: args,
            calling_convention: convention,
        });
        target
    }
    pub fn do_return(&mut self, value: impl Into<Expr>) {
        self.push_instruction(Instruction::Return(value.into()));
    }

    fn expr_type(&mut self, e: &Expr) -> Type {
        match e {
            &Expr::Register(id) => self.module[id].reg_type(),
            Expr::Struct(members) => {
                let members = members.iter().map(|m| self.expr_type(m)).collect();
                self.types.make_struct(members).into()
            }
            &Expr::ShortArray(ref element, length) => {
                let element_type = self.expr_type(element);
                self.types.make_array(element_type, length).into()
            }
            Expr::Array(elements) => {
                let element_type = self.expr_type(&elements[0]);
                let length = elements.len() as u64;
                self.types.make_array(element_type, length).into()
            }
            Expr::Constant(v) => v.expr_type(),
        }
    }
}
