use super::{
    block::BlockID, calling_convention::CallingConvention, function::FuncID, register::RegID,
    variable::VarID,
};
use crate::types::{IntegerSize, Type};

pub enum Instruction {
    Nop,
    Set(RegID, Expr),
    BinaryOp(RegID, BinaryOp, Expr, Expr),
    UnaryOp(RegID, UnaryOp, Expr),
    TestOp(RegID, TestOp, Expr, Expr),
    Select {
        target: RegID,
        condition: Expr,
        true_value: Expr,
        false_value: Expr,
    },
    GetElement {
        target: RegID,
        array: Expr,
        index: Expr,
    },
    GetElementPtr {
        target: RegID,
        array_pointer: RegID,
        index: Expr,
        element_type: Type,
    },
    GetMember {
        target: RegID,
        structure: Expr,
        index: u32,
    },
    GetMemberPtr {
        target: RegID,
        struct_pointer: RegID,
        member: u32,
    },

    GetVarPointer(RegID, VarID),
    GetFunctionPointer(RegID, FuncID),
    Load {
        target: RegID,
        pointer: RegID,
    },
    Store {
        pointer: RegID,
        value: Expr,
    },

    Jump(BlockTarget),
    Branch(Expr, BlockTarget, BlockTarget),
    Call {
        target: RegID,
        function: FuncID,
        parameters: Vec<Expr>,
    },
    CallPtr {
        target: RegID,
        calling_convention: CallingConvention,
        function_ptr: RegID,
        parameters: Vec<Expr>,
    },
    Return(Expr),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    UDiv,
    IDiv,

    ShiftLeft,
    ShiftLogicalRight,
    ShiftArithmeticRight,

    And,
    Nand,
    Or,
    Nor,
    Xor,
    XNor,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum UnaryOp {
    Not,
    Neg,
    Freeze,
    Truncate,
    SignExtend,
    ZeroExtend,
    IntToPtr,
    PtrToInt,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum TestOp {
    Equal,
    NotEqual,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
    Above,
    Below,
    AboveEqual,
    BelowEqual,
}

pub struct BlockTarget {
    pub block: BlockID,
    pub parameters: Vec<Expr>,
}
impl From<BlockID> for BlockTarget {
    fn from(value: BlockID) -> Self {
        Self {
            block: value,
            parameters: Vec::new(),
        }
    }
}
impl<T: Into<Expr>> From<(BlockID, T)> for BlockTarget {
    fn from(value: (BlockID, T)) -> Self {
        Self {
            block: value.0,
            parameters: vec![value.1.into()],
        }
    }
}

pub enum Expr {
    Register(RegID),
    Struct(Vec<Expr>),
    ShortArray(Box<Expr>, u64),
    Array(Vec<Expr>),
    Constant(ConstValue),
}
impl From<RegID> for Expr {
    fn from(value: RegID) -> Self {
        Self::Register(value)
    }
}
impl From<()> for Expr {
    fn from(value: ()) -> Self {
        Self::Constant(value.into())
    }
}
impl From<u8> for Expr {
    fn from(value: u8) -> Self {
        Self::Constant(value.into())
    }
}
impl From<i8> for Expr {
    fn from(value: i8) -> Self {
        Self::Constant(value.into())
    }
}
impl From<u16> for Expr {
    fn from(value: u16) -> Self {
        Self::Constant(value.into())
    }
}
impl From<i16> for Expr {
    fn from(value: i16) -> Self {
        Self::Constant(value.into())
    }
}
impl From<u32> for Expr {
    fn from(value: u32) -> Self {
        Self::Constant(value.into())
    }
}
impl From<i32> for Expr {
    fn from(value: i32) -> Self {
        Self::Constant(value.into())
    }
}
impl From<u64> for Expr {
    fn from(value: u64) -> Self {
        Self::Constant(value.into())
    }
}
impl From<i64> for Expr {
    fn from(value: i64) -> Self {
        Self::Constant(value.into())
    }
}

pub enum ConstValue {
    Poison(Type),
    Unit,
    NullPtr,
    Integer(i128, IntegerSize),
}
impl ConstValue {
    pub fn expr_type(&self) -> Type {
        match self {
            &Self::Poison(t) => t,
            &Self::Unit => Type::Unit,
            &Self::NullPtr => Type::Pointer,
            &Self::Integer(_, size) => size.into(),
        }
    }
}
impl From<()> for ConstValue {
    fn from(_value: ()) -> Self {
        Self::Unit
    }
}
impl From<u8> for ConstValue {
    fn from(value: u8) -> Self {
        Self::Integer(value as i128, IntegerSize::make(8))
    }
}
impl From<i8> for ConstValue {
    fn from(value: i8) -> Self {
        Self::Integer(value as i128, IntegerSize::make(8))
    }
}
impl From<u16> for ConstValue {
    fn from(value: u16) -> Self {
        Self::Integer(value as i128, IntegerSize::make(16))
    }
}
impl From<i16> for ConstValue {
    fn from(value: i16) -> Self {
        Self::Integer(value as i128, IntegerSize::make(16))
    }
}
impl From<u32> for ConstValue {
    fn from(value: u32) -> Self {
        Self::Integer(value as i128, IntegerSize::make(32))
    }
}
impl From<i32> for ConstValue {
    fn from(value: i32) -> Self {
        Self::Integer(value as i128, IntegerSize::make(32))
    }
}
impl From<u64> for ConstValue {
    fn from(value: u64) -> Self {
        Self::Integer(value as i128, IntegerSize::make(64))
    }
}
impl From<i64> for ConstValue {
    fn from(value: i64) -> Self {
        Self::Integer(value as i128, IntegerSize::make(64))
    }
}
