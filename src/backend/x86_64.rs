use std::{io::{Write, self}, collections::{HashMap, HashSet}, ops::{Add, Sub, Rem, SubAssign, AddAssign}, num::NonZeroU32};
use x64_writer::{writer::AsmWriter, register::{rbp, rsp, rax, RegisterName, Register, RegisterSize}, args::Memory};
use crate::{Module, Types, target::System, function::{FuncID, calling_convention::CallingConvention}, variable::VarID, Type, struct_type::StructTypeID, register::RegID, block::BlockID, instruction::{Instruction, Expr, ConstValue}};

mod systemv_parameters;

pub struct Backend<'a, O> {
    system: System,
    writer: AsmWriter<O>,
    module: &'a Module,
    types: &'a mut Types,

    ctx: FuncCtx,
    struct_layouts: HashMap<StructTypeID, StructLayout>,
}
impl<'a, O: Write> Backend<'a, O> {
    pub fn new(system: System, out: O, module: &'a Module, types: &'a mut Types) -> Self {
        Self {
            system,
            writer: AsmWriter::new(out),
            module,
            types,

            ctx: FuncCtx::new(FuncID(0xdeadbeefdeadbeef)),
            struct_layouts: HashMap::new(),
        }
    }


    pub fn gen_code(mut self) -> io::Result<()> {
        self.writer.begin_text()?;
        for func in &self.module.functions {
            self.gen_function(func.id())?;
        }

        Ok(())
    }

    fn gen_function(&mut self, fid: FuncID) -> io::Result<()> {
        if self.module[fid].definition().is_none() {
            return Ok(());
        }

        let name = self.module[fid].name();
        self.writer.declare_global(name)?;
        self.writer.emit_label(name)?;
        self.gen_function_prologue(fid)?;
        self.gen_function_body(fid)?;

        Ok(())
    }
    fn gen_function_prologue(&mut self, fid: FuncID) -> io::Result<()> {
        let convention = self.module[fid].calling_convention();

        use System::*;
        use CallingConvention::*;
        match (self.system, convention) {
            (_, SystemV) => self.gen_systemv_prologue(fid),
            (Linux | BareMetal, Default) => self.gen_systemv_prologue(fid),
            (Linux | BareMetal, C) => self.gen_systemv_prologue(fid),
        }
    }
    fn gen_systemv_prologue(&mut self, fid: FuncID) -> io::Result<()> {
        self.writer.build_push(rbp())?;
        self.writer.build_mov(rbp(), rsp())?;

        let definition = self.module[fid].definition().unwrap();
        for &var in definition.variables() {
            let var_t = self.module[var].var_type();
            let offset = self.delayed_alloc_type(var_t);
            self.ctx.variables.insert(var, offset);
        }

        for &reg in self.module[fid].registers() {
            let reg_t = self.module[reg].reg_type();
            let offset = self.delayed_alloc_type(reg_t);
            self.ctx.registers.insert(reg, offset);
        }
        self.realize_alloc()?;
        self.ctx.start_temps();

        self.init_systemv_arguments(fid)
    }
    fn init_systemv_arguments(&mut self, _fid: FuncID) -> io::Result<()> {
        todo!()
    }    
    fn gen_function_epilogue(&mut self, fid: FuncID) -> io::Result<()> {
        let convention = self.module[fid].calling_convention();

        use System::*;
        use CallingConvention::*;
        match (self.system, convention) {
            (_, SystemV) => self.gen_systemv_epilogue(fid),
            (Linux | BareMetal, Default) => self.gen_systemv_epilogue(fid),
            (Linux | BareMetal, C) => self.gen_systemv_epilogue(fid),
        }
    }
    fn gen_systemv_epilogue(&mut self, _fid: FuncID) -> io::Result<()> {
        self.writer.build_mov(rsp(), rbp())?;
        self.writer.build_pop(rbp())?;
        self.writer.build_ret()?;
        Ok(())
    }

    fn gen_function_body(&mut self, fid: FuncID) -> io::Result<()> {
        let definition = self.module[fid].definition().unwrap();
        
        let mut todo = vec!(definition.entry());
        let mut done = HashSet::new();

        while let Some(block) = todo.pop() {
            if done.contains(&block) {
                continue;
            }

            let do_next = self.gen_block(block)?;
            todo.extend(do_next);
            done.insert(block);
        }

        Ok(())
    }
    fn gen_block(&mut self, bid: BlockID) -> io::Result<Vec<BlockID>> {
        let number = bid.0;
        self.writer.emit_label(&format!(".LBB{number}"))?;
        for instr in self.module[bid].body() {
            match instr {
                Instruction::Nop => (),
                _ => todo!(),
            }
            self.clear_temps()?;
        }

        Ok(vec!())
    }

    fn clear_temps(&mut self) -> io::Result<()> {
        let movement = self.ctx.temp_start - self.ctx.rsp_offset;
        let movement = movement.0;
        self.writer.build_add(rsp(), movement)?;
        self.ctx.rsp_offset += movement;

        Ok(())
    }
    fn eval_expr(&mut self, target: Register, e: &Expr) -> io::Result<EvaledExpr> {
        todo!()
    }
    fn move_bytes<'b>(&mut self, from: impl Into<Memory<'b>>, to: impl Into<Memory<'b>>, size: u32) -> io::Result<()> {
        let from: Memory = from.into();
        let to: Memory = to.into();
        for i in 0..size {
            let src = from.offset(i);
            let dst = to.offset(i);
            self.writer.build_mov(rax(), src)?;
            self.writer.build_mov(dst, rax())?;
        }

        Ok(())
    }

    fn alloc_type(&mut self, t: Type) -> io::Result<BaseOffset> {
        let offset = self.delayed_alloc_type(t);
        self.realize_alloc()?;
        Ok(offset)
    }
    fn delayed_alloc_type(&mut self, t: Type) -> BaseOffset {
        let (size, align) = self.layout(t);
        self.delayed_alloc(size, align)
    }
    fn delayed_alloc(&mut self, size: u32, alignment: NonZeroU32) -> BaseOffset {
        let offset = self.ctx.rsp_offset + self.ctx.unrealized_movement;
        let mut offset = offset.0;
        let start_offset = offset;
        let alignment: u32 = alignment.into();
        let alignment = alignment as i32;
        let size = size as i32;

        offset -= size;
        while offset % alignment != 0 {
            offset -= 1;
        }

        let change = offset - start_offset;
        self.ctx.unrealized_movement += change;

        BaseOffset(offset)
    }
    fn realize_alloc(&mut self) -> io::Result<()> {
        let change = self.ctx.unrealized_movement;
        self.ctx.unrealized_movement = 0;
        self.writer.build_add(rsp(), change)?;
        Ok(())
    }

    fn layout(&mut self, t: Type) -> (u32, NonZeroU32) {
        use Type::*;
        match t {
            Unit => (0, 1.try_into().unwrap()),
            Bool => (1, 1.try_into().unwrap()),
            Integer(size) => {
                let bytes = size.bytes() as u32;
                (bytes, bytes.try_into().unwrap())
            }
            Pointer => (8, 8.try_into().unwrap()),
            Struct(s) => {
                let layout = self.struct_layout(s);
                (layout.size, layout.alignment)
            }
            Array(id) => {
                let element = self.types[id].member();
                let length = self.types[id].length() as u32;
                let (el_size, align) = self.layout(element);

                (el_size * length, align)
            }
        }
    }
    fn struct_layout(&mut self, s: StructTypeID) -> &StructLayout {
        if !self.struct_layouts.contains_key(&s) {
            let members = self.types[s].members().to_vec();
            let mut size = 0;
            let mut align = NonZeroU32::new(1).unwrap();
            let mut field_offsets = Vec::with_capacity(members.len());

            for member in members {
                let (mem_size, mem_align) = self.layout(member);
                align = align.max(mem_align);
                let mem_align: u32 = mem_align.into();
                while size % mem_align != 0 {
                    size += 1;
                }
                field_offsets.push(size);
                size += mem_size;
            }
            while size % align != 0 {
                size += 1;
            }
        }

        
        &self.struct_layouts[&s]
    }
}


enum EvaledExpr {
    Register,
    Temporary(BaseOffset),
}



struct FuncCtx {
    id: FuncID,
    rsp_offset: BaseOffset,
    unrealized_movement: i32,
    temp_start: BaseOffset,
    variables: HashMap<VarID, BaseOffset>,
    registers: HashMap<RegID, BaseOffset>,
}
impl FuncCtx {
    fn new(id: FuncID) -> Self {
        Self {
            id,
            unrealized_movement: 0,
            rsp_offset: 0.into(),
            temp_start: 0.into(),
            variables: HashMap::new(),
            registers: HashMap::new(),
        }
    }

    fn start_temps(&mut self) {
        self.temp_start = self.rsp_offset;
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct BaseOffset(i32);
impl Add for BaseOffset {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        BaseOffset(self.0 + rhs.0)
    }
}
impl Add<i32> for BaseOffset {
    type Output = Self;
    fn add(self, rhs: i32) -> Self::Output {
        Self(self.0 + rhs)
    }
}
impl AddAssign<i32> for BaseOffset {
    fn add_assign(&mut self, rhs: i32) {
        self.0 += rhs;
    }
}
impl Sub for BaseOffset {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}
impl SubAssign for BaseOffset {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}
impl Sub<i32> for BaseOffset {
    type Output = Self;
    fn sub(self, rhs: i32) -> Self::Output {
        Self(self.0 - rhs)
    }
}
impl SubAssign<i32> for BaseOffset {
    fn sub_assign(&mut self, rhs: i32) {
        self.0 -= rhs;
    }
}
impl Rem<i32> for BaseOffset {
    type Output = i32;
    fn rem(self, rhs: i32) -> Self::Output {
        self.0 % rhs
    }
}
impl From<i32> for BaseOffset {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
impl From<u16> for BaseOffset {
    fn from(value: u16) -> Self {
        Self(value as i32)
    }
}
impl From<i16> for BaseOffset {
    fn from(value: i16) -> Self {
        Self(value as i32)
    }
}
impl From<u8> for BaseOffset {
    fn from(value: u8) -> Self {
        Self(value as i32)
    }
}
impl From<i8> for BaseOffset {
    fn from(value: i8) -> Self {
        Self(value as i32)
    }
}



struct StructLayout {
    size: u32,
    alignment: NonZeroU32,
    field_offsets: Vec<u32>,
}
