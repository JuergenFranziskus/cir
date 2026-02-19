use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::{self, Write},
};

use super::*;

pub struct Printer<'a, O> {
    module: &'a Module,
    out: O,

    reg_names: HashMap<RegID, usize>,
    var_names: HashMap<VarID, usize>,
    block_names: HashMap<BlockID, usize>,
}
impl<'a, O: Write> Printer<'a, O> {
    pub fn new(module: &'a Module, out: O) -> Self {
        Self {
            module,
            out,

            reg_names: HashMap::new(),
            var_names: HashMap::new(),
            block_names: HashMap::new(),
        }
    }

    pub fn print(&mut self) -> io::Result<()> {
        for fun in self.module.functions() {
            self.print_function(fun)?;
            writeln!(self.out)?;
        }

        Ok(())
    }

    fn print_function(&mut self, fun: &'a Function) -> io::Result<()> {
        self.clear_names();

        write!(self.out, "fun {} ", fun.name)?;
        self.print_ty(fun.ret_ty)?;
        write!(self.out, "( ")?;
        for (i, &param) in fun.parameters.iter().enumerate() {
            self.print_ty(self.module[param].ty)?;
            write!(self.out, " ")?;
            self.print_reg(param)?;
            if i != fun.parameters.len() - 1 {
                write!(self.out, ", ")?;
            }
        }
        write!(self.out, " )")?;

        let Some(entry) = fun.entry_block else {
            writeln!(self.out, ";")?;
            return Ok(());
        };

        writeln!(self.out, " {{")?;
        let mut blocks = VecDeque::new();
        blocks.push_back(entry);
        let mut done = HashSet::new();
        let mut to_insert = Vec::with_capacity(2);

        while let Some(block) = blocks.pop_front() {
            done.insert(block);
            write!(self.out, "  ")?;
            self.print_block_id(block)?;
            let params = &self.module[block].parameters;
            if !params.is_empty() {
                self.print_regs("( ", params, " )")?;
            }
            writeln!(self.out, ":")?;

            let instructions = &self.module[block].instructions;
            for instruction in instructions {
                self.print_instr(instruction)?;
                if let Some(next) = instruction.next_blocks() {
                    to_insert.push(next.0);
                    if let Some(next) = next.1 {
                        to_insert.push(next)
                    }
                    break;
                }
            }

            writeln!(self.out)?;

            for to_insert in to_insert.drain(..) {
                if !done.contains(&to_insert) && !blocks.contains(&to_insert) {
                    blocks.push_back(to_insert);
                }
            }
        }

        Ok(())
    }
    fn clear_names(&mut self) {
        self.reg_names.clear();
        self.var_names.clear();
        self.block_names.clear();
    }

    fn print_instr(&mut self, instr: &Instruction) -> io::Result<()> {
        use Instruction::*;

        write!(self.out, "    ")?;
        match *instr {
            Set(dst, to) => self.print_set(dst, to)?,
            SetFunPtr(dst, fid) => self.print_set_fun_ptr(dst, fid)?,
            SetStruct(dst, ref values) => self.print_set_struct(dst, values)?,
            SetArray(dst, ref values) => self.print_set_array(dst, values)?,
            SetArraySplat(dst, value) => self.print_set_array_splat(dst, value)?,
            Binary(op, dst, a, b) => self.print_binary(op, dst, a, b)?,
            Unary(op, dst, a) => self.print_unary(op, dst, a)?,

            Poison(dst) => {
                self.print_assign(dst)?;
                write!(self.out, "poison")?;
            }
            Select(dst, c, a, b) => {
                self.print_assign(dst)?;
                write!(self.out, "select ")?;
                self.print_value(c)?;
                write!(self.out, ", ")?;
                self.print_value(a)?;
                write!(self.out, ", ")?;
                self.print_value(b)?;
            }
            Freeze(dst, a) => {
                self.print_assign(dst)?;
                write!(self.out, "freeze ")?;
                self.print_value(a)?;
            }

            GetVarAddr(dst, vid) => {
                self.print_assign(dst)?;
                write!(self.out, "get_var_addr ")?;
                self.print_var(vid)?;
            }
            Store { ptr, value } => {
                write!(self.out, "store ")?;
                self.print_reg(ptr)?;
                write!(self.out, ", ")?;
                self.print_value(value)?;
            }
            Load { dst, ptr } => {
                self.print_assign(dst)?;
                write!(self.out, "load ")?;
                self.print_reg(ptr)?;
            }

            Jump(ref tgt) => {
                write!(self.out, "jump ")?;
                self.print_jump_tgt(tgt)?;
            }
            Branch(c, ref t, ref f) => {
                write!(self.out, "branch ")?;
                self.print_value(c)?;
                write!(self.out, ", ")?;
                self.print_jump_tgt(t)?;
                write!(self.out, ", ")?;
                self.print_jump_tgt(f)?;
            }
            Call(dst, fid, ref args) => {
                self.print_assign(dst)?;
                let name = &self.module[fid].name;
                write!(self.out, "call {name}")?;
                self.print_values("( ", args, " )")?;
            }
            CallPtr(dst, ptr, _fun_ty, ref args) => {
                self.print_assign(dst)?;
                write!(self.out, "call ")?;
                self.print_reg(ptr)?;
                self.print_values("( ", args, " )")?;
            }
            Ret(value) => self.print_ret(value)?,
            IndexArray {
                dst,
                ptr,
                element_ty,
                index,
            } => self.print_index_array(dst, ptr, element_ty, index)?,
            SyscallX86_64 {
                dst,
                call_number,
                ref args,
            } => self.print_syscall_x86_64(dst, call_number, &args)?,
            ref or => write!(self.out, "UNPRINTABLE {or:?}")?,
        }

        writeln!(self.out)?;

        Ok(())
    }
    fn print_set(&mut self, dst: RegID, to: Value) -> io::Result<()> {
        self.print_assign(dst)?;
        self.print_value(to)?;
        Ok(())
    }
    fn print_set_fun_ptr(&mut self, dst: RegID, fid: FunID) -> io::Result<()> {
        self.print_assign(dst)?;
        let name = &self.module[fid].name;
        write!(self.out, "fun_ptr {name}")?;
        Ok(())
    }
    fn print_set_struct(&mut self, dst: RegID, values: &Values) -> io::Result<()> {
        self.print_assign(dst)?;
        self.print_values("{{ ", values, " }}")?;
        Ok(())
    }
    fn print_set_array(&mut self, dst: RegID, values: &Values) -> io::Result<()> {
        self.print_assign(dst)?;
        self.print_values("[ ", values, " ]")?;
        Ok(())
    }
    fn print_set_array_splat(&mut self, dst: RegID, value: Value) -> io::Result<()> {
        let Ty::Array(arr_ty) = self.module[dst].ty else {
            unreachable!()
        };
        let size = self.module[arr_ty].size;
        self.print_assign(dst)?;
        write!(self.out, "[ ")?;
        self.print_value(value)?;
        write!(self.out, " * {size}]")?;
        Ok(())
    }
    fn print_binary(&mut self, op: BinOp, dst: RegID, a: Value, b: Value) -> io::Result<()> {
        let name = match op {
            BinOp::Add => "add",
            BinOp::Sub => "sub",
            BinOp::Mul => "mul",
            BinOp::IDiv => "idiv",
            BinOp::UDiv => "udiv",
            BinOp::IMod => "imod",
            BinOp::UMod => "umod",
            BinOp::And => "and",
            BinOp::Or => "or",
            BinOp::Xor => "xor",
            BinOp::Shl => "shl",
            BinOp::Shr => "shr",
            BinOp::Sar => "sar",
            BinOp::Equal => "equal",
            BinOp::NotEqual => "not_equal",
            BinOp::Greater => "greater",
            BinOp::GreaterEqual => "greater_equal",
            BinOp::Less => "less",
            BinOp::LessEqual => "less_equal",
            BinOp::Above => "above",
            BinOp::AboveEqual => "above_equal",
            BinOp::Below => "below",
            BinOp::BelowEqual => "below_equal",
        };

        self.print_assign(dst)?;
        write!(self.out, "{name} ")?;
        self.print_value(a)?;
        write!(self.out, ", ")?;
        self.print_value(b)?;

        Ok(())
    }
    fn print_unary(&mut self, op: UnOp, dst: RegID, a: Value) -> io::Result<()> {
        let name = match op {
            UnOp::Neg => "neg",
            UnOp::Not => "not",
            UnOp::IntToPtr => "int_to_ptr",
            UnOp::PtrToInt => "ptr_to_int",
            UnOp::Sext => "sext",
            UnOp::Zext => "zext",
            UnOp::Trunc => "trunc",
        };

        self.print_assign(dst)?;
        write!(self.out, "{name} ")?;
        self.print_value(a)?;

        Ok(())
    }
    fn print_ret(&mut self, value: Value) -> io::Result<()> {
        write!(self.out, "ret ")?;
        self.print_value(value)?;
        Ok(())
    }
    fn print_index_array(
        &mut self,
        dst: RegID,
        ptr: RegID,
        element_ty: Ty,
        index: Value,
    ) -> io::Result<()> {
        self.print_assign(dst)?;
        write!(self.out, "index_array ")?;
        self.print_ty(element_ty)?;
        write!(self.out, " ")?;
        self.print_reg(ptr)?;
        write!(self.out, ", ")?;
        self.print_value(index)?;

        Ok(())
    }
    fn print_syscall_x86_64(
        &mut self,
        dst: RegID,
        call_num: Value,
        args: &Values,
    ) -> io::Result<()> {
        self.print_assign(dst)?;

        write!(self.out, "syscall ")?;
        self.print_value(call_num)?;
        self.print_values("( ", args, " )")?;

        Ok(())
    }

    fn print_assign(&mut self, dst: RegID) -> io::Result<()> {
        self.print_reg(dst)?;
        write!(self.out, " = ")?;
        Ok(())
    }

    fn print_values(&mut self, prefix: &str, values: &Values, suffix: &str) -> io::Result<()> {
        write!(self.out, "{prefix}")?;
        for (i, &value) in values.0.iter().enumerate() {
            self.print_value(value)?;
            if i != values.0.len() - 1 {
                write!(self.out, ", ")?;
            }
        }
        write!(self.out, "{suffix}")?;

        Ok(())
    }
    fn print_value(&mut self, value: Value) -> io::Result<()> {
        match value {
            Value::Reg(r) => self.print_reg(r)?,
            Value::Int(ty, val) => {
                write!(self.out, "{val} ")?;
                self.print_ty(ty)?;
            }
            Value::Void => write!(self.out, "void")?,
            Value::Bool(val) => write!(self.out, "{val}")?,
        }

        Ok(())
    }
    fn print_jump_tgt(&mut self, tgt: &JumpTarget) -> io::Result<()> {
        self.print_block_id(tgt.block)?;

        if !tgt.args.0.is_empty() {
            self.print_values("( ", &tgt.args, " )")?;
        }

        Ok(())
    }
    fn print_regs(&mut self, prefix: &str, regs: &[RegID], suffix: &str) -> io::Result<()> {
        write!(self.out, "{prefix}")?;
        for (i, &reg) in regs.iter().enumerate() {
            self.print_reg(reg)?;
            if i != regs.len() - 1 {
                write!(self.out, ", ")?;
            }
        }
        write!(self.out, "{suffix}")?;

        Ok(())
    }
    fn print_reg(&mut self, reg: RegID) -> io::Result<()> {
        let name = if let Some(&name) = self.reg_names.get(&reg) {
            name
        } else {
            let name = self.reg_names.len();
            self.reg_names.insert(reg, name);
            name
        };

        let ty = self.module[reg].ty;
        write!(self.out, "%{name} ")?;
        self.print_ty(ty)?;

        Ok(())
    }
    fn print_var(&mut self, var: VarID) -> io::Result<()> {
        let name = if let Some(&name) = self.var_names.get(&var) {
            name
        } else {
            let name = self.var_names.len();
            self.var_names.insert(var, name);
            name
        };
        write!(self.out, "&{name}")?;

        Ok(())
    }
    fn print_block_id(&mut self, block: BlockID) -> io::Result<()> {
        let name = if let Some(&name) = self.block_names.get(&block) {
            name
        } else {
            let name = self.block_names.len();
            self.block_names.insert(block, name);
            name
        };
        write!(self.out, "@{name}")?;

        Ok(())
    }

    fn print_tys(&mut self, prefix: &str, tys: &[Ty], suffix: &str) -> io::Result<()> {
        write!(self.out, "{prefix}")?;
        for (i, &ty) in tys.iter().enumerate() {
            self.print_ty(ty)?;
            if i != tys.len() - 1 {
                write!(self.out, ", ")?;
            }
        }
        write!(self.out, "{suffix}")?;

        Ok(())
    }
    fn print_ty(&mut self, ty: impl Into<Ty>) -> io::Result<()> {
        let ty: Ty = ty.into();
        match ty {
            Ty::Void => write!(self.out, "void")?,
            Ty::Bool => write!(self.out, "bool")?,
            Ty::Ptr => write!(self.out, "ptr")?,
            Ty::Int(int_ty) => self.print_int_ty(int_ty)?,
            Ty::Array(array_ty_id) => self.print_array_ty(array_ty_id)?,
            Ty::Struct(struct_ty_id) => self.print_struct_ty(struct_ty_id)?,
        }

        Ok(())
    }
    fn print_int_ty(&mut self, ty: IntTy) -> io::Result<()> {
        match ty {
            IntTy::I8 => write!(self.out, "i8")?,
            IntTy::I16 => write!(self.out, "i16")?,
            IntTy::I32 => write!(self.out, "i32")?,
            IntTy::I64 => write!(self.out, "i64")?,
        }

        Ok(())
    }
    fn print_array_ty(&mut self, ty: ArrayTyID) -> io::Result<()> {
        write!(self.out, "[ ")?;
        self.print_ty(self.module[ty].element)?;
        write!(self.out, " * {} ]", self.module[ty].size)?;

        Ok(())
    }
    fn print_struct_ty(&mut self, ty: StructTyID) -> io::Result<()> {
        let members = &self.module[ty].members;
        self.print_tys("{{ ", members, " }}")?;

        Ok(())
    }
}
