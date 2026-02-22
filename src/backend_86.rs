use std::{collections::HashMap, io};

use gen86::writer::Condition;
use gen86::{gp_regs::*, mem::Mem, writer::X86Writer};
use gen86::nasm::NasmWriter;
use crate::frontend::{BinOp, IntTy, JumpTarget, Ty, UnOp, Value, Values};
use crate::{frontend::{BlockID, FunID, Function, Instruction, Module, RegID, VarID}, layout::TyLayout};

pub struct CodeGen<'a, O> {
    module: &'a Module,
    o: NasmWriter<O>,

    rsp: i64,

    known_ptrs: HashMap<RegID, Mem<'static>>,
    regs: HashMap<RegID, Mem<'static>>,
    vars: HashMap<VarID, Mem<'static>>,

    local_counter: usize,
    blocks: HashMap<BlockID, String>,    
}
impl<'a, O: io::Write> CodeGen<'a, O> {
    pub fn new(module: &'a Module, o: O) -> Self {
        Self {
            module,
            o: NasmWriter::new(o),

            rsp: 0,

            known_ptrs: HashMap::new(),
            regs: HashMap::new(),
            vars: HashMap::new(),

            local_counter: 0,
            blocks: HashMap::new(),
        }
    }

    pub fn gen_code(mut self) -> io::Result<()> {
        for function in self.module.functions() {
            self.gen_function(function)?;
            self.o.blank()?;
        }

        Ok(())
    }
    fn gen_function(&mut self, fun: &Function) -> io::Result<()> {
        if fun.entry_block.is_none() { return Ok(()) };
        
        self.o.global(&fun.name)?;
        self.o.label(&fun.name)?;
        self.o.push(RBP)?;
        self.o.mov(RBP, RSP)?;

        self.init_func();
        self.alloc_regs_vars(fun.id)?;
        self.collect_known_ptrs(fun.id);

        let entry = fun.entry_block.unwrap();
        let name = self.register_block(entry);
        self.o.jmp(&name)?;
        self.o.blank()?;

        let mut blocks: Vec<_> = fun.blocks.iter().copied().collect();
        blocks.sort_by_key(|b| b.0);

        for block in blocks {
            self.gen_block(block)?;
            self.o.blank()?;
        }

        Ok(())
    }
    fn init_func(&mut self) {
        self.rsp = 0;
        self.regs.clear();
        self.vars.clear();
        self.known_ptrs.clear();
    }
    fn alloc_regs_vars(&mut self, fid: FunID) -> io::Result<()> {
        let mut layout = TyLayout::new(0, 1);
        
        let mut reg_offsets = HashMap::new();
        for &reg in &self.module[fid].registers {
            let ty = self.module[reg].ty;
            let ty_layout = self.module.ty_layout(ty);
            let (next_layout, offset) = layout.extend(ty_layout);
            layout = next_layout;
            reg_offsets.insert(reg, offset);
        }

        let mut var_offsets = HashMap::new();
        for &var in &self.module[fid].variables {
            let ty = self.module[var].ty;
            let ty_layout = self.module.ty_layout(ty);
            let (next_layout, offset) = layout.extend(ty_layout);
            layout = next_layout;
            var_offsets.insert(var, offset);
        }


        layout = layout.align_to(16).pad_to_align();

        let (size, _align) = layout.bytes_signed();
        if size != 0 {
            self.o.add(RSP, -size)?;
        }
        self.rsp -= size;

        for (reg, offs) in reg_offsets {
            let mem = RBP.mem() + self.rsp + offs;
            self.regs.insert(reg, mem);
        }

        for (var, offs) in var_offsets {
            let mem = RBP.mem() + self.rsp + offs;
            self.vars.insert(var, mem);
        }


        Ok(())
    }
    fn collect_known_ptrs(&mut self, fid: FunID) {
        for &block in &self.module[fid].blocks {
            for instr in &self.module[block].instructions {
                if let Instruction::GetVarAddr(dst, var) = *instr {
                    let mem = self.vars[&var];
                    self.known_ptrs.insert(dst, mem);
                }
            }
        }
    }

    fn gen_block(&mut self, bid: BlockID) -> io::Result<()> {
        let name = self.register_block(bid);
        self.o.label(&name)?;

        for instr in &self.module[bid].instructions {
            self.gen_instr(instr)?;
        }

        Ok(())
    }
    fn make_local_label(&mut self) -> String {
        let id = self.local_counter;
        self.local_counter += 1;
        format!(".L{id}")
    }
    fn register_block(&mut self, bid: BlockID) -> String {
        if !self.blocks.contains_key(&bid) {
            let name = self.make_local_label();
            self.blocks.insert(bid, name);
        }

        self.blocks[&bid].clone()
    }

    fn gen_instr(&mut self, instr: &Instruction) -> io::Result<()> {
        match *instr {
            Instruction::Binary(BinOp::Add, dst, a, b) => self.gen_add(dst, a, b)?,
            Instruction::Binary(BinOp::Sub, dst, a, b) => self.gen_sub(dst, a, b)?,
            Instruction::Binary(BinOp::UDiv, dst, a, b) => self.gen_udiv(dst, a, b)?,
            Instruction::Binary(BinOp::UMod, dst, a, b) => self.gen_umod(dst, a, b)?,
            Instruction::Binary(BinOp::Equal, dst, a, b) => self.gen_test(Condition::E, dst, a, b)?,
            Instruction::Binary(BinOp::Less, dst, a, b) => self.gen_test(Condition::L, dst, a, b)?,
            Instruction::Unary(UnOp::Neg, dst, a) => self.gen_neg(dst, a)?,
            Instruction::Unary(UnOp::Trunc, dst, a) => self.gen_trunc(dst, a)?,
            Instruction::Select(dst, c, a, b) => self.gen_select(dst, c, a, b)?,
            Instruction::GetVarAddr(dst, var) => self.gen_get_var_addr(dst, var)?,
            Instruction::Load { dst, ptr } => self.gen_load(dst, ptr)?,
            Instruction::Store { ptr, value } => self.gen_store(ptr, value)?,
            Instruction::Jump(ref tgt) => self.gen_jump(tgt)?,
            Instruction::Branch(c, ref t, ref f) => self.gen_branch(c, t, f)?,
            Instruction::Ret(value) => self.gen_ret(value)?,
            Instruction::IndexArray { dst, ptr, element_ty, index } => self.gen_index_array(dst, ptr, element_ty, index)?,
            Instruction::SyscallLinux64 { dst, call_number, ref args } => self.gen_syscall_linux64(dst, call_number, args)?,
            ref or => todo!("Cannot compile {or:?}"),
        }

        Ok(())
    }
    fn gen_add(&mut self, dst: RegID, a: Value, b: Value) -> io::Result<()> {
        let Ty::Int(int_ty) = self.module[dst].ty else { unreachable!() };
        let size = int_rsize(int_ty);
        let rax = RAX + size;
        let rdx = RDX + size;
        self.place_value_in_register(rax, a)?;
        self.place_value_in_register(rdx, b)?;
        self.o.add(rax, rdx)?;
        self.place_register_in_reg(dst, rax)?;

        Ok(())
    }
    fn gen_sub(&mut self, dst: RegID, a: Value, b: Value) -> io::Result<()> {
        let Ty::Int(int_ty) = self.module[dst].ty else { unreachable!() };
        let size = int_rsize(int_ty);
        let rax = RAX + size;
        let rdx = RDX + size;
        self.place_value_in_register(rax, a)?;
        self.place_value_in_register(rdx, b)?;
        self.o.sub(rax, rdx)?;
        self.place_register_in_reg(dst, rax)?;

        Ok(())
    }
    fn gen_udiv(&mut self, dst: RegID, a: Value, b: Value) -> io::Result<()> {
        let Ty::Int(ty) = self.module[dst].ty else { unreachable!() };
        self.o.xor(EAX, EAX)?;
        self.o.xor(EDX, EDX)?;
        self.o.xor(ECX, ECX)?;

        let size = int_rsize(ty);
        let rax = RAX + size;
        let rcx = RCX + size;
        self.place_value_in_register(rax, a)?;
        self.place_value_in_register(rcx, b)?;
        self.o.div(rcx)?;
        self.place_register_in_reg(dst, rax)?;

        Ok(())
    }
    fn gen_umod(&mut self, dst: RegID, a: Value, b: Value) -> io::Result<()> {
        let Ty::Int(ty) = self.module[dst].ty else { unreachable!() };
        self.o.xor(EAX, EAX)?;
        self.o.xor(EDX, EDX)?;
        self.o.xor(ECX, ECX)?;

        let size = int_rsize(ty);
        let rax = RAX + size;
        let rcx = RCX + size;
        let rdx = RDX + size;
        self.place_value_in_register(rax, a)?;
        self.place_value_in_register(rcx, b)?;
        self.o.div(rcx)?;

        if ty == IntTy::I8 {
            self.o.shr(AX, 8)?;
            self.place_register_in_reg(dst, AL)?;
        }
        else {
            self.place_register_in_reg(dst, rdx)?;
        }


        Ok(())
    }
    fn gen_test(&mut self, cc: Condition, dst: RegID, a: Value, b: Value) -> io::Result<()> {
        let Ty::Int(ty) = a.ty(self.module) else { unreachable!() };
        let size = int_rsize(ty);
        let rax = RAX + size;
        let rdx = RDX + size;
        self.place_value_in_register(rax, a)?;
        self.place_value_in_register(rdx, b)?;
        self.o.xor(ECX, ECX)?;
        self.o.mov(EDI, 1)?;
        self.o.cmp(rax, rdx)?;
        self.o.cmov(cc, ECX, EDI)?;
        self.place_register_in_reg(dst, CL)?;

        Ok(())
    }
    fn gen_neg(&mut self, dst: RegID, a: Value) -> io::Result<()> {
        let Ty::Int(ty) = self.module[dst].ty else { unreachable!() };
        let size = int_rsize(ty);
        let rax = RAX + size;
        self.place_value_in_register(rax, a)?;
        self.o.neg(rax)?;
        self.place_register_in_reg(dst, rax)?;

        Ok(())
    }
    fn gen_trunc(&mut self, dst: RegID, a: Value) -> io::Result<()> {
        let Ty::Int(to_ty) = self.module[dst].ty else { unreachable!() };
        let Ty::Int(from_ty) = a.ty(self.module) else { unreachable!() };
        let to_size = int_rsize(to_ty);
        let from_size = int_rsize(from_ty);

        self.place_value_in_register(RAX + from_size, a)?;
        self.place_register_in_reg(dst, RAX + to_size)?;

        Ok(())
    }
    fn gen_select(&mut self, dst: RegID, c: Value, a: Value, b: Value) -> io::Result<()> {
        let take_b = self.make_local_label();
        let end = self.make_local_label();

        self.place_value_in_register(AL, c)?;
        self.o.cmp(AL, 0)?;
        self.o.jcc(Condition::E, &take_b)?;
        self.mov_value_to_reg(dst, a)?;
        self.o.jmp(&end)?;

        self.o.label(&take_b)?;
        self.mov_value_to_reg(dst, b)?;
        self.o.label(&end)?;

        Ok(())
    }
    fn gen_get_var_addr(&mut self, dst: RegID, var: VarID) -> io::Result<()> {
        let dst_slot = self.regs[&dst];
        let var_slot = self.vars[&var];

        self.o.lea(RBX, var_slot)?;
        self.o.mov(dst_slot, RBX)?;

        Ok(())
    }
    fn gen_load(&mut self, dst: RegID, ptr: RegID) -> io::Result<()> {
        let ty = self.module[dst].ty;
        let layout = self.module.ty_layout(ty);
        let ptr_slot = self.regs[&ptr];
        let dst_slot = self.regs[&dst];

        if let Some(&mem) = self.known_ptrs.get(&ptr) {
            self.memcpy(dst_slot, mem, layout)?;
        }
        else {   
            self.o.mov(RBX, ptr_slot)?;
            self.memcpy(dst_slot, RBX.mem(), layout)?;
        }

        Ok(())
    }
    fn gen_store(&mut self, ptr: RegID, value: Value) -> io::Result<()> {
        if let Some(&mem) = self.known_ptrs.get(&ptr) {
            self.mov_value_to_mem(mem, value)?;
        }
        else {   
            let ptr_slot = self.regs[&ptr];
            self.o.mov(RBX, ptr_slot)?;
            self.mov_value_to_mem(RBX.mem(), value)?;
        }

        Ok(())
    }
    fn gen_jump(&mut self, tgt: &JumpTarget) -> io::Result<()> {
        self.prepare_jump(tgt)?;
        let name = self.register_block(tgt.block);
        self.o.jmp(&name)?;

        Ok(())
    }
    fn gen_branch(&mut self, c: Value, t: &JumpTarget, f: &JumpTarget) -> io::Result<()> {
        let take_false = self.make_local_label();
        
        self.place_value_in_register(AL, c)?;
        self.o.cmp(AL, 0)?;
        self.o.jcc(Condition::E, &take_false)?;
        self.prepare_jump(t)?;
        let name = self.register_block(t.block);
        self.o.jmp(&name)?;

        self.o.label(&take_false)?;
        self.prepare_jump(f)?;
        let name = self.register_block(f.block);
        self.o.jmp(&name)?;

        Ok(())
    }
    fn gen_ret(&mut self, value: Value) -> io::Result<()> {
        let to = RBP.mem() + 16;
        self.mov_value_to_mem(to, value)?;
        self.o.mov(RSP, RBP)?;
        self.o.pop(RBP)?;
        self.o.ret()?;

        Ok(())
    }
    fn gen_index_array(&mut self, dst: RegID, ptr: RegID, elem_ty: Ty, index: Value) -> io::Result<()> {
        let ptr_slot = self.regs[&ptr];
        self.place_value_in_register(RDX, index)?;
        self.o.mov(RBX, ptr_slot)?;

        let layout = self.module.ty_layout(elem_ty).pad_to_align();
        let size = layout.size() as i64;
        if size != 1 {
            self.o.imul3(RDX, RDX, size)?;
        }
        self.o.add(RBX, RDX)?;

        let dst_slot = self.regs[&dst];
        self.o.mov(dst_slot, RBX)?;

        Ok(())
    }
    fn gen_syscall_linux64(&mut self, dst: RegID, call_number: Value, args: &Values) -> io::Result<()> {
        let call_num_ty = call_number.ty(self.module);
        let Ty::Int(call_num_ty) = call_num_ty else { unreachable!() };
        let call_num_size = int_rsize(call_num_ty);
        self.o.xor(EAX, EAX)?;
        self.place_value_in_register(RAX + call_num_size, call_number)?;

        let regs = [RDI, RSI, RDX, R10, R8, R9];
        for (reg, &arg) in regs.into_iter().zip(&args.0) {
            let arg_ty = arg.ty(self.module);
            if let Ty::Int(arg_ty) = arg_ty {
                let ereg = reg + RSize::DWord;
                self.o.xor(ereg, ereg)?;
                let size = int_rsize(arg_ty);
                let reg = reg + size;
                self.place_value_in_register(reg, arg)?;
            }
            else if let Ty::Ptr = arg_ty {
                self.place_value_in_register(reg, arg)?;
            }
            else {
                panic!();
            }
        }

        self.o.syscall()?;

        let ret_ty = self.module[dst].ty;
        let size = match ret_ty {
            Ty::Int(ty) => int_rsize(ty),
            Ty::Ptr => RSize::QWord,
            _ => panic!(),
        };

        self.place_register_in_reg(dst, RAX + size)?;

        Ok(())
    }


    fn place_value_in_register(&mut self, to: Reg, value: Value) -> io::Result<()> {
        match value {
            Value::Void => (),
            Value::Bool(value) => self.o.mov(to, value as i64)?,
            Value::Int(_int_ty, value) => self.o.mov(to, value)?,
            Value::Reg(reg_id) => {
                let slot = self.regs[&reg_id];
                self.o.mov(to, slot)?;
            }
        }

        Ok(())
    }
    fn mov_value_to_mem(&mut self, to: Mem, value: Value) -> io::Result<()> {
        match value {
            Value::Void => (),
            Value::Bool(value) => self.o.mov(to + RSize::Byte, value as u8)?,
            Value::Int(int_ty, value) => {
                let size = int_rsize(int_ty);
                if fits_32_bit(value) {
                    self.o.mov(to + size, value)?;
                }
                else {
                    let reg = RAX + size;
                    self.o.mov(reg, value)?;
                    self.o.mov(to, reg)?;
                }
            }
            Value::Reg(reg_id) => self.mov_reg_to_mem(to, reg_id)?,
        }

        Ok(())
    }
    fn mov_reg_to_mem(&mut self, to: Mem, reg: RegID) -> io::Result<()> {
        let layout = self.module.ty_layout(self.module[reg].ty);
        let slot = self.regs[&reg];
        self.memcpy(to, slot, layout)?;
        Ok(())
    }
    fn mov_reg_to_reg(&mut self, to: RegID, from: RegID) -> io::Result<()> {
        let ty = self.module[to].ty;
        let layout = self.module.ty_layout(ty);
        let to_slot = self.regs[&to];
        let from_slot = self.regs[&from];
        self.memcpy(to_slot, from_slot, layout)?;
        Ok(())
    }
    fn mov_value_to_reg(&mut self, to: RegID, value: Value) -> io::Result<()> {
        let slot = self.regs[&to];
        self.mov_value_to_mem(slot, value)?;
        Ok(())
    }
    fn memcpy(&mut self, to: Mem, from: Mem, layout: TyLayout) -> io::Result<()> {
        let layout = layout.pad_to_align();
        let (size, align) = layout.bytes_signed();
        let (rsize, stride) = match align {
            1 => (RSize::Byte, 1),
            2 => (RSize::Word, 2),
            4 => (RSize::DWord, 4),
            8 => (RSize::QWord, 8),
            _ => unreachable!(),
        };
        let reg = RAX + rsize;

        for offs in (0..size).step_by(stride) {
            self.o.mov(reg, from + offs)?;
            self.o.mov(to + offs, reg)?;
        }

        Ok(())
    }
    fn place_register_in_reg(&mut self, to: RegID, from: Reg) -> io::Result<()> {
        let size = self.module.ty_layout(self.module[to].ty).size();
        if size == 0 { return Ok(()) };
        let slot = self.regs[&to];
        self.o.mov(slot, from)?;
        Ok(())
    }

    fn prepare_jump(&mut self, tgt: &JumpTarget) -> io::Result<()> {
        let params = &self.module[tgt.block].parameters;
        for (&p, &a) in params.iter().zip(&tgt.args.0) {
            self.mov_value_to_reg(p, a)?;
        }

        Ok(())
    }
}

fn int_rsize(ty: IntTy) -> RSize {
    match ty {
        IntTy::I8 => RSize::Byte,
        IntTy::I16 => RSize::Word,
        IntTy::I32 => RSize::DWord,
        IntTy::I64 => RSize::QWord,
    }
}
fn fits_32_bit(value: i64) -> bool {
    let max = u32::MAX as i64;
    let min = i32::MIN as i64;
    min <= value && value <= max
}
