use std::{collections::{HashMap, HashSet, VecDeque}, io::{self, Write}};

use crate::*;

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

        while let Some(block) = blocks.pop_front() {
            done.insert(block);
            write!(self.out, "  ")?;
            self.print_block_id(block)?;
            let params = &self.module[block].parameters;
            if !params.is_empty() {
                write!(self.out, "( ")?;
                for (i, &param) in params.iter().enumerate() {
                    self.print_ty(self.module[param].ty)?;
                    write!(self.out, " ")?;
                    self.print_reg(param)?;
                    if i != params.len() - 1 {
                        write!(self.out, ", ")?;
                    }
                }

                write!(self.out, " )")?;
            }
            writeln!(self.out, ":")?;

            let instructions = &self.module[block].instructions;
            for instruction in instructions {
                if let Some(next) = self.print_instr(instruction)? {
                    if !done.contains(&next.0) {
                        blocks.push_back(next.0);
                    }
                    if let Some(next) = next.1 && !done.contains(&next) {
                        blocks.push_back(next);
                    }
                    break;
                }
            }

            writeln!(self.out)?;
        }

        Ok(())
    }
    fn clear_names(&mut self) {
        self.reg_names.clear();
        self.var_names.clear();
        self.block_names.clear();
    }



    fn print_instr(&mut self, instr: &Instruction) -> io::Result<Option<(BlockID, Option<BlockID>)>> {
        use Instruction::*;
        let mut ret = None;

        write!(self.out, "    ")?;
        match *instr {
            SetInt(dst, value) => {
                self.print_reg(dst)?;
                write!(self.out, " = ")?;
                self.print_ty(self.module[dst].ty)?;
                write!(self.out, " {value}")?;
            }
            SetBool(dst, value) => {
                self.print_reg(dst)?;
                write!(self.out, " = {value}")?;
            }
            SetStruct(dst, ref members) => {
                self.print_reg(dst)?;
                write!(self.out, " = {{ ")?;
                for (i, &member) in members.iter().enumerate() {
                    self.print_ty(self.module[member].ty)?;
                    write!(self.out, " ")?;
                    self.print_reg(member)?;
                    let last = i == members.len() - 1;
                    if !last {
                        write!(self.out, ", ")?;
                    }
                }
                write!(self.out, " }}")?;
            }
            SetArray(dst, ref elements) => {
                self.print_reg(dst)?;
                write!(self.out, " = ")?;
                self.print_ty(self.module[dst].ty)?;
                write!(self.out, " [ ")?;
                for (i, &element) in elements.iter().enumerate() {
                    self.print_reg(element)?;
                    let last = i == elements.len() - 1;
                    if !last {
                        write!(self.out, ", ")?;
                    }
                }
                write!(self.out, " ]")?;
            }
            Add(dst, a, b) => self.print_binop("add", dst, a, b)?,
            Sub(dst, a, b) => self.print_binop("sub", dst, a, b)?,
            Mul(dst, a, b) => self.print_binop("mul", dst, a, b)?,
            IDiv(dst, a, b) => self.print_binop("idiv", dst, a, b)?,
            UDiv(dst, a, b) => self.print_binop("udiv", dst, a, b)?,
            IMod(dst, a, b) => self.print_binop("imod", dst, a, b)?,
            UMod(dst, a, b) => self.print_binop("umod", dst, a, b)?,
            And(dst, a, b) => self.print_binop("and", dst, a, b)?,
            Or(dst, a, b) => self.print_binop("or", dst, a, b)?,
            Xor(dst, a, b) => self.print_binop("xor", dst, a, b)?,
            ShiftLeft(dst, a, b) => self.print_binop("shl", dst, a, b)?,
            ShiftRightLogical(dst, a, b) => self.print_binop("shr", dst, a, b)?,
            ShiftRightArithmetic(dst, a, b) => self.print_binop("sar", dst, a, b)?,
            IntToPtr(dst, a) => {
                self.print_reg(dst)?;
                write!(self.out, " = itp ")?;
                self.print_ty(self.module[a].ty)?;
                self.print_reg(a)?;
            }
            PtrToInt(dst, ptr) => self.print_unop("pti", dst, ptr)?,
            Neg(dst, a) => self.print_unop("neg", dst, a)?,
            Not(dst, a) => self.print_unop("not", dst, a)?,
            Sext(dst, a) => self.print_unop("sext", dst, a)?,
            Zext(dst, a) => self.print_unop("zext", dst, a)?,
            Truncate(dst, a) => self.print_unop("trunc", dst, a)?,
            
            Poison(dst) => {
                self.print_reg(dst)?;
                write!(self.out, " = ")?;
                self.print_ty(self.module[dst].ty)?;
                write!(self.out, " poison")?;
            }
            Select(dst, c, a, b) => {
                self.print_reg(dst)?;
                write!(self.out, " = ")?;
                self.print_ty(self.module[dst].ty)?;
                write!(self.out, " select ")?;
                self.print_reg(c)?;
                write!(self.out, ", ")?;
                self.print_reg(a)?;
                write!(self.out, ", ")?;
                self.print_reg(b)?;
            }
            Freeze(dst, a) => {
                self.print_reg(dst)?;
                write!(self.out, " = ")?;
                self.print_ty(self.module[dst].ty)?;
                write!(self.out, " freeze ")?;
                self.print_reg(a)?;
            }
            GetVarAddr(dst, var) => {
                self.print_reg(dst)?;
                write!(self.out, " = get_var_addr ")?;
                self.print_var(var)?;
            }
            Store { ptr, value } => {
                write!(self.out, "store ")?;
                self.print_ty(self.module[value].ty)?;
                write!(self.out, " ")?;
                self.print_reg(value)?;
                write!(self.out, " into ")?;
                self.print_reg(ptr)?;
            }
            Load { ptr, dst } => {
                self.print_reg(dst)?;
                write!(self.out, " = ")?;
                self.print_ty(self.module[dst].ty)?;
                write!(self.out, " load ")?;
                self.print_reg(ptr)?;
            }
        
            Jump(ref tgt) => {
                write!(self.out, "jump ")?;
                self.print_jump_tgt(tgt)?;
                ret = Some((tgt.block, None));
            }
            Branch(c, ref t, ref f) => {
                write!(self.out, "branch ")?;
                self.print_reg(c)?;
                write!(self.out, ", ")?;
                self.print_jump_tgt(t)?;
                write!(self.out, ", ")?;
                self.print_jump_tgt(f)?;
                ret = Some((t.block, Some(f.block)));
            }
            Ret(val) => {
                write!(self.out, "ret")?;
                if let Some(val) = val {
                    write!(self.out, " ")?;
                    self.print_ty(self.module[val].ty)?;
                    write!(self.out, " ")?;
                    self.print_reg(val)?;
                }
            }
            Call(dst, fun, ref args) => {
                if let Some(dst) = dst {
                    self.print_reg(dst)?;
                    write!(self.out, " = ")?;
                }
                else {
                    write!(self.out, "void = ")?;
                }

                let ret_ty = self.module[fun].ret_ty;
                self.print_ty(ret_ty)?;
                let name = &self.module[fun].name;
                write!(self.out, " call {name}( ")?;
                for (i, &arg) in args.iter().enumerate() {
                    self.print_ty(self.module[arg].ty)?;
                    self.print_reg(arg)?;
                    if i != args.len() - 1 {
                        write!(self.out, ", ")?;
                    }
                }
                write!(self.out, " )")?;
            }
            CallPtr(dst, ptr, fun_ty, ref args) => {
                if let Some(dst) = dst {
                    self.print_reg(dst)?;
                    write!(self.out, " = ")?;
                }
                else {
                    write!(self.out, "void = ")?;
                }

                let ret_ty = self.module[fun_ty].ret;
                self.print_ty(ret_ty)?;
                write!(self.out, " call ptr ")?;
                self.print_reg(ptr)?;
                write!(self.out, "( ")?;
                for (i, &arg) in args.iter().enumerate() {
                    self.print_ty(self.module[arg].ty)?;
                    self.print_reg(arg)?;
                    if i != args.len() - 1 {
                        write!(self.out, ", ")?;
                    }
                }
                write!(self.out, " )")?;
            }
            
            GetStructMember { dst, strct, index } => {
                self.print_reg(dst)?;
                write!(self.out, " = ")?;
                self.print_ty(self.module[dst].ty)?;
                write!(self.out, " get_struct_member ")?;
                self.print_reg(strct)?;
                write!(self.out, ".{index}")?;
            }
            SetStructMember { dst, strct, value, index } => {
                self.print_reg(dst)?;
                write!(self.out, " = ")?;
                self.print_ty(self.module[dst].ty)?;
                write!(self.out, " set_struct_member ")?;
                self.print_reg(strct)?;
                write!(self.out, ".{index} = ")?;
                self.print_reg(value)?;
            }
            GetArrayElement { dst, array, index } => {
                self.print_reg(dst)?;
                write!(self.out, " = ")?;
                self.print_ty(self.module[dst].ty)?;
                write!(self.out, " get_array_member ")?;
                self.print_reg(array)?;
                write!(self.out, "[ ")?;
                self.print_reg(index)?;
                write!(self.out, " ]")?;
            }
            SetArrayElement { dst, array, value, index } => {
                self.print_reg(dst)?;
                write!(self.out, " = ")?;
                self.print_ty(self.module[dst].ty)?;
                write!(self.out, " set_array_member ")?;
                self.print_reg(array)?;
                write!(self.out, "[ ")?;
                self.print_reg(index)?;
                write!(self.out, " ] = ")?;
                self.print_reg(value)?;
            }
            IndexStruct { dst, ptr, index } => {
                self.print_reg(dst)?;
                write!(self.out, " = ptr index_struct ")?;
                self.print_reg(ptr)?;
                write!(self.out, ".{index}")?;
            }
            IndexArray { dst, ptr, index } => {
                self.print_reg(dst)?;
                write!(self.out, " = ptr index_array ")?;
                self.print_reg(ptr)?;
                write!(self.out, "[ ")?;
                self.print_reg(index)?;
                write!(self.out, " ]")?;
            }
        }

        writeln!(self.out)?;

        Ok(ret)
    }
    fn print_binop(&mut self, name: &str, dst: RegID, a: RegID, b: RegID) -> io::Result<()> {
        self.print_reg(dst)?;
        write!(self.out, " = ")?;
        self.print_ty(self.module[dst].ty)?;
        write!(self.out, " {name} ")?;
        self.print_reg(a)?;
        write!(self.out, ", ")?;
        self.print_reg(b)?;

        Ok(())
    }
    fn print_unop(&mut self, name: &str, dst: RegID, a: RegID) -> io::Result<()> {
        self.print_reg(dst)?;
        write!(self.out, " = ")?;
        self.print_ty(self.module[dst].ty)?;
        write!(self.out, " {name} ")?;
        self.print_reg(a)?;

        Ok(())
    }
    fn print_jump_tgt(&mut self, tgt: &JumpTarget) -> io::Result<()> {
        self.print_block_id(tgt.block)?;

        if !tgt.args.is_empty() {
            write!(self.out, "( ")?;
            for (i, &arg) in tgt.args.iter().enumerate() {
                self.print_reg(arg)?;
                if i != tgt.args.len() - 1 {
                    write!(self.out, ", ")?;
                }
            }
            write!(self.out, " )")?;
        }

        Ok(())
    }
    fn print_reg(&mut self, reg: RegID) -> io::Result<()> {
        let name = if let Some(&name) = self.reg_names.get(&reg) {
            name
        }
        else {
            let name = self.reg_names.len();
            self.reg_names.insert(reg, name);
            name
        };
        write!(self.out, "%{name}")?;

        Ok(())
    }
    fn print_var(&mut self, var: VarID) -> io::Result<()> {
        let name = if let Some(&name) = self.var_names.get(&var) {
            name
        }
        else {
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
        }
        else {
            let name = self.block_names.len();
            self.block_names.insert(block, name);
            name
        };
        write!(self.out, "@{name}")?;

        Ok(())
    }
    
    fn print_ty(&mut self, ty: impl Into<Ty>) -> io::Result<()> {
        let ty: Ty = ty.into();
        match ty {
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
        write!(self.out, "; {} ]", self.module[ty].size)?;

        Ok(())
    }
    fn print_struct_ty(&mut self, ty: StructTyID) -> io::Result<()> {
        write!(self.out, "{{ ")?;
        let members = &self.module[ty].members;

        for (i, &member) in members.iter().enumerate() {
            self.print_ty(member)?;
            let last = i == members.len() - 1;
            if !last {
                write!(self.out, ", ")?;
            }
        }

        write!(self.out, " }}")?;

        Ok(())
    }
}
