use crate::{
    block::BlockID,
    function::FuncID,
    instruction::{BinaryOp, BlockTarget, ConstValue, Expr, Instruction, TestOp, UnaryOp},
    register::RegID,
    struct_type::StructTypeID,
    variable::VarID,
    Module, Type, Types,
};
use std::io::{self, Write};

pub struct Printer<'a, O> {
    out: O,
    module: &'a Module,
    types: &'a mut Types,
}
impl<'a, O> Printer<'a, O> {
    pub fn new(out: O, module: &'a Module, types: &'a mut Types) -> Self {
        Self { out, module, types }
    }
}
impl<'a, O: Write> Printer<'a, O> {
    pub fn pretty_print(&mut self) -> io::Result<()> {
        let max_func = self.module.functions.len();
        for i in 0..max_func {
            let id = FuncID(i);
            self.print_function(id)?;
        }

        Ok(())
    }

    fn print_function(&mut self, fid: FuncID) -> io::Result<()> {
        let func = &self.module[fid];
        let name = func.name();
        let ret = func.return_type();

        write!(self.out, "fun {fid}:{name} (")?;
        for (i, param) in func.parameters().iter().enumerate() {
            let reg = param.register();
            let param_type = self.module[reg].reg_type();
            self.print_type(param_type)?;

            write!(self.out, "{reg}")?;
            let last = i == func.parameters().len() - 1;
            if !last {
                write!(self.out, ", ")?;
            }
        }
        write!(self.out, ") -> ")?;
        self.print_type(ret)?;
        writeln!(self.out)?;

        if let Some(def) = func.definition() {
            let mut blocks: Vec<_> = def.blocks().iter().copied().collect();
            blocks.sort_by_key(|id| id.0);
            for bid in blocks {
                self.print_block(bid)?;
            }
        }

        Ok(())
    }
    fn print_block(&mut self, bid: BlockID) -> io::Result<()> {
        let block = &self.module[bid];
        write!(self.out, "  {bid}")?;
        if let Some((&last, others)) = block.parameters().split_last() {
            write!(self.out, "(")?;
            for &other in others {
                let reg_t = self.module[other].reg_type();
                self.print_type(reg_t)?;
                write!(self.out, " {other}, ")?;
            }
            let last_t = self.module[last].reg_type();
            self.print_type(last_t)?;
            write!(self.out, " {last})")?;
        }
        writeln!(self.out, ":")?;

        for instruction in block.body() {
            self.print_instruction(instruction)?;
        }

        Ok(())
    }
    fn print_instruction(&mut self, inst: &Instruction) -> io::Result<()> {
        write!(self.out, "    ")?;

        use Instruction::*;
        match inst {
            Nop => write!(self.out, "nop")?,
            &Set(t, ref val) => self.print_set(t, val)?,
            &BinaryOp(t, op, ref a, ref b) => self.print_binop(t, op, a, b)?,
            &UnaryOp(t, op, ref a) => self.print_unop(t, op, a)?,
            &TestOp(t, op, ref a, ref b) => self.print_testop(t, op, a, b)?,
            &Select {
                target,
                ref condition,
                ref true_value,
                ref false_value,
            } => self.print_select(target, condition, true_value, false_value)?,
            &GetElement {
                target,
                ref array,
                ref index,
            } => self.print_get_element(target, array, index)?,
            &GetElementPtr {
                target,
                array_pointer,
                ref index,
                element_type,
            } => self.print_get_element_ptr(target, array_pointer, index, element_type)?,
            &GetMember {
                target,
                ref structure,
                index,
            } => self.print_get_member(target, structure, index)?,
            &GetMemberPtr {
                target,
                struct_pointer,
                member,
                struct_type,
            } => self.print_get_member_ptr(target, struct_pointer, member, struct_type)?,
            &GetVarPointer(target, var) => self.print_get_var_ptr(target, var)?,
            &GetFunctionPointer(target, func) => self.print_get_func_ptr(target, func)?,
            &Load { target, pointer } => self.print_load(target, pointer)?,
            &Store { pointer, ref value } => self.print_store(pointer, value)?,
            Jump(block) => self.print_jump(block)?,
            Branch(c, t, f) => self.print_branch(c, t, f)?,
            &Call {
                target,
                function,
                ref parameters,
            } => self.print_call(target, function, parameters)?,
            &CallPtr {
                target,
                function_ptr,
                ref parameters,
                ..
            } => self.print_call_ptr(target, function_ptr, parameters)?,
            Return(value) => self.print_return(value)?,
        }
        writeln!(self.out)?;

        Ok(())
    }
    fn print_set(&mut self, target: RegID, value: &Expr) -> io::Result<()> {
        let target_t = self.module[target].reg_type();
        write!(self.out, "{target} = ")?;
        self.print_type(target_t)?;
        write!(self.out, " ")?;
        self.print_expr(value)?;
        Ok(())
    }
    fn print_binop(&mut self, target: RegID, op: BinaryOp, a: &Expr, b: &Expr) -> io::Result<()> {
        let target_t = self.module[target].reg_type();
        write!(self.out, "{target} = {op} ")?;
        self.print_type(target_t)?;
        write!(self.out, " ")?;
        self.print_expr(a)?;
        write!(self.out, ", ")?;
        self.print_expr(b)?;

        Ok(())
    }
    fn print_unop(&mut self, target: RegID, op: UnaryOp, a: &Expr) -> io::Result<()> {
        let target_t = self.module[target].reg_type();
        write!(self.out, "{target} = {op} ")?;
        self.print_type(target_t)?;
        write!(self.out, " ")?;
        self.print_expr(a)?;

        Ok(())
    }
    fn print_testop(&mut self, target: RegID, op: TestOp, a: &Expr, b: &Expr) -> io::Result<()> {
        write!(self.out, "{target} = {op} ")?;
        let a_t = a.expr_type(self.module, self.types);
        self.print_type(a_t)?;
        write!(self.out, " ")?;
        self.print_expr(a)?;
        write!(self.out, ", ")?;
        self.print_expr(b)?;

        Ok(())
    }
    fn print_select(&mut self, target: RegID, c: &Expr, t: &Expr, f: &Expr) -> io::Result<()> {
        write!(self.out, "{target} = select ")?;
        self.print_expr(c)?;
        write!(self.out, " ")?;
        let a_t = t.expr_type(self.module, self.types);
        self.print_type(a_t)?;
        write!(self.out, " ")?;
        self.print_expr(t)?;
        write!(self.out, ", ")?;
        self.print_expr(f)?;

        Ok(())
    }
    fn print_get_element(&mut self, target: RegID, array: &Expr, index: &Expr) -> io::Result<()> {
        let target_t = self.module[target].reg_type();
        write!(self.out, "{target} = getelement ")?;
        self.print_type(target_t)?;
        write!(self.out, " ")?;

        self.print_expr(array)?;
        write!(self.out, ", ")?;
        self.print_expr(index)?;

        Ok(())
    }
    fn print_get_element_ptr(
        &mut self,
        target: RegID,
        ptr: RegID,
        index: &Expr,
        el_t: Type,
    ) -> io::Result<()> {
        write!(self.out, "{target} = getelementptr ")?;
        self.print_type(el_t)?;
        write!(self.out, " {ptr}, ")?;
        self.print_expr(index)?;

        Ok(())
    }
    fn print_get_member(&mut self, target: RegID, structure: &Expr, index: u32) -> io::Result<()> {
        let target_t = self.module[target].reg_type();
        write!(self.out, "{target} = getmember ")?;
        self.print_type(target_t)?;
        write!(self.out, " ")?;

        self.print_expr(structure)?;
        write!(self.out, ", {index}")?;

        Ok(())
    }
    fn print_get_member_ptr(
        &mut self,
        target: RegID,
        ptr: RegID,
        index: u32,
        struct_type: StructTypeID,
    ) -> io::Result<()> {
        write!(self.out, "{target} = getmemberptr ")?;
        let member_t = self.types[struct_type].members()[index as usize];
        self.print_type(member_t)?;
        write!(self.out, " {ptr}, ")?;
        write!(self.out, ", {index}")?;

        Ok(())
    }
    fn print_get_var_ptr(&mut self, target: RegID, var: VarID) -> io::Result<()> {
        let var_type = self.module[var].var_type();
        write!(self.out, "{target} = getvarptr ")?;
        self.print_type(var_type)?;
        write!(self.out, " {var}")?;

        Ok(())
    }
    fn print_get_func_ptr(&mut self, target: RegID, func: FuncID) -> io::Result<()> {
        write!(self.out, "{target} = getfuncptr {func}")?;
        Ok(())
    }
    fn print_load(&mut self, target: RegID, ptr: RegID) -> io::Result<()> {
        let target_t = self.module[target].reg_type();
        write!(self.out, "{target} = load ")?;
        self.print_type(target_t)?;
        write!(self.out, " {ptr}")?;
        Ok(())
    }
    fn print_store(&mut self, ptr: RegID, value: &Expr) -> io::Result<()> {
        let value_t = value.expr_type(self.module, self.types);

        write!(self.out, "store ")?;
        self.print_type(value_t)?;
        write!(self.out, " ")?;
        self.print_expr(value)?;
        write!(self.out, " into {ptr}")?;

        Ok(())
    }
    fn print_jump(&mut self, block: &BlockTarget) -> io::Result<()> {
        write!(self.out, "jump ")?;
        self.print_block_target(block)
    }
    fn print_branch(&mut self, c: &Expr, t: &BlockTarget, f: &BlockTarget) -> io::Result<()> {
        write!(self.out, "branch ")?;
        self.print_expr(c)?;
        write!(self.out, " ")?;
        self.print_block_target(t)?;
        write!(self.out, " ")?;
        self.print_block_target(f)?;

        Ok(())
    }
    fn print_call(&mut self, target: RegID, func: FuncID, args: &[Expr]) -> io::Result<()> {
        let target_t = self.module[target].reg_type();
        write!(self.out, "{target} = call ")?;
        self.print_type(target_t)?;

        write!(self.out, "{func} (")?;
        if let Some((last, others)) = args.split_last() {
            for other in others {
                self.print_expr(other)?;
                write!(self.out, ", ")?;
            }
            self.print_expr(last)?;
        }
        write!(self.out, ")")?;

        Ok(())
    }
    fn print_call_ptr(&mut self, target: RegID, ptr: RegID, args: &[Expr]) -> io::Result<()> {
        let target_t = self.module[target].reg_type();
        write!(self.out, "{target} = call ")?;
        self.print_type(target_t)?;

        write!(self.out, "{ptr} (")?;
        if let Some((last, others)) = args.split_last() {
            for other in others {
                self.print_expr(other)?;
                write!(self.out, ", ")?;
            }
            self.print_expr(last)?;
        }
        write!(self.out, ")")?;

        Ok(())
    }
    fn print_return(&mut self, value: &Expr) -> io::Result<()> {
        let value_t = value.expr_type(self.module, self.types);
        write!(self.out, "return ")?;
        self.print_type(value_t)?;
        write!(self.out, " ")?;
        self.print_expr(value)?;

        Ok(())
    }

    fn print_block_target(&mut self, b: &BlockTarget) -> io::Result<()> {
        if let Some((last, others)) = b.parameters.split_last() {
            write!(self.out, "({}: ", b.block)?;
            for other in others {
                self.print_expr(other)?;
                write!(self.out, ", ")?;
            }
            self.print_expr(last)?;
            write!(self.out, ")")?;
        } else {
            write!(self.out, "{}", b.block)?;
        }

        Ok(())
    }

    fn print_expr(&mut self, e: &Expr) -> io::Result<()> {
        use Expr::*;
        match e {
            &Register(id) => write!(self.out, "{id}")?,
            Array(elements) => {
                write!(self.out, "[ ")?;
                if let Some((last, others)) = elements.split_last() {
                    for other in others {
                        self.print_expr(other)?;
                        write!(self.out, ", ")?;
                    }
                    self.print_expr(last)?;
                }
                write!(self.out, " ]")?;
            }
            &ShortArray(ref element, size) => {
                write!(self.out, "[ ")?;
                self.print_expr(element)?;
                write!(self.out, " * {size} ]")?;
            }
            Struct(members) => {
                write!(self.out, "{{ ")?;
                if let Some((last, others)) = members.split_last() {
                    for other in others {
                        self.print_expr(other)?;
                        write!(self.out, ", ")?;
                    }
                    self.print_expr(last)?;
                }
                write!(self.out, " }}")?;
            }
            Constant(c) => self.print_const_val(c)?,
        }

        Ok(())
    }
    fn print_const_val(&mut self, val: &ConstValue) -> io::Result<()> {
        use ConstValue::*;
        match val {
            Poison(_) => write!(self.out, "poison")?,
            Unit => write!(self.out, "()")?,
            NullPtr => write!(self.out, "nullptr")?,
            &Integer(value, _) => write!(self.out, "{value}")?,
        }

        Ok(())
    }

    fn print_type(&mut self, t: Type) -> io::Result<()> {
        match t {
            Type::Unit => write!(self.out, "()"),
            Type::Integer(size) => write!(self.out, "i{}", size.bits()),
            Type::Pointer => write!(self.out, "ptr"),
            Type::Array(id) => {
                let element = self.types[id].member();
                let length = self.types[id].length();
                write!(self.out, "[ ")?;
                self.print_type(element)?;
                write!(self.out, " * {length} ]")
            }
            Type::Struct(id) => {
                let members = self.types[id].members().to_vec();
                write!(self.out, "{{ ")?;
                if let Some((&last, others)) = members.split_last() {
                    for &other in others {
                        self.print_type(other)?;
                        write!(self.out, ", ")?;
                    }
                    self.print_type(last)?;
                }
                write!(self.out, " }}")
            }
        }
    }
}
