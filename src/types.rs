use self::{
    array_type::{ArrTypeID, ArrayType},
    struct_type::{StructType, StructTypeID},
};
use crate::util::dedup_list::DedupList;
use std::{ops::Index};

pub mod array_type;
pub mod struct_type;

pub struct Types {
    arrays: DedupList<ArrayType>,
    structs: DedupList<StructType>,
}
impl Types {
    pub fn new() -> Self {
        Self {
            arrays: DedupList::new(),
            structs: DedupList::new(),
        }
    }

    pub fn make_array(&mut self, member: impl Into<Type>, length: impl Into<u64>) -> ArrTypeID {
        let id = self
            .arrays
            .insert(|id| ArrayType::new(id, member.into(), length.into()));
        ArrTypeID(id)
    }
    pub fn make_struct(&mut self, members: Vec<Type>) -> StructTypeID {
        let i = self
            .structs
            .insert(|i| StructType::new(StructTypeID(i), members));
        StructTypeID(i)
    }
}
impl Index<ArrTypeID> for Types {
    type Output = ArrayType;
    fn index(&self, index: ArrTypeID) -> &Self::Output {
        &self.arrays[index.0]
    }
}
impl Index<StructTypeID> for Types {
    type Output = StructType;
    fn index(&self, index: StructTypeID) -> &Self::Output {
        &self.structs[index.0]
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Type {
    Unit,
    Bool,
    Integer(IntSize),
    Pointer,
    Array(ArrTypeID),
    Struct(StructTypeID),
}
impl Type {
    pub fn is_integer(&self) -> bool {
        matches!(self, Self::Integer(_))
    }
    pub fn is_pointer(&self) -> bool {
        matches!(self, Self::Pointer)
    }
}
impl From<IntSize> for Type {
    fn from(value: IntSize) -> Self {
        Self::Integer(value)
    }
}
impl From<ArrTypeID> for Type {
    fn from(value: ArrTypeID) -> Self {
        Self::Array(value)
    }
}
impl From<StructTypeID> for Type {
    fn from(value: StructTypeID) -> Self {
        Self::Struct(value)
    }
}



#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IntSize {
    Byte,
    Short,
    Int,
    Long,
}
impl IntSize {
    pub fn bits(self) -> u8 {
        self.bytes() * 8
    }
    pub fn bytes(self) -> u8 {
        match self {
            Self::Byte => 1,
            Self::Short => 2,
            Self::Int => 4,
            Self::Long => 8,
        }
    }
}
