use super::{label::Label, register::Register};
use crate::size::Size;
use std::{fmt::Display, ops::AddAssign};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Arg<'a> {
    Register(Register),
    Label(Label<'a>),
    Int(ConstInt),
    Memory(Memory<'a>),
}
impl<'a> Arg<'a> {
    pub fn size(&self) -> Option<Size> {
        let size = match self {
            &Self::Register(r) => match r.1 {
                Size::Byte => Size::Byte,
                Size::Word => Size::Word,
                Size::Double => Size::Double,
                Size::Quad => Size::Quad,
            },
            &Self::Int(c) => match c {
                ConstInt::I8(_) | ConstInt::U8(_) => Size::Byte,
                ConstInt::I16(_) | ConstInt::U16(_) => Size::Word,
                ConstInt::I32(_) | ConstInt::U32(_) => Size::Double,
                ConstInt::I64(_) | ConstInt::U64(_) => Size::Quad,
            },
            &Self::Label(_) => return None,
            &Self::Memory(m) => return m.size,
        };
        Some(size)
    }
    pub fn is_register(&self) -> bool {
        match self {
            Self::Register(_) => true,
            _ => false,
        }
    }
    pub fn is_memory(&self) -> bool {
        match self {
            Self::Memory(_) => true,
            _ => false,
        }
    }
}
impl From<Register> for Arg<'_> {
    fn from(value: Register) -> Self {
        Self::Register(value)
    }
}
impl<'a> From<Label<'a>> for Arg<'a> {
    fn from(value: Label<'a>) -> Self {
        Self::Label(value)
    }
}
impl From<i8> for Arg<'_> {
    fn from(value: i8) -> Self {
        Self::Int(ConstInt::from(value))
    }
}
impl From<u8> for Arg<'_> {
    fn from(value: u8) -> Self {
        Self::Int(ConstInt::from(value))
    }
}
impl From<i16> for Arg<'_> {
    fn from(value: i16) -> Self {
        Self::Int(ConstInt::from(value))
    }
}
impl From<u16> for Arg<'_> {
    fn from(value: u16) -> Self {
        Self::Int(ConstInt::from(value))
    }
}
impl From<i32> for Arg<'_> {
    fn from(value: i32) -> Self {
        Self::Int(ConstInt::from(value))
    }
}
impl From<u32> for Arg<'_> {
    fn from(value: u32) -> Self {
        Self::Int(ConstInt::from(value))
    }
}
impl From<i64> for Arg<'_> {
    fn from(value: i64) -> Self {
        Self::Int(ConstInt::from(value))
    }
}
impl From<u64> for Arg<'_> {
    fn from(value: u64) -> Self {
        Self::Int(ConstInt::from(value))
    }
}
impl From<ConstInt> for Arg<'_> {
    fn from(value: ConstInt) -> Self {
        Self::Int(value)
    }
}
impl<'a> From<&'a str> for Arg<'a> {
    fn from(value: &'a str) -> Self {
        Self::Label(value.into())
    }
}
impl<'a> From<&'a String> for Arg<'a> {
    fn from(value: &'a String) -> Self {
        Self::Label(Label::from(value))
    }
}
impl<'a> From<Memory<'a>> for Arg<'a> {
    fn from(value: Memory<'a>) -> Self {
        Self::Memory(value)
    }
}
impl Display for Arg<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &Arg::Int(c) => write!(f, "${c}"),
            &Arg::Label(l) => write!(f, "{l}"),
            &Arg::Register(r) => write!(f, "{r}"),
            &Arg::Memory(mem) => write!(f, "{mem}"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ConstInt {
    I8(i8),
    U8(u8),
    I16(i16),
    U16(u16),
    I32(i32),
    U32(u32),
    I64(i64),
    U64(u64),
}
impl ConstInt {
    pub fn is_zero(self) -> bool {
        match self {
            Self::I8(v) => v == 0,
            Self::U8(v) => v == 0,
            Self::I16(v) => v == 0,
            Self::U16(v) => v == 0,
            Self::I32(v) => v == 0,
            Self::U32(v) => v == 0,
            Self::I64(v) => v == 0,
            Self::U64(v) => v == 0,
        }
    }
    pub fn is_negative(self) -> bool {
        match self {
            Self::I8(v) => v < 0,
            Self::U8(_) => false,
            Self::I16(v) => v < 0,
            Self::U16(_) => false,
            Self::I32(v) => v < 0,
            Self::U32(_) => false,
            Self::I64(v) => v < 0,
            Self::U64(_) => false,
        }
    }
}
impl Display for ConstInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::I8(v) => write!(f, "{v}"),
            Self::U8(v) => write!(f, "{v}"),
            Self::I16(v) => write!(f, "{v}"),
            Self::U16(v) => write!(f, "{v}"),
            Self::I32(v) => write!(f, "{v}"),
            Self::U32(v) => write!(f, "{v}"),
            Self::I64(v) => write!(f, "{v}"),
            Self::U64(v) => write!(f, "{v}"),
        }
    }
}
impl From<i8> for ConstInt {
    fn from(value: i8) -> Self {
        Self::I8(value)
    }
}
impl From<u8> for ConstInt {
    fn from(value: u8) -> Self {
        Self::U8(value)
    }
}
impl From<i16> for ConstInt {
    fn from(value: i16) -> Self {
        Self::I16(value)
    }
}
impl From<u16> for ConstInt {
    fn from(value: u16) -> Self {
        Self::U16(value)
    }
}
impl From<i32> for ConstInt {
    fn from(value: i32) -> Self {
        Self::I32(value)
    }
}
impl From<u32> for ConstInt {
    fn from(value: u32) -> Self {
        Self::U32(value)
    }
}
impl From<i64> for ConstInt {
    fn from(value: i64) -> Self {
        Self::I64(value)
    }
}
impl From<u64> for ConstInt {
    fn from(value: u64) -> Self {
        Self::U64(value)
    }
}
impl AddAssign for ConstInt {
    fn add_assign(&mut self, rhs: Self) {
        use ConstInt::*;
        match (self, rhs) {
            (I32(a), I32(b)) => *a += b,
            (U32(a), U32(b)) => *a += b,
            (I64(a), I64(b)) => *a += b,
            (U64(a), U64(b)) => *a += b,
            _ => panic!(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Memory<'a> {
    pub size: Option<Size>,
    pub displacement_label: Option<Label<'a>>,
    pub displacement_constant: Option<ConstInt>,
    pub kind: MemoryKind,
}
impl<'a> Memory<'a> {
    pub fn sib() -> Self {
        Self {
            size: None,
            displacement_label: None,
            displacement_constant: None,
            kind: MemoryKind::Sib(SibMemory {
                base: None,
                index: None,
            }),
        }
    }
    pub fn rip() -> Self {
        Self {
            size: None,
            displacement_label: None,
            displacement_constant: None,
            kind: MemoryKind::Rip,
        }
    }

    pub fn base(mut self, base: Register) -> Self {
        let MemoryKind::Sib(m) = &mut self.kind else { panic!() };
        m.base = Some(base);
        self
    }
    pub fn index(mut self, index: Register, scale: Size) -> Self {
        let MemoryKind::Sib(m) = &mut self.kind else { panic!() };
        m.index = Some((index, scale));
        self
    }
    pub fn offset(mut self, disp: impl Into<ConstInt>) -> Self {
        let disp: ConstInt = disp.into();
        if let Some(displacement) = &mut self.displacement_constant {
            *displacement += disp;
        } else {
            self.displacement_constant = Some(disp);
        }

        self
    }
    pub fn label(mut self, label: impl Into<Label<'a>>) -> Self {
        let label = label.into();
        if self.displacement_label.is_some() {
            panic!()
        }
        self.displacement_label = Some(label);
        self
    }
    pub fn size(mut self, size: Size) -> Self {
        self.size = Some(size);
        self
    }
}
impl Display for Memory<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(label) = self.displacement_label {
            write!(f, "{label}")?;
        }
        if let Some(constant) = self.displacement_constant {
            if !constant.is_zero() {
                if self.displacement_label.is_some() && !constant.is_negative() {
                    write!(f, "+")?;
                }

                write!(f, "{constant}")?;
            }
        }

        write!(f, "{}", self.kind)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum MemoryKind {
    Rip,
    Sib(SibMemory),
}
impl Display for MemoryKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Rip => write!(f, "(%rip)"),
            Self::Sib(mem) => write!(f, "{mem}"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SibMemory {
    base: Option<Register>,
    index: Option<(Register, Size)>,
}
impl Display for SibMemory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let any = self.base.is_some() || self.index.is_some();
        if any {
            write!(f, "(")?;
        }

        if let Some(base) = self.base {
            write!(f, "{base}")?;
        }

        if let Some((index, scale)) = self.index {
            write!(f, ", {index}")?;
            if scale != Size::Byte {
                write!(f, ", {}", scale.in_bytes())?;
            }
        }

        if any {
            write!(f, ")")?;
        }

        Ok(())
    }
}
