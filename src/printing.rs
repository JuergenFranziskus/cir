use crate::{
    block::BlockID,
    function::FuncID,
    instruction::{BinaryOp, BlockTarget, ConstValue, Expr, Instruction, TestOp, UnaryOp},
    register::RegID,
    struct_type::StructTypeID,
    variable::VarID,
    Module, Type, Types,
};
use std::{
    collections::HashMap,
    io::{self, Write},
};

pub struct Printer<'a, O> {
    out: O,
    module: &'a Module,
    types: &'a mut Types,
    local_regs: HashMap<RegID, RegID>,
    reg_counter: usize,
    local_blocks: HashMap<BlockID, BlockID>,
    block_counter: usize,
    local_vars: HashMap<VarID, VarID>,
    var_counter: usize,
}
impl<'a, O> Printer<'a, O> {
    pub fn new(out: O, module: &'a Module, types: &'a mut Types) -> Self {
        Self {
            out,
            module,
            types,
            local_regs: HashMap::new(),
            reg_counter: 0,
            local_blocks: HashMap::new(),
            block_counter: 0,
            local_vars: HashMap::new(),
            var_counter: 0,
        }
    }
}
impl<'a, O: Write> Printer<'a, O> {
    pub fn pretty_print(&mut self) -> io::Result<()> {
        let max_func = self.module.functions.len();
        for i in 0..max_func {
            let id = FuncID(i);
            self.print_function(id)?;
            writeln!(self.out)?;
        }

        Ok(())
    }

    fn print_function(&mut self, fid: FuncID) -> io::Result<()> {
        self.local_regs.clear();
        self.local_blocks.clear();
        self.local_vars.clear();
        self.reg_counter = 0;
        self.block_counter = 0;
        self.var_counter = 0;

        let func = &self.module[fid];
        let name = func.name();
        let ret = func.return_type();

        write!(self.out, "fun {fid}:{name} (")?;
        for (i, &reg) in func.parameter_registers().iter().enumerate() {
            let param_type = self.module[reg].reg_type();
            self.print_type(param_type)?;

            let reg_local = self.make_reg_local(reg);
            write!(self.out, " {reg_local}")?;
            let last = i == func.parameters() - 1;
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
        let local_bid = self.make_block_local(bid);
        write!(self.out, "  {local_bid}")?;
        if let Some((&last, others)) = block.parameters().split_last() {
            write!(self.out, "(")?;
            for &other in others {
                let reg_t = self.module[other].reg_type();
                self.print_type(reg_t)?;
                let other_local = self.make_reg_local(other);
                write!(self.out, " {other_local}, ")?;
            }
            let last_t = self.module[last].reg_type();
            self.print_type(last_t)?;
            let last_local = self.make_reg_local(last);
            write!(self.out, " {last_local})")?;
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
            &MakeStruct(t, ref values) => self.print_make_struct(t, values)?,
            &MakeArray(t, ref values) => self.print_make_array(t, values)?,
            &MakeShortArray(t, ref value, length) => {
                self.print_make_short_array(t, value, length)?
            }
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
            &Load {
                target,
                pointer,
                volatile,
            } => self.print_load(target, pointer, volatile)?,
            &Store {
                pointer,
                ref value,
                volatile,
            } => self.print_store(pointer, value, volatile)?,
            Jump(block) => self.print_jump(block)?,
            Branch(c, t, f) => self.print_branch(c, t, f)?,
            &Call {
                target,
                function,
                args: ref parameters,
            } => self.print_call(target, function, parameters)?,
            &CallPtr {
                target,
                function_ptr,
                args: ref parameters,
                ..
            } => self.print_call_ptr(target, function_ptr, parameters)?,
            Return(value) => self.print_return(value)?,
        }
        writeln!(self.out)?;

        Ok(())
    }
    fn print_set(&mut self, target: RegID, value: &Expr) -> io::Result<()> {
        let target_t = self.module[target].reg_type();
        let target = self.make_reg_local(target);
        write!(self.out, "{target} = ")?;
        self.print_type(target_t)?;
        write!(self.out, " ")?;
        self.print_expr(value)?;
        Ok(())
    }
    fn print_binop(&mut self, target: RegID, op: BinaryOp, a: &Expr, b: &Expr) -> io::Result<()> {
        let target_t = self.module[target].reg_type();
        let target = self.make_reg_local(target);
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
        let target = self.make_reg_local(target);
        write!(self.out, "{target} = {op} ")?;
        self.print_type(target_t)?;
        write!(self.out, " ")?;
        self.print_expr(a)?;

        Ok(())
    }
    fn print_testop(&mut self, target: RegID, op: TestOp, a: &Expr, b: &Expr) -> io::Result<()> {
        let target = self.make_reg_local(target);
        write!(self.out, "{target} = {op} ")?;
        let a_t = a.expr_type(self.module);
        self.print_type(a_t)?;
        write!(self.out, " ")?;
        self.print_expr(a)?;
        write!(self.out, ", ")?;
        self.print_expr(b)?;

        Ok(())
    }
    fn print_make_struct(&mut self, target: RegID, values: &[Expr]) -> io::Result<()> {
        let target = self.make_reg_local(target);
        write!(self.out, "{target} = struct {{ ")?;

        if let Some((last, others)) = values.split_last() {
            for other in others {
                self.print_expr(other)?;
                write!(self.out, ", ")?;
            }
            self.print_expr(last)?;
        }
        write!(self.out, " }}")?;

        Ok(())
    }
    fn print_make_array(&mut self, target: RegID, values: &[Expr]) -> io::Result<()> {
        let target = self.make_reg_local(target);
        write!(self.out, "{target} = array [ ")?;

        if let Some((last, others)) = values.split_last() {
            for other in others {
                self.print_expr(other)?;
                write!(self.out, ", ")?;
            }
            self.print_expr(last)?;
        }
        write!(self.out, " ]")?;

        Ok(())
    }
    fn print_make_short_array(
        &mut self,
        target: RegID,
        value: &Expr,
        length: u64,
    ) -> io::Result<()> {
        let target = self.make_reg_local(target);
        write!(self.out, "{target} = array [ ")?;
        self.print_expr(value)?;
        write!(self.out, " * {length} ]")?;

        Ok(())
    }
    fn print_select(&mut self, target: RegID, c: &Expr, t: &Expr, f: &Expr) -> io::Result<()> {
        let target = self.make_reg_local(target);
        write!(self.out, "{target} = select ")?;
        self.print_expr(c)?;
        write!(self.out, " ")?;
        let a_t = t.expr_type(self.module);
        self.print_type(a_t)?;
        write!(self.out, " ")?;
        self.print_expr(t)?;
        write!(self.out, ", ")?;
        self.print_expr(f)?;

        Ok(())
    }
    fn print_get_element(&mut self, target: RegID, array: &Expr, index: &Expr) -> io::Result<()> {
        let target_t = self.module[target].reg_type();
        let target = self.make_reg_local(target);
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
        let target = self.make_reg_local(target);
        let ptr = self.make_reg_local(ptr);
        write!(self.out, "{target} = getelementptr ")?;
        self.print_type(el_t)?;
        write!(self.out, " {ptr}, ")?;
        self.print_expr(index)?;

        Ok(())
    }
    fn print_get_member(&mut self, target: RegID, structure: &Expr, index: u32) -> io::Result<()> {
        let target_t = self.module[target].reg_type();
        let target = self.make_reg_local(target);
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
        let target = self.make_reg_local(target);
        let ptr = self.make_reg_local(ptr);
        write!(self.out, "{target} = getmemberptr ")?;
        let member_t = self.types[struct_type].members()[index as usize];
        self.print_type(member_t)?;
        write!(self.out, " {ptr}, {index}")?;

        Ok(())
    }
    fn print_get_var_ptr(&mut self, target: RegID, var: VarID) -> io::Result<()> {
        let target = self.make_reg_local(target);
        let var_type = self.module[var].var_type();
        let local_var = self.make_var_local(var);
        write!(self.out, "{target} = getvarptr ")?;
        self.print_type(var_type)?;
        write!(self.out, " {local_var}")?;

        Ok(())
    }
    fn print_get_func_ptr(&mut self, target: RegID, func: FuncID) -> io::Result<()> {
        let target = self.make_reg_local(target);
        let func_name = self.module[func].name();
        write!(self.out, "{target} = getfuncptr {func}:{func_name}")?;
        Ok(())
    }
    fn print_load(&mut self, target: RegID, ptr: RegID, volatile: bool) -> io::Result<()> {
        let target_t = self.module[target].reg_type();
        let target = self.make_reg_local(target);
        let ptr = self.make_reg_local(ptr);
        write!(self.out, "{target} = load ")?;
        if volatile {
            write!(self.out, "volatile ")?;
        }

        self.print_type(target_t)?;
        write!(self.out, " {ptr}")?;
        Ok(())
    }
    fn print_store(&mut self, ptr: RegID, value: &Expr, volatile: bool) -> io::Result<()> {
        let value_t = value.expr_type(self.module);
        let ptr = self.make_reg_local(ptr);

        write!(self.out, "store ")?;
        if volatile {
            write!(self.out, "volatile ")?;
        }

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
        write!(self.out, "\n      ")?;
        self.print_block_target(t)?;
        write!(self.out, "\n      ")?;
        self.print_block_target(f)?;

        Ok(())
    }
    fn print_call(&mut self, target: RegID, func: FuncID, args: &[Expr]) -> io::Result<()> {
        let target_t = self.module[target].reg_type();
        let target = self.make_reg_local(target);
        write!(self.out, "{target} = call ")?;
        self.print_type(target_t)?;

        let func_name = self.module[func].name();
        write!(self.out, " {func}:{func_name} (")?;
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
        let target = self.make_reg_local(target);
        write!(self.out, "{target} = call ")?;
        self.print_type(target_t)?;

        write!(self.out, " {ptr} (")?;
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
        let value_t = value.expr_type(self.module);
        write!(self.out, "return ")?;
        self.print_type(value_t)?;
        write!(self.out, " ")?;
        self.print_expr(value)?;

        Ok(())
    }

    fn print_block_target(&mut self, b: &BlockTarget) -> io::Result<()> {
        let bid = self.make_block_local(b.block);
        if let Some((last, others)) = b.parameters.split_last() {
            write!(self.out, "({bid}: ")?;
            for other in others {
                self.print_expr(other)?;
                write!(self.out, ", ")?;
            }
            self.print_expr(last)?;
            write!(self.out, ")")?;
        } else {
            write!(self.out, "{bid}")?;
        }

        Ok(())
    }

    fn print_expr(&mut self, e: &Expr) -> io::Result<()> {
        use Expr::*;
        match e {
            &Register(id) => {
                let id = self.make_reg_local(id);
                write!(self.out, "{id}")?;
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
            &Bool(value) => write!(self.out, "{value}")?,
            &Integer(value, _) => write!(self.out, "{value}")?,
            &SizeOf(t, _) => {
                write!(self.out, "sizeof ")?;
                self.print_type(t)?;
            }
        }

        Ok(())
    }

    fn make_reg_local(&mut self, reg: RegID) -> RegID {
        if let Some(&reg) = self.local_regs.get(&reg) {
            reg
        } else {
            let id = RegID(self.reg_counter);
            self.reg_counter += 1;
            self.local_regs.insert(reg, id);
            id
        }
    }
    fn make_block_local(&mut self, block: BlockID) -> BlockID {
        if let Some(id) = self.local_blocks.get(&block).copied() {
            id
        } else {
            let id = BlockID(self.block_counter);
            self.block_counter += 1;
            self.local_blocks.insert(block, id);
            id
        }
    }
    fn make_var_local(&mut self, vars: VarID) -> VarID {
        if let Some(id) = self.local_vars.get(&vars).copied() {
            id
        } else {
            let id = VarID(self.var_counter);
            self.var_counter += 1;
            self.local_vars.insert(vars, id);
            id
        }
    }

    fn print_type(&mut self, t: Type) -> io::Result<()> {
        match t {
            Type::Unit => write!(self.out, "()"),
            Type::Bool => write!(self.out, "i1"),
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
