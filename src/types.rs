use std::{fmt::{Display}, error::Error, ops::Index};
use crate::util::dedup_list::DedupList;
use self::{array_type::{ArrTypeID, ArrayType}};

pub mod array_type;


pub struct Types {
    arrays: DedupList<ArrayType>,
}
impl Types {
    pub fn new() -> Self {
        Self {
            arrays: DedupList::new(),
        }
    }

    pub fn make_array(&mut self, member: impl Into<Type>, length: impl Into<u64>) -> ArrTypeID {
        let id = self.arrays.insert(|id| ArrayType::new(id, member.into(), length.into()));
        ArrTypeID(id)
    }
}
impl Index<ArrTypeID> for Types {
    type Output = ArrayType;
    fn index(&self, index: ArrTypeID) -> &Self::Output {
        &self.arrays[index.0]
    }
}


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Type {
    Unit,
    Byte(ByteSize),
    Integer(IntegerSize),
    Pointer,
    Array(ArrTypeID),
}
impl Type {
    pub fn integer<T, E>(size: T) -> Self
        where T: TryInto<IntegerSize, Error = E>,
              E: Error
    {
        Self::Integer(size.try_into().unwrap())
    }
    pub fn byte(size: impl Into<ByteSize>) -> Self {
        Self::Byte(size.into())
    }

    pub fn is_integer(&self) -> bool {
        matches!(self, Self::Integer(_))
    }
    pub fn is_pointer(&self) -> bool {
        matches!(self, Self::Pointer)
    }
}
impl From<ByteSize> for Type {
    fn from(value: ByteSize) -> Self {
        Self::Byte(value)
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


/// The size of a 'byte' type.
/// Only powers of two are valid.
/// A byte type may be between 1 and 256 bytes
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ByteSize(
    /// The logarithm of the amount of bytes in the byte type.
    /// In other words, the amount of bytes is obtained by taking two to the power of this number.
    u8
);
impl ByteSize {
    pub fn from_bits(bits: usize) -> Self {
        assert!(bits.is_power_of_two());
        assert!(bits % 8 == 0);
        let bytes = bits / 8;
        Self::from_bytes(bytes)
    }
    pub fn from_bytes(bytes: usize) -> Self {
        assert!(bytes > 1 && bytes <= 256);
        let log = bytes.ilog2();
        Self(log as u8)   
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
    u8
);
impl IntegerSize {
    pub fn bits(self) -> usize {
        self.0 as usize + 1
    }
}
impl IntegerSize {
    pub fn make<I, E>(bits: I) -> Self 
        where I: TryInto<IntegerSize, Error = E>,
              E: Error
    {
        bits.try_into().unwrap()
    }
}
impl TryFrom<u8> for IntegerSize {
    type Error = IntSizeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > 0 {
            Ok(Self(value - 1))
        }
        else {
            Err(IntSizeError)
        }
    }
}
impl TryFrom<u16> for IntegerSize {
    type Error = IntSizeError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value > 0 && value <= 256 {
            Ok(Self((value - 1) as u8))
        }
        else {
            Err(IntSizeError)
        }
    }
}
impl TryFrom<u32> for IntegerSize {
    type Error = IntSizeError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value > 0 && value <= 256 {
            Ok(Self((value - 1) as u8))
        }
        else {
            Err(IntSizeError)
        }
    }
}
impl TryFrom<u64> for IntegerSize {
    type Error = IntSizeError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value > 0 && value <= 256 {
            Ok(Self((value - 1) as u8))
        }
        else {
            Err(IntSizeError)
        }
    }
}
impl TryFrom<usize> for IntegerSize {
    type Error = IntSizeError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value > 0 && value <= 256 {
            Ok(Self((value - 1) as u8))
        }
        else {
            Err(IntSizeError)
        }
    }
}
impl TryFrom<i8> for IntegerSize {
    type Error = IntSizeError;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        if value > 0 {
            Ok(Self((value - 1) as u8))
        }
        else {
            Err(IntSizeError)
        }
    }
}
impl TryFrom<i16> for IntegerSize {
    type Error = IntSizeError;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        if value > 0 && value <= 256 {
            Ok(Self((value - 1) as u8))
        }
        else {
            Err(IntSizeError)
        }
    }
}
impl TryFrom<i32> for IntegerSize {
    type Error = IntSizeError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value > 0 && value <= 256 {
            Ok(Self((value - 1) as u8))
        }
        else {
            Err(IntSizeError)
        }
    }
}
impl TryFrom<i64> for IntegerSize {
    type Error = IntSizeError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if value > 0 && value <= 256 {
            Ok(Self((value - 1) as u8))
        }
        else {
            Err(IntSizeError)
        }
    }
}
impl TryFrom<isize> for IntegerSize {
    type Error = IntSizeError;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        if value > 0 && value <= 256 {
            Ok(Self((value - 1) as u8))
        }
        else {
            Err(IntSizeError)
        }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct IntSizeError;
impl Display for IntSizeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Size of an integer type must be between 2 and 255 inclusive")
    }
}
impl Error for IntSizeError {

}
