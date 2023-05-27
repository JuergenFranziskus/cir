use super::args::Memory;
use crate::{args::ConstInt, size::Size};
use std::fmt::Display;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Register(pub RegisterName, pub Size);
impl Register {
    pub fn memory(self) -> Memory<'static> {
        Memory::sib().base(self)
    }
    pub fn offset(self, offset: impl Into<ConstInt>) -> Memory<'static> {
        self.memory().offset(offset)
    }
    pub fn index(self, index: Register, scale: Size) -> Memory<'static> {
        self.memory().index(index, scale)
    }

    pub fn with_size(self, size: Size) -> Self {
        Self(self.0, size)
    }
    pub fn as_byte(self) -> Self {
        self.with_size(Size::Byte)
    }
    pub fn as_word(self) -> Self {
        self.with_size(Size::Word)
    }
    pub fn as_double(self) -> Self {
        self.with_size(Size::Double)
    }
    pub fn as_quad(self) -> Self {
        self.with_size(Size::Quad)
    }
}
impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let get_affixes = if self.0.is_sandwich() {
            Size::sandwich_affixes
        } else if self.0.is_pointer() {
            Size::pointer_affixes
        } else if self.0.is_numbered() {
            Size::numbered_affixes
        } else {
            unreachable!()
        };

        let (prefix, suffix) = get_affixes(self.1);
        let name = self.0.name();
        write!(f, "%{prefix}{name}{suffix}")
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum RegisterName {
    A,
    B,
    C,
    D,
    SI,
    DI,
    SP,
    BP,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
}
impl RegisterName {
    fn name(&self) -> &'static str {
        use RegisterName::*;
        match self {
            A => "a",
            B => "b",
            C => "c",
            D => "d",
            SI => "si",
            DI => "di",
            SP => "sp",
            BP => "bp",
            R8 => "r8",
            R9 => "r9",
            R10 => "r10",
            R11 => "r11",
            R12 => "r12",
            R13 => "r13",
            R14 => "r14",
            R15 => "r15",
        }
    }
    fn is_sandwich(self) -> bool {
        use RegisterName::*;
        match self {
            A | B | C | D => true,
            _ => false,
        }
    }
    fn is_pointer(self) -> bool {
        use RegisterName::*;
        match self {
            SI | DI | SP | BP => true,
            _ => false,
        }
    }
    fn is_numbered(self) -> bool {
        use RegisterName::*;
        match self {
            R8 | R9 | R10 | R11 | R12 | R13 | R14 | R15 => true,
            _ => false,
        }
    }

    pub fn with_size(self, size: Size) -> Register {
        Register(self, size)
    }
    pub fn byte(self) -> Register {
        Register(self, Size::Byte)
    }
    pub fn word(self) -> Register {
        Register(self, Size::Word)
    }
    pub fn double(self) -> Register {
        Register(self, Size::Double)
    }
    pub fn quad(self) -> Register {
        Register(self, Size::Quad)
    }
}

pub fn a_name() -> RegisterName {
    RegisterName::A
}
pub fn b_name() -> RegisterName {
    RegisterName::B
}
pub fn c_name() -> RegisterName {
    RegisterName::C
}
pub fn d_name() -> RegisterName {
    RegisterName::D
}
pub fn si_name() -> RegisterName {
    RegisterName::SI
}
pub fn di_name() -> RegisterName {
    RegisterName::DI
}
pub fn sp_name() -> RegisterName {
    RegisterName::SP
}
pub fn bp_name() -> RegisterName {
    RegisterName::BP
}
pub fn rx_name(x: u8) -> RegisterName {
    match x {
        8 => RegisterName::R8,
        9 => RegisterName::R9,
        10 => RegisterName::R10,
        11 => RegisterName::R11,
        12 => RegisterName::R12,
        13 => RegisterName::R13,
        14 => RegisterName::R14,
        15 => RegisterName::R15,
        _ => panic!("{x} is not the name of a x64 register"),
    }
}

pub fn rax() -> Register {
    a_name().quad()
}
pub fn eax() -> Register {
    a_name().double()
}
pub fn ax() -> Register {
    a_name().word()
}
pub fn al() -> Register {
    a_name().byte()
}

pub fn rbx() -> Register {
    b_name().quad()
}
pub fn ebx() -> Register {
    b_name().double()
}
pub fn bx() -> Register {
    b_name().word()
}
pub fn bl() -> Register {
    b_name().byte()
}

pub fn rcx() -> Register {
    c_name().quad()
}
pub fn ecx() -> Register {
    c_name().double()
}
pub fn cx() -> Register {
    c_name().word()
}
pub fn cl() -> Register {
    c_name().byte()
}

pub fn rdx() -> Register {
    d_name().quad()
}
pub fn edx() -> Register {
    d_name().double()
}
pub fn dx() -> Register {
    d_name().word()
}
pub fn dl() -> Register {
    d_name().byte()
}

pub fn rdi() -> Register {
    di_name().quad()
}
pub fn edi() -> Register {
    di_name().double()
}
pub fn di() -> Register {
    di_name().word()
}
pub fn dil() -> Register {
    di_name().byte()
}

pub fn rsi() -> Register {
    si_name().quad()
}
pub fn esi() -> Register {
    si_name().double()
}
pub fn si() -> Register {
    si_name().word()
}
pub fn sil() -> Register {
    si_name().byte()
}

pub fn rsp() -> Register {
    sp_name().quad()
}
pub fn esp() -> Register {
    sp_name().double()
}
pub fn sp() -> Register {
    sp_name().word()
}
pub fn spl() -> Register {
    sp_name().byte()
}

pub fn rbp() -> Register {
    bp_name().quad()
}
pub fn ebp() -> Register {
    bp_name().double()
}
pub fn bp() -> Register {
    bp_name().word()
}
pub fn bpl() -> Register {
    bp_name().byte()
}

pub fn rx(x: u8) -> Register {
    rx_name(x).quad()
}
pub fn rxd(x: u8) -> Register {
    rx_name(x).double()
}
pub fn rxw(x: u8) -> Register {
    rx_name(x).word()
}
pub fn rxb(x: u8) -> Register {
    rx_name(x).byte()
}
