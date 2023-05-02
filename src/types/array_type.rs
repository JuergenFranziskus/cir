use std::{collections::hash_map::DefaultHasher, hash::{Hash, Hasher}};

use crate::util::dedup_list::DedupItem;

use super::Type;





pub struct ArrayType {
    id: ArrTypeID,
    member: Type,
    length: u64,
}
impl ArrayType {
    pub(super) fn new(id: usize, member: Type, length: u64) -> Self {
        Self {
            id: ArrTypeID(id),
            member,
            length,
        }
    }

    pub fn id(&self) -> ArrTypeID {
        self.id
    }
    pub fn member(&self) -> Type {
        self.member
    }
    pub fn length(&self) -> u64 {
        self.length
    }
}
impl DedupItem for ArrayType {
    fn hash_sans_id(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.member.hash(&mut hasher);
        self.length.hash(&mut hasher);
        hasher.finish()
    }
    fn is_equivalent_to(&self, other: &Self) -> bool {
        self.member == other.member
            && self.length == other.length
    }
}



#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ArrTypeID(pub usize);
