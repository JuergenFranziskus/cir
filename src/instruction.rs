use crate::{FunTyID, block::BlockID, function::FunID, register::RegID, variable::VarID};


#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Instruction {
    SetInt(RegID, i64),
    SetBool(RegID, bool),
    SetStruct(RegID, Vec<RegID>),
    SetArray(RegID, Vec<RegID>),

    Add(RegID, RegID, RegID),
    Sub(RegID, RegID, RegID),
    Mul(RegID, RegID, RegID),
    UDiv(RegID, RegID, RegID),
    IDiv(RegID, RegID, RegID),
    UMod(RegID, RegID, RegID),
    IMod(RegID, RegID, RegID),
    And(RegID, RegID, RegID),
    Or(RegID, RegID, RegID),
    Xor(RegID, RegID, RegID),
    ShiftLeft(RegID, RegID, RegID),
    ShiftRightLogical(RegID, RegID, RegID),
    ShiftRightArithmetic(RegID, RegID, RegID),
    Neg(RegID, RegID),
    Not(RegID, RegID),
    IntToPtr(RegID, RegID),
    PtrToInt(RegID, RegID),
    Sext(RegID, RegID),
    Zext(RegID, RegID),
    Truncate(RegID, RegID),

    Poison(RegID),
    Select(RegID, RegID, RegID, RegID),
    Freeze(RegID, RegID),

    GetVarAddr(RegID, VarID),
    Store {
        ptr: RegID,
        value: RegID,
    },
    Load {
        ptr: RegID,
        dst: RegID,
    },

    Jump(JumpTarget),
    Branch(RegID, JumpTarget, JumpTarget),
    Ret(Option<RegID>),
    Call(Option<RegID>, FunID, Vec<RegID>),
    CallPtr(Option<RegID>, RegID, FunTyID, Vec<RegID>),

    GetStructMember {
        dst: RegID,
        strct: RegID,
        index: u64,
    },
    SetStructMember {
        dst: RegID,
        strct: RegID,
        value: RegID,
        index: u64,
    },
    GetArrayElement {
        dst: RegID,
        array: RegID,
        index: RegID,
    },
    SetArrayElement {
        dst: RegID,
        array: RegID,
        value: RegID,
        index: RegID,
    },
    IndexStruct {
        dst: RegID,
        ptr: RegID,
        index: u64,
    },
    IndexArray {
        dst: RegID,
        ptr: RegID,
        index: RegID,
    }
}


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JumpTarget {
    pub block: BlockID,
    pub args: Vec<RegID>,
}
impl From<BlockID> for JumpTarget {
    fn from(value: BlockID) -> Self {
        Self { block: value, args: Vec::new() }
    }
}
impl From<(BlockID, Vec<RegID>)> for JumpTarget {
    fn from(value: (BlockID, Vec<RegID>)) -> Self {
        Self {
            block: value.0,
            args: value.1,
        }
    }
}
impl<const N: usize> From<(BlockID, [RegID; N])> for JumpTarget {
    fn from(value: (BlockID, [RegID; N])) -> Self {
        Self {
            block: value.0,
            args: value.1.into(),
        }
    }
}
impl From<(BlockID, &[RegID])> for JumpTarget {
    fn from(value: (BlockID, &[RegID])) -> Self {
        Self {
            block: value.0,
            args: value.1.into(),
        }
    }
}
