use super::{block::BlockID, function::FuncID, register::RegID, variable::VarID};
use crate::{
    function::calling_convention::CallingConvention,
    struct_type::StructTypeID,
    types::{IntegerSize, Type},
    Module, Types,
};
use std::fmt::Display;

#[derive(Clone, Debug)]
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
        struct_type: StructTypeID,
    },

    GetVarPointer(RegID, VarID),
    GetFunctionPointer(RegID, FuncID),
    Load {
        target: RegID,
        pointer: RegID,
        volatile: bool,
    },
    Store {
        pointer: RegID,
        value: Expr,
        volatile: bool,
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
        function_ptr: RegID,
        convention: CallingConvention,
        potentially_vararg: bool,
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
impl Display for BinaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use BinaryOp::*;
        match self {
            Add => write!(f, "add"),
            Sub => write!(f, "sub"),
            Mul => write!(f, "mul"),
            UDiv => write!(f, "udiv"),
            IDiv => write!(f, "idiv"),
            ShiftLeft => write!(f, "shl"),
            ShiftLogicalRight => write!(f, "shr"),
            ShiftArithmeticRight => write!(f, "sar"),
            And => write!(f, "and"),
            Nand => write!(f, "nand"),
            Or => write!(f, "or"),
            Nor => write!(f, "nor"),
            Xor => write!(f, "xor"),
            XNor => write!(f, "xnor"),
        }
    }
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
impl Display for UnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use UnaryOp::*;
        match self {
            Not => write!(f, "not"),
            Neg => write!(f, "neg"),
            Freeze => write!(f, "freeze"),
            Truncate => write!(f, "trunc"),
            SignExtend => write!(f, "sext"),
            ZeroExtend => write!(f, "zext"),
            IntToPtr => write!(f, "inttoptr"),
            PtrToInt => write!(f, "ptrtoint"),
        }
    }
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
impl Display for TestOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use TestOp::*;
        match self {
            Equal => write!(f, "test.eq"),
            NotEqual => write!(f, "test.neq"),
            Greater => write!(f, "test.sgt"),
            GreaterEqual => write!(f, "test.sge"),
            Less => write!(f, "test.slt"),
            LessEqual => write!(f, "test.sle"),
            Above => write!(f, "test.ugt"),
            AboveEqual => write!(f, "test.uge"),
            Below => write!(f, "test.ult"),
            BelowEqual => write!(f, "test.ule"),
        }
    }
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub enum Expr {
    Register(RegID),
    Struct(Vec<Expr>),
    ShortArray(Box<Expr>, u64),
    Array(Vec<Expr>),
    Constant(ConstValue),
}
impl Expr {
    pub fn expr_type(&self, module: &Module, types: &mut Types) -> Type {
        match self {
            &Expr::Register(id) => module[id].reg_type(),
            Expr::Struct(members) => {
                let members = members.iter().map(|m| m.expr_type(module, types)).collect();
                types.make_struct(members).into()
            }
            &Expr::ShortArray(ref element, length) => {
                let element_type = element.expr_type(module, types);
                types.make_array(element_type, length).into()
            }
            Expr::Array(elements) => {
                let element_type = elements[0].expr_type(module, types);
                let length = elements.len() as u64;
                types.make_array(element_type, length).into()
            }
            Expr::Constant(v) => v.expr_type(),
        }
    }
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

#[derive(Clone, Debug)]
pub enum ConstValue {
    Poison(Type),
    Unit,
    NullPtr,
    Integer(i128, IntegerSize),
    SizeOf(Type, IntegerSize),
}
impl ConstValue {
    pub fn expr_type(&self) -> Type {
        match self {
            &Self::Poison(t) => t,
            Self::Unit => Type::Unit,
            Self::NullPtr => Type::Pointer,
            &Self::Integer(_, size) => size.into(),
            &Self::SizeOf(_, s) => s.into(),
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
