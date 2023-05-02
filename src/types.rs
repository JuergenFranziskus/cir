use self::{
    array_type::{ArrTypeID, ArrayType},
    struct_type::{StructType, StructTypeID},
};
use crate::util::dedup_list::DedupList;
use std::{error::Error, fmt::Display, ops::Index};

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
    Integer(IntegerSize),
    Pointer,
    Array(ArrTypeID),
    Struct(StructTypeID),
}
impl Type {
    pub fn integer<T, E>(size: T) -> Self
    where
        T: TryInto<IntegerSize, Error = E>,
        E: Error,
    {
        Self::Integer(size.try_into().unwrap())
    }

    pub fn is_integer(&self) -> bool {
        matches!(self, Self::Integer(_))
    }
    pub fn is_pointer(&self) -> bool {
        matches!(self, Self::Pointer)
    }
}
impl From<IntegerSize> for Type {
    fn from(value: IntegerSize) -> Self {
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

/// The size of an integer size.
/// Integers may be between one and 256 bits.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IntegerSize(
    /// The amount of bits in the integer type.
    /// Stored with an offset of 1.
    /// So, a value of 0 is actually one bit,
    /// a value of 1 is two, etc.
    u8,
);
impl IntegerSize {
    pub fn bits(self) -> usize {
        self.0 as usize + 1
    }
}
impl IntegerSize {
    pub fn make<I, E>(bits: I) -> Self
    where
        I: TryInto<IntegerSize, Error = E>,
        E: Error,
    {
        bits.try_into().unwrap()
    }
}
impl TryFrom<u8> for IntegerSize {
    type Error = IntSizeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > 0 {
            Ok(Self(value - 1))
        } else {
            Err(IntSizeError)
        }
    }
}
impl TryFrom<u16> for IntegerSize {
    type Error = IntSizeError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value > 0 && value <= 256 {
            Ok(Self((value - 1) as u8))
        } else {
            Err(IntSizeError)
        }
    }
}
impl TryFrom<u32> for IntegerSize {
    type Error = IntSizeError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value > 0 && value <= 256 {
            Ok(Self((value - 1) as u8))
        } else {
            Err(IntSizeError)
        }
    }
}
impl TryFrom<u64> for IntegerSize {
    type Error = IntSizeError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value > 0 && value <= 256 {
            Ok(Self((value - 1) as u8))
        } else {
            Err(IntSizeError)
        }
    }
}
impl TryFrom<usize> for IntegerSize {
    type Error = IntSizeError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value > 0 && value <= 256 {
            Ok(Self((value - 1) as u8))
        } else {
            Err(IntSizeError)
        }
    }
}
impl TryFrom<i8> for IntegerSize {
    type Error = IntSizeError;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        if value > 0 {
            Ok(Self((value - 1) as u8))
        } else {
            Err(IntSizeError)
        }
    }
}
impl TryFrom<i16> for IntegerSize {
    type Error = IntSizeError;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        if value > 0 && value <= 256 {
            Ok(Self((value - 1) as u8))
        } else {
            Err(IntSizeError)
        }
    }
}
impl TryFrom<i32> for IntegerSize {
    type Error = IntSizeError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value > 0 && value <= 256 {
            Ok(Self((value - 1) as u8))
        } else {
            Err(IntSizeError)
        }
    }
}
impl TryFrom<i64> for IntegerSize {
    type Error = IntSizeError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if value > 0 && value <= 256 {
            Ok(Self((value - 1) as u8))
        } else {
            Err(IntSizeError)
        }
    }
}
impl TryFrom<isize> for IntegerSize {
    type Error = IntSizeError;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        if value > 0 && value <= 256 {
            Ok(Self((value - 1) as u8))
        } else {
            Err(IntSizeError)
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct IntSizeError;
impl Display for IntSizeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Size of an integer type must be between 2 and 255 inclusive"
        )
    }
}
impl Error for IntSizeError {}
