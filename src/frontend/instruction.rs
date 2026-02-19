use super::{
    FunTyID, IntTy, StructTyID, Ty, block::BlockID, function::FunID, register::RegID,
    variable::VarID,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Instruction {
    Set(RegID, Value),
    SetFunPtr(RegID, FunID),
    SetStruct(RegID, Values),
    SetArray(RegID, Values),
    SetArraySplat(RegID, Value),

    Binary(BinOp, RegID, Value, Value),
    Unary(UnOp, RegID, Value),

    Poison(RegID),
    Select(RegID, Value, Value, Value),
    Freeze(RegID, Value),

    GetVarAddr(RegID, VarID),
    Store {
        ptr: RegID,
        value: Value,
    },
    Load {
        dst: RegID,
        ptr: RegID,
    },

    Jump(JumpTarget),
    Branch(Value, JumpTarget, JumpTarget),
    Ret(Value),
    Call(RegID, FunID, Values),
    CallPtr(RegID, RegID, FunTyID, Values),

    GetStructMember {
        dst: RegID,
        strct: RegID,
        index: u64,
    },
    SetStructMember {
        dst: RegID,
        strct: RegID,
        value: Value,
        index: u64,
    },
    GetArrayElement {
        dst: RegID,
        array: RegID,
        index: Value,
    },
    SetArrayElement {
        dst: RegID,
        array: RegID,
        value: Value,
        index: Value,
    },
    IndexStruct {
        dst: RegID,
        ptr: RegID,
        struct_ty: StructTyID,
        index: u64,
    },
    IndexArray {
        dst: RegID,
        ptr: RegID,
        element_ty: Ty,
        index: Value,
    },

    SyscallX86_64 {
        dst: RegID,
        call_number: Value,
        args: Values,
    },
}
impl Instruction {
    pub fn next_blocks(&self) -> Option<(BlockID, Option<BlockID>)> {
        match self {
            Self::Jump(tgt) => Some((tgt.block, None)),
            Self::Branch(_, t, f) => Some((t.block, Some(f.block))),
            _ => None,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    IDiv,
    UDiv,
    IMod,
    UMod,
    And,
    Or,
    Xor,
    Shl,
    Shr,
    Sar,
    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Above,
    AboveEqual,
    Below,
    BelowEqual,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum UnOp {
    Neg,
    Not,
    IntToPtr,
    PtrToInt,
    Sext,
    Zext,
    Trunc,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Values(pub Vec<Value>);
impl Values {
    pub fn new() -> Self {
        Self(Vec::new())
    }
}
impl From<Vec<Value>> for Values {
    fn from(value: Vec<Value>) -> Self {
        Self(value)
    }
}
impl<const N: usize> From<[Value; N]> for Values {
    fn from(value: [Value; N]) -> Self {
        Self(value.into())
    }
}
impl From<&[Value]> for Values {
    fn from(value: &[Value]) -> Self {
        Self(value.into())
    }
}
impl From<Vec<RegID>> for Values {
    fn from(value: Vec<RegID>) -> Self {
        let value: Vec<Value> = value.into_iter().map(|v| v.into()).collect();
        Self::from(value)
    }
}
impl<const N: usize> From<[RegID; N]> for Values {
    fn from(value: [RegID; N]) -> Self {
        let value: Vec<Value> = value.into_iter().map(|v| v.into()).collect();
        Self::from(value)
    }
}
impl From<&[RegID]> for Values {
    fn from(value: &[RegID]) -> Self {
        let value: Vec<Value> = value.into_iter().map(|&v| v.into()).collect();
        Self::from(value)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Value {
    Void,
    Bool(bool),
    Int(IntTy, i64),
    Reg(RegID),
}
impl Value {
    pub fn reg(self) -> RegID {
        match self {
            Self::Reg(reg) => reg,
            _ => panic!(),
        }
    }
}
impl From<RegID> for Value {
    fn from(value: RegID) -> Self {
        Self::Reg(value)
    }
}
impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Self::Int(IntTy::I64, value)
    }
}
impl From<u64> for Value {
    fn from(value: u64) -> Self {
        Self::Int(IntTy::I64, value as i64)
    }
}
impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Self::Int(IntTy::I32, value as i64)
    }
}
impl From<u32> for Value {
    fn from(value: u32) -> Self {
        Self::Int(IntTy::I32, value as i64)
    }
}
impl From<i16> for Value {
    fn from(value: i16) -> Self {
        Self::Int(IntTy::I16, value as i64)
    }
}
impl From<u16> for Value {
    fn from(value: u16) -> Self {
        Self::Int(IntTy::I16, value as i64)
    }
}
impl From<i8> for Value {
    fn from(value: i8) -> Self {
        Self::Int(IntTy::I8, value as i64)
    }
}
impl From<u8> for Value {
    fn from(value: u8) -> Self {
        Self::Int(IntTy::I8, value as i64)
    }
}
impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}
impl From<()> for Value {
    fn from(value: ()) -> Self {
        let _ = value;
        Self::Void
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JumpTarget {
    pub block: BlockID,
    pub args: Values,
}
impl From<BlockID> for JumpTarget {
    fn from(value: BlockID) -> Self {
        Self {
            block: value,
            args: Values::new(),
        }
    }
}
impl<T: Into<Values>> From<(BlockID, T)> for JumpTarget {
    fn from(value: (BlockID, T)) -> Self {
        Self {
            block: value.0,
            args: value.1.into(),
        }
    }
}
